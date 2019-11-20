// TODO: Move to nalgebra matrix implementations
#[derive(Debug)]
pub enum Error {
    InvalidVectorLength,
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Coords3D {
    x : f32,
    y : f32,
    z : f32,
}
impl Coords3D {
    pub fn from_vec(vec: Vec<f32>) -> Result<Coords3D, Error> {
        if  vec.len() != 3 {
            return Err(Error::InvalidVectorLength);
        }

        Ok(Coords3D{
            x : vec[0],
            y : vec[1],
            z : vec[2],
        })
    }
    pub fn new(x:f32,y:f32,z:f32) -> Coords3D {
        Coords3D{x,y,z}
    }
}

impl From<(f32, f32, f32)> for Coords3D {
    fn from(other: (f32, f32, f32)) -> Self {
        Coords3D::new(other.0, other.1, other.2)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Coords2D {
    x : f32,
    y : f32,
}
impl Coords2D {
    pub fn from_vec(vec: Vec<f32>) -> Result<Coords2D, Error> {
        if  vec.len() != 2 {
            return Err(Error::InvalidVectorLength);
        }

        Ok(Coords2D{
            x : vec[0],
            y : vec[1],
        })
    }
}
impl From<(f32, f32)> for Coords2D {
    fn from(other: (f32, f32)) -> Self {
        Coords2D {
            x: other.0,
            y: other.1
        }
    }
}


// -- Colour -- //
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Colour {
    r: f32, g:f32, b: f32, a:f32
}
impl Colour {
    pub fn new(r : f32, g : f32, b : f32, a : f32) -> Colour {
        Colour {
            r,g,b,a
        }
    }
    pub fn from_vec(vec: Vec<f32>) -> Result<Colour, Error> {
        if  vec.len() != 3 && vec.len() != 4 {
            return Err(Error::InvalidVectorLength);
        }

        let mut res = Colour {
            r : vec[0],
            g : vec[1],
            b : vec[2],
            a : 1.0
        };

        if vec.len() == 4 {
            res.a = vec[3];
        }

        Ok(res)
    }
}

impl From<(f32, f32, f32, f32)> for Colour {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        Colour::new(other.0, other.1, other.2, other.3)
    }
}

impl From<(f32, f32, f32)> for Colour {
    fn from(other: (f32, f32, f32)) -> Self {
        Colour::new(other.0, other.1, other.2, 1.0)
    }
}

// -- Vertex -- //
pub trait Vertex {
    fn to_vec(&self) -> Vec<f32>;
    fn from_vec(v: Vec<f32>) -> Result<Self, Error> where Self : Sized;    // TODO: Do I want a Result type? Should I "Size" the class

    fn configure_vao(vao_id: gl::types::GLuint) -> gl::types::GLuint;
}


#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VertexP {
    pub position : Coords3D,
}

use std::ffi;

impl VertexP {
    pub fn from_coords(position: Coords3D) -> VertexP {
        VertexP {
            position
        }
    }

    // -- Upgraders -- //
    pub fn add_color(self, colour: Colour) -> VertexPC {
        VertexPC {
            position: self.position,
            colour: colour,
        }
    }

    pub fn add_texture_coords(self, texture_coords: Coords2D) -> VertexPT {
        VertexPT {
            position: self.position,
            texture_coords: texture_coords,
        }
    }
}
impl Vertex for VertexP {
    fn to_vec(&self) -> Vec<f32> {
        vec![
            self.position.x,
            self.position.y,
            self.position.z
        ]
    }

    fn from_vec(v: Vec<f32>) -> Result<VertexP, Error> {
        match Coords3D::from_vec(v) {
            Ok(x) => {
                Ok(VertexP {
                    position: x
                })
            },
            Err(y) => {Err(y)}, // There's probably better syntax for this...
        }
    }

    // Takes in a VAO id, configure it and returns it.
    fn configure_vao(vao_id: gl::types::GLuint) -> gl::types::GLuint {
        unsafe {
            gl::BindVertexArray(vao_id);
    
            // -- Set Attributes -- //
            let stride = std::mem::size_of::<Coords3D>() as gl::types::GLint;
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

            // Unbind the VAO
            gl::BindVertexArray(0);
        }

        // Return the id
        vao_id
    }
}


#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VertexPC {
    pub position : Coords3D,
    pub colour: Colour,
}

impl VertexPC {
    // -- Upgraders -- //
    pub fn add_texture_coords(self, texture_coords: Coords2D) -> VertexPCT {
        VertexPCT {
            position: self.position,
            colour: self.colour,
            texture_coords: texture_coords,
        }
    }
}
impl Vertex for VertexPC {
    // -- Getter -- //
    fn to_vec(&self) -> Vec<f32> {
        vec![
            self.position.x,
            self.position.y,
            self.position.z,
            self.colour.r,
            self.colour.g,
            self.colour.b,
            self.colour.a,
        ]
    }

    fn from_vec(v: Vec<f32>) -> Result<VertexPC, Error> {
        // Need atleast 7 values
        if v.len() < 7 { return Err(Error::InvalidVectorLength); }

        let position = Coords3D::from_vec(vec![v[0], v[1], v[2]])?;
        let colour = Colour::from_vec(vec![v[3],v[4],v[5],v[6]])?;
        Ok(
            VertexPC {
                position,
                colour,
            }
        )
    }

