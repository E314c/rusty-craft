extern crate gl;
extern crate image;

use image::ImageDecoder;
use crate::resources::Resources;
use std::convert::TryInto;

pub struct Texture {
    id: gl::types::GLuint,
}

impl Texture {
    // Load a shader from a resource
    pub fn from_resource(res: &Resources, name: &str) -> Result<Texture, String> {
        
        let image_decoder = res.load_image(name)
            .map_err(|e| format!("Error loading resource {}: {:?}", name, e))?;


        let (width, height) = image_decoder.dimensions();
        let data = image_decoder.read_image().unwrap();

        // Load the texture into OpenGL
        let mut id : gl::types::GLuint = 0;
        unsafe { 
            gl::GenTextures(1, &mut id); 
            gl::BindTexture(gl::TEXTURE_2D, id);
            // Configure this texture
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as gl::types::GLint);

            // Push image data
            gl::TexImage2D(
                gl::TEXTURE_2D, 
                0, 
                gl::RGB as gl::types::GLint, 
                width.try_into().unwrap(),
                height.try_into().unwrap(), 
                0, 
                gl::RGB, 
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const std::ffi::c_void
            );
            // gl::GenerateMipmap(gl::TEXTURE_2D);
        };

        Ok(Texture{id})
    }

    // Getter for own ID
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

}

impl Drop for Texture {
    /**
    When the `Shader` object is dropped, we need to delete the shader from the OpenGL context
    */
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}