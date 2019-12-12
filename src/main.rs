use std::path::{Path};
use std::ffi::{CString};
use std::rc::Rc;

extern crate sdl2;
extern crate sdl2_sys;
extern crate gl;

pub mod resources;
pub mod render;
pub mod ffi_utils;
pub mod obj;

use crate::obj::vertex::{Vertex, VertexP, VertexPT, Coords3D};
use crate::obj::shape::{Shape};

// The SDL needs u32, but gl viewport needs i32
// const SCREEN_WIDTH:u32 = 800;
// const SCREEN_HEIGHT:u32 = 450;

// There must be a better macro definition method than making it a function call....
#[macro_export]
macro_rules! SCREEN_HEIGHT {
    () => {
        450
    };
}
#[macro_export]
macro_rules! SCREEN_WIDTH {
    () => {
        800
    };
}

fn main() {
    // Creating SDL instance
    let sdl = sdl2::init().unwrap();
    
    // Create a video subsystem
    let video_subsystem = sdl.video().unwrap();

    // Specify a OpenGL version and use "core" profile 
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Rusty-craft", SCREEN_WIDTH!(), SCREEN_HEIGHT!())
        .opengl()   // We're using OpenGL
        .resizable()
        // .fullscreen() // Maybe later
        .build()
        .unwrap();

    // OpenGL context
    let _gl_context = window.gl_create_context().unwrap();  // TODO: Use this variable?

    // Load the video subsystem
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void); // TODO: Use this variable?
    
    // Designate the "clear" colour and viewport
    unsafe {
        gl::Viewport(0, 0, SCREEN_WIDTH!(), SCREEN_HEIGHT!()); // set viewport
        gl::ClearColor(0.5, 0.5, 0.7, 1.0); // Set the 'clear' colour to light blue
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    // Get a reference the event stream
    let mut event_pump = sdl.event_pump().unwrap();

    // -- Load the shaders -- //
    // TODO: Load in another file, then import?
    let resources = resources::Resources::from_relative_exe_path(Path::new("assets")).unwrap();
    let vert_shader = render::shader::Shader::from_resource(&resources, "shaders/triangle.vert").unwrap();
    let frag_shader = render::shader::Shader::from_resource(&resources, "shaders/triangle.frag").unwrap();
    // -- -- //

    // -- Load Texture -- //
    // TODO: Maybe texture shouldn't automatically bind itself?
    let texture = render::texture::Texture::from_resource(&resources, "textures/test_16.png").unwrap();

    // declare main sharder program
    let shader_program = render::program::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();
    
    // Set the program as the main shader program
    shader_program.set();

    // New shape declaration: 
    let mut square = Shape::from_vertices_and_triangle(
        vec![
            VertexPT {
                position: (-0.5, -0.5, 0.0).into(),
                texture_coords : (0.0,0.0).into(),
            },
            VertexPT {
                position: (0.5, -0.5, 0.0).into(),
                texture_coords : (1.0,0.0).into(),
            },
            VertexPT {
                position: (0.5, 0.5, 0.0).into(),
                texture_coords : (1.0,1.0).into(),
            },
            VertexPT {
                position: (-0.5, 0.5, 0.0).into(),
                texture_coords : (0.0,1.0).into(),
            },
        ], 
        vec![
            (0, 1, 2),    // First triangle
            (2, 3, 0),    // Second triangle
        ]
    );

    // Setup it's VAO and such
    square.setup();

    // Create global square reference:
    let global_square = Rc::new(square);

    // Get a world version of the square:
    let mut in_world_square = obj::shape::Object::new(&global_square);

    // Loop state variables
    let mut last_tick = unsafe{sdl2_sys::SDL_GetTicks()};
    let mut u_colour_angle : u32 = 0;
    // Main loop
    'main: loop {
        // Calculate current FPS
        let cur_tick = unsafe{sdl2_sys::SDL_GetTicks()};
        let tick_diff = cur_tick - last_tick;
        let fps : f32 = 1000.0 / ( tick_diff as f32);
        // println!("fsp {}", fps);    // TODO: Write to corner of screen?
        last_tick = cur_tick;
        
        // Modify a `uniform` variable to affect triangle colour
        const LOOP_TIME :u32 = 10000;
        u_colour_angle = (u_colour_angle + tick_diff) % LOOP_TIME;
        // Normalise around 10000 -> 2*pi.
        let u_colour_angle_rad = ((u_colour_angle as f32) * 2.0 * std::f32::consts::PI)/ (LOOP_TIME as f32);
        unsafe{
            let radius_uniform_location = gl::GetUniformLocation(shader_program.id(), CString::new("timed_colour").unwrap().as_ptr() );
            gl::Uniform1f(
                radius_uniform_location, //GLint, 
                u_colour_angle_rad as gl::types::GLfloat
            );
        }


        // Handle events:
        for event in event_pump.poll_iter() {
            use sdl2::event::Event; // Shorten the current namespace
            match event {

                Event::KeyDown {..} => {
                    println!("{:?}", event);
                }
                Event::KeyUp {..} => {
                    println!("{:?}", event);
                }
                
                // Exit Game:
                Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        // TODO: render something
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            in_world_square.rotation.x = u_colour_angle_rad;
            // in_world_square.rotation.y = u_colour_angle_rad + 1.6;

            // Draw our shape:
            in_world_square.draw(shader_program.id());
        }
        
        window.gl_swap_window();
    }
}
