use std::path::{Path};
use std::ffi::{CString};

extern crate sdl2;
extern crate sdl2_sys;
extern crate gl;

pub mod resources;
pub mod render;
pub mod ffi_utils;
pub mod obj;

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

    // -- Declaring a shape to render -- //
    const VERT_LENGTH: usize = 8;
    let vertices: Vec<f32> = vec![
        // Declared anti-clockwise from bottom right
        // Pos                 Colour           text position
        -0.5, -0.5, 0.0,    1.0, 0.0, 0.0,      0.0, 0.0, // A triangle with RGB information
         0.5, -0.5, 0.0,    0.0, 1.0, 0.0,      1.0, 0.0,
         0.5,  0.5, 0.0,    0.0, 0.0, 1.0,      1.0, 1.0,
        -0.5,  0.5, 0.0,    0.0, 0.0, 1.0,      0.0, 1.0
    ];

    // Get a Vertex Buffer Object (VBO)
    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);    // Create 1 buffer.
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);  // Bind as an ARRAY_BUFFER
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage hint: Data rarely changes, used for drawing
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }

    // Create an Element Buffer Object (to re use vertex information for a square)
    let indexes: Vec<u32> = vec![
        0, 1, 2,    // First triangle
        2, 3, 0,    // Second triangle
    ];

    let mut ebo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut ebo);    // Create 1 buffer.
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);  // Bind as an ELEMENT_ARRAY_BUFFER
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER, // target
            (indexes.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr, // size of data in bytes
            indexes.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage hint: Data rarely changes, used for drawing
        );
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); // unbind the buffer
    }


    // Create a Vertex Array Object (VAO)
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);  // re-bind the vbo into the context
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);  // re-bind the ebo into the context

        // -- Set Attributes -- //
        let stride = (VERT_LENGTH * std::mem::size_of::<f32>()) as gl::types::GLint;
        // Position info:
        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // disable normalization (int-to-float conversion)
            stride, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );
        // Base colour info:
        gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
        gl::VertexAttribPointer(
            1, // index of the generic vertex attribute ("layout (location = 1)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // disable normalization (int-to-float conversion)
            stride, // stride (byte offset between consecutive attributes)
            (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
        );
        // Texture info:
        gl::EnableVertexAttribArray(2); // this is "layout (location = 2)" in vertex shader
        gl::VertexAttribPointer(
            2, // index of the generic vertex attribute ("layout (location = 2)")
            2, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // disable normalization (int-to-float conversion)
            stride, // stride (byte offset between consecutive attributes)
            (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
        );
        // Unbind the objects
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); // EBO must be unbound after VAO, otherwise it is unbound from VAO
    }
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
        unsafe{
            let radius_uniform_location = gl::GetUniformLocation(shader_program.id(), CString::new("timed_colour").unwrap().as_ptr() );
            gl::Uniform1f(
                radius_uniform_location, //GLint, 
                (((u_colour_angle as f32)*2.0*std::f32::consts::PI)/ (LOOP_TIME as f32)) as gl::types::GLfloat
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
            // Draw our triangle:
            gl::BindVertexArray(vao);
            
            // This is the EBO rendering method:
            gl::DrawElements(
                gl::TRIANGLES,  // render method
                6 as gl::types::GLsizei,  // Count of indexes to render
                gl::UNSIGNED_INT,    // Type in EBO
                std::ptr::null(),  // Offset in the EBO
            );
        }
        
        window.gl_swap_window();
    }
}
