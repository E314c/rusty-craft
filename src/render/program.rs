extern crate gl;

// pub mod shader;
use crate::render::shader;
use crate::ffi_utils;

// Shader program
pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    // Compile a program from a collection of Shaders
    pub fn from_shaders(shaders: &[shader::Shader]) -> Result<Program, String> {
        // Create a program
        let program_id = unsafe { gl::CreateProgram() };

        // Attach the shaders
        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }

        // Link the program
        unsafe { gl::LinkProgram(program_id); }

        // Check that the program linked correctly
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = ffi_utils::create_whitespace_cstring_of_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,             // the program id
                    len,                    // length of the string buffer we've allocated
                    std::ptr::null_mut(),   // We already know the length
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }


            return Err(error.to_string_lossy().into_owned());
        }

        // After linking, detach the shaders
        for shader in shaders {
            unsafe { gl::DetachShader(program_id, shader.id()); }
        }

        Ok(Program { id: program_id })
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        // We need to delete the program if this object is dropped
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
