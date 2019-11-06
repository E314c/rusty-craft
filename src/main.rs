use std::path::{Path};

extern crate sdl2;
extern crate gl;

pub mod resources;
pub mod render;
pub mod ffi_utils;

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
    
    // Designate the "clear" colour
    unsafe { // unsafe because ClearColor is marked as unsafe.
        gl::Viewport(0, 0, SCREEN_WIDTH!(), SCREEN_HEIGHT!()); // set viewport
        gl::ClearColor(0.5, 0.5, 0.7, 1.0); // Set the 'clear' colour to light blue
    }

    // Get a reference the event stream
    let mut event_pump = sdl.event_pump().unwrap();

    // -- Load the shaders -- //
    // TODO: Load in another file, then import?
    let resources = resources::Resources::from_relative_exe_path(Path::new("assets")).unwrap();
    let vert_shader = render::shader::Shader::from_resource(&resources, "shaders/triangle.vert").unwrap();
    let frag_shader = render::shader::Shader::from_resource(&resources, "shaders/triangle.frag").unwrap();
    // -- -- //

    let shader_program = render::program::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();
    
    // Set the program as the main shader program
    shader_program.set();


    // -- Declaring a shape to render -- //
    const VERT_LENGTH: usize = 6;
    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,    1.0, 0.0, 0.0,   // A triangle with RGB information
         0.5, -0.5, 0.0,    0.0, 1.0, 0.0,
         0.0,  0.5, 0.0,    0.0, 0.0, 1.0
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

    // Create a Vertex Array Object (VAO)
    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);  // re-bind the vbo into the context

        // Set Attributes
        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // disable normalization (int-to-float conversion)
            (VERT_LENGTH * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );
        gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
        gl::VertexAttribPointer(
            1, // index of the generic vertex attribute ("layout (location = 1)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // disable normalization (int-to-float conversion)
            (VERT_LENGTH * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            (VERT_LENGTH/2 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
        );
        // Unbind the objects
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    // Main loop
    'main: loop {
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
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }
        
        window.gl_swap_window();
    }
}
