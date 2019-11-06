extern crate sdl2;
extern crate gl;

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
    let gl_context = window.gl_create_context().unwrap();

    // Load the video subsystem
    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    // Designate the "clear" colour
    unsafe { // unsafe because ClearColor is marked as unsafe.
        gl::Viewport(0, 0, SCREEN_WIDTH!(), SCREEN_HEIGHT!()); // set viewport
        gl::ClearColor(0.5, 0.5, 0.7, 1.0); // Set the 'clear' colour to light blue
    }

    // Get a reference the event stream
    let mut event_pump = sdl.event_pump().unwrap();

    // -- Load the shaders -- //
    // TODO: Load in another file, then import?
    use std::ffi::CString;

    let vert_shader = render::shader::Shader::from_vert_source(
        &CString::new(
            // TODO: can this path be made cross-platform?
            // include_str!("data\\shaders\\triangle.vert")    // For Windows
            include_str!("data/shaders/triangle.vert")   // For Posix
        ).unwrap()
    ).unwrap();

    let frag_shader = render::shader::Shader::from_frag_source(
        &CString::new(
            // include_str!("data\\shaders\\triangle.frag")    // For Windows
            include_str!("data/shaders/triangle.frag")  // For Posix
        ).unwrap()  
    ).unwrap();
    // -- -- //

    let shader_program = render::program::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();
    
    // Set the program as the main shader program
    shader_program.set();

    // Main loop
    'main: loop {
        // Handle events:
        for event in event_pump.poll_iter() {
            use sdl2::event::Event; // Shorten the current namespace
            match event {

                Event::KeyDown {..} => {
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
        }
        
        window.gl_swap_window();
    }
}
