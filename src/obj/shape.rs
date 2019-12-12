use crate::obj::vertex::{self, Vertex, Coords3D};

pub struct Shape<T : Vertex> {
    verts: Vec<T>,
    triangles: Vec<(u32,u32,u32)>,   // Triangles from the indexes of the arrays in `verts` (sets of 3 indexes)

    // GL IDs owned by this shape
    vbo: gl::types::GLuint,
    ebo: gl::types::GLuint,
    // TODO: Is VAO owned by a single shape? or should we reuse across many shapes? (would assume reuse if possible)
    vao: gl::types::GLuint,
}

impl <T:Vertex> Shape<T> {
    
    pub fn from_vertices_and_triangle(verts: Vec<T>, triangles: Vec<(u32,u32,u32)>) -> Shape<T> {
        Shape {
            verts,
            triangles,
            ebo: 0,
            vbo:0,
            vao:0,
        }
    }


    // Getters.

    // Setup functions:
    pub fn setup(&mut self) {
        // Configure the different GL aspects
        self.setup_vbo();
        self.setup_ebo();

        self.setup_vao();
    }

    // TODO: I don't think these need to be public?
    fn setup_vbo(&mut self) {
        unsafe {
            gl::GenBuffers(1, &mut self.vbo);    // Create 1 buffer.
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);  // Bind as an ARRAY_BUFFER
            gl::BufferData(
                gl::ARRAY_BUFFER, // target
                (self.verts.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr, // size of data in bytes
                self.verts.as_ptr() as *const gl::types::GLvoid, // pointer to data (Verts are tight packed, so this should work)
                gl::STATIC_DRAW, // usage hint: Data rarely changes, used for drawing
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
        }
    }
    fn setup_ebo(&mut self) {
        unsafe {
            gl::GenBuffers(1, &mut self.ebo);    // Create 1 buffer.
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);  // Bind as an ELEMENT_ARRAY_BUFFER
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER, // target
                (self.triangles.len() * 3 * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr, // size of data in bytes
                self.triangles.as_ptr() as *const gl::types::GLvoid, // pointer to data
                gl::STATIC_DRAW, // usage hint: Data rarely changes, used for drawing
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); // unbind the buffer
        }
    }

    fn setup_vao(&mut self) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);  // re-bind the vbo into the context
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);  // re-bind the ebo into the context

            // Use the Vertex's method to configure the VAO strides and locations:
            <T>::configure_vao(self.vao);

            // Unbind the objects
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); // EBO must be unbound after VAO, otherwise it is unbound from VAO
        }
    }

    pub fn bind_vao(&self) {
        // TODO: Panic if VAO not yet configured? or run setup now?

        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn draw(&self) {
        // Make sure our vao is bound:
        self.bind_vao();

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,  // render method
                (self.triangles.len() * 3) as gl::types::GLsizei,  // Count of indexes to render
                gl::UNSIGNED_INT,    // Type in EBO
                std::ptr::null(),  // Offset in the EBO
            );
        }
    }
}

// TODO: Implement Drop trait:
// impl Drop for Shape<T> {
//     /**
//     When the Shape is dropped, we should delete associated VAOs, VBOs and EBOs
//     */
//     fn drop(&mut self) {
//         unsafe {
//         }
//     }
// }

/**
 * An Object is a specific instance of a given shape: It will contain a reference to the original shape, 
 * but will have it's own local->world space transformations.
 */
use std::rc::Rc;
use std::ffi::{CString};

pub struct Object<T:Vertex> {
    shape: Rc<Shape<T>>,

    // Local -> world space transformation information
    pub rotation: Coords3D,
    pub translation: Coords3D,
    pub scale: Coords3D,
}

impl <T:Vertex> Object<T> { 
    pub fn new(shape: &Rc<Shape<T>>) -> Object<T> {
        Object {
            shape: Rc::clone(shape),
            rotation: (0.0, 0.0, 0.0).into(),
            translation: (0.0, 0.0, 0.0).into(),
            scale: (0.0, 0.0, 0.0).into(),
        }
    }

    pub fn draw(&self, shader_program_id : gl::types::GLuint) {
        // Setup up the transformations for openGL
        unsafe {
            let translation_location = gl::GetUniformLocation(shader_program_id, CString::new("translation").unwrap().as_ptr());
            gl::Uniform3f(
                translation_location,
                self.translation.x,
                self.translation.y,
                self.translation.z
            );
            let rotation_location = gl::GetUniformLocation(shader_program_id, CString::new("rotation").unwrap().as_ptr());
            gl::Uniform3f(
                rotation_location,
                self.rotation.x,
                self.rotation.y,
                self.rotation.z
            );
            let scale_location = gl::GetUniformLocation(shader_program_id, CString::new("scale").unwrap().as_ptr());
            gl::Uniform3f(
                scale_location,
                self.scale.x,
                self.scale.y,
                self.scale.z
            );
        }

        // draw
        self.shape.draw();
    }
}