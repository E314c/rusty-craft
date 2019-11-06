extern crate gl;

use std::ffi::{CString, CStr};
use crate::ffi_utils;

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    fn from_source(
        source: &CStr,
        shader_type: gl::types::GLenum
    ) -> Result<Shader, String> {
        let id = shader_from_source(source, shader_type)?;

        Ok(Shader { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

}

impl Drop for Shader {
    /**
    When the `Shader` object is dropped, we need to delete the shader from the OpenGL context
    */
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(source: &CStr, shader_type: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    // ShaderId
    let id : gl::types::GLuint = unsafe { gl::CreateShader(shader_type) };

    // Compile it 
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    // Check it was successful
    let mut success: gl::types::GLint = 1;  // 1 == GL_TRUE 

    unsafe {
        // Query the shader, asking about compile status. The result is set into `success`
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        // Failure: Get Compilation error.
        let mut len: gl::types::GLint = 0;
        unsafe {
            // Determine required string length:
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        // Get a suitably sized CString
        let error: CString = ffi_utils::create_whitespace_cstring_of_len(len as usize);

        unsafe {
            // Get the error string
            gl::GetShaderInfoLog(
                id, // The shaderID
                len,    // <maxLength>
                std::ptr::null_mut(),   // We already have the length, so we don't care
                error.as_ptr() as *mut gl::types::GLchar
            );
        }
        // Return the error string
        return Err(error.to_string_lossy().into_owned());
    }

    // Success
    return Ok(id);
}