    // Takes in a VAO id, configure it and returns it.
    fn configure_vao(vao_id: gl::types::GLuint) -> gl::types::GLuint {
        unsafe {
            gl::BindVertexArray(vao_id);
            
            // -- Set Attributes -- //
            let stride = (std::mem::size_of::<Coords3D>() + std::mem::size_of::<Colour>()) as gl::types::GLint;
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
                4, // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // disable normalization (int-to-float conversion)
                stride, // stride (byte offset between consecutive attributes)
                (std::mem::size_of::<Coords3D>()) as *const gl::types::GLvoid // offset of the first component
            );

            // Unbind the VAO
            gl::BindVertexArray(0);
        }
            
        // Return the id
        vao_id
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VertexPT {
    pub position : Coords3D,
    pub texture_coords: Coords2D,
}

impl VertexPT {
    // -- Upgraders -- //
    pub fn add_color(self, colour: Colour) -> VertexPCT {
        VertexPCT {
            position: self.position,
            colour: colour,
            texture_coords: self.texture_coords,
        }
    }
}

impl Vertex for VertexPT {
    // -- Getter -- //
    fn to_vec(&self) -> Vec<f32> {
        vec![
            self.position.x,
            self.position.y,
            self.position.z,
            self.texture_coords.x,
            self.texture_coords.y,
        ]
    }

    fn from_vec(v: Vec<f32>) -> Result<VertexPT, Error> {
        // Need atleast 7 values
        if v.len() < 5 { return Err(Error::InvalidVectorLength); }

        let position = Coords3D::from_vec(vec![v[0], v[1], v[2]])?;
        let texture_coords = Coords2D::from_vec(vec![v[3],v[4]])?;
        Ok(
            VertexPT {
                position,
                texture_coords,
            }
        )
    }

    // Takes in a VAO id, configure it and returns it.
    fn configure_vao(vao_id: gl::types::GLuint) -> gl::types::GLuint {
        unsafe {
            gl::BindVertexArray(vao_id);
            
            // -- Set Attributes -- //
            let stride = (std::mem::size_of::<Coords3D>() + std::mem::size_of::<Coords2D>()) as gl::types::GLint;
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
            // Texture info:
            gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
            gl::VertexAttribPointer(
                1, // index of the generic vertex attribute ("layout (location = 1)")
                2, // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // disable normalization (int-to-float conversion)
                stride, // stride (byte offset between consecutive attributes)
                (std::mem::size_of::<Coords3D>()) as *const gl::types::GLvoid // offset of the first component
            );

            // Unbind the VAO
            gl::BindVertexArray(0);
        }

        // Return the id
        vao_id
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct VertexPCT {
    pub position : Coords3D,
    pub colour: Colour,
    pub texture_coords: Coords2D,
}

// TODO: Implement
impl Vertex for VertexPCT {
    // -- Getter -- //
    fn to_vec(&self) -> Vec<f32> {
        vec![
            self.position.x,
            self.position.y,
            self.position.z,
            self.colour.r,
            self.colour.g,
            self.colour.b,
            self.colour.a,
            self.texture_coords.x,
            self.texture_coords.y,

        ]
    }

    fn from_vec(v: Vec<f32>) -> Result<VertexPCT, Error> {
        // Need atleast 7 values
        if v.len() < 5 { return Err(Error::InvalidVectorLength); }

        let position = Coords3D::from_vec(vec![v[0], v[1], v[2]])?;
        let colour = Colour::from_vec(vec![v[3],v[4],v[5],v[6]])?;
        let texture_coords = Coords2D::from_vec(vec![v[7],v[8]])?;
        Ok(
            VertexPCT {
                position,
                colour,
                texture_coords,
            }
        )
    }

    // Takes in a VAO id, configure it and returns it.
    fn configure_vao(vao_id: gl::types::GLuint) -> gl::types::GLuint {
        unsafe {
            // Bind the VAO to context
            gl::BindVertexArray(vao_id);
            
            // -- Set Attributes -- //
            let stride = (
                std::mem::size_of::<Coords3D>() 
                + std::mem::size_of::<Colour>()
                + std::mem::size_of::<Coords2D>()
            ) as gl::types::GLint;
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
                4, // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // disable normalization (int-to-float conversion)
                stride, // stride (byte offset between consecutive attributes)
                (std::mem::size_of::<Coords3D>()) as *const gl::types::GLvoid // offset of the first component
            );

            // Texture info:
            gl::EnableVertexAttribArray(2); // this is "layout (location = 2)" in vertex shader
            gl::VertexAttribPointer(
                2, // index of the generic vertex attribute ("layout (location = 2)")
                2, // the number of components per generic vertex attribute
                gl::FLOAT, // data type
                gl::FALSE, // disable normalization (int-to-float conversion)
                stride, // stride (byte offset between consecutive attributes)
                (std::mem::size_of::<Coords3D>() + std::mem::size_of::<Colour>()) as *const gl::types::GLvoid // offset of the first component
            );

            // Unbind the VAO
            gl::BindVertexArray(0);
        }
   
        // Return the id
        vao_id
    }
}