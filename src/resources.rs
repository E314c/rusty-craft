/**
This file handles the loading of assets and other resources
// TODO: put into module folder with separate modules per resource type?
*/

use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Read};
use std::ffi;
use image::{self, png};

// Custom errors for this module
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FailedToGetExePath,
    FileContainsNullByte,
    ImageDecodeFailed
}

// Create a caster for io:Error to our Error
impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

pub struct Resources {
    root_path: PathBuf,
}

impl Resources {
    /**
    Create a new Resources object for a given Path
    */
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Resources, Error> {
        
        let exe_file_name = ::std::env::current_exe()   // Get path of current executable
            .map_err(|_| Error::FailedToGetExePath)?;   // Map any errors to our "FailedTOGetExePath" error
        // Get the directory:
        let exe_path = exe_file_name.parent().ok_or(Error::FailedToGetExePath)?;    // `ok_or`: If no parent, throw error

        Ok(Resources {
            root_path: exe_path.join(rel_path)
        }) 
    }

    // -- Instance methods -- //
    // Loads a file as a CString
    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(
            resource_name_to_path(&self.root_path,resource_name)
        )?;

        // Create a buffer for the file contents
        let mut buffer: Vec<u8> = Vec::with_capacity(
            (file.metadata()?.len() as usize) + 1
        );
        file.read_to_end(&mut buffer)?;

        // check for null byte
        // if buffer.iter().find(|i| **i == 0).is_some() {
        if buffer.iter().find(|&&i| i == 0).is_some() {
            return Err(Error::FileContainsNullByte);
        }

        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
    }


    pub fn load_image(&self, image_name : &str) -> Result<png::PNGDecoder<fs::File>, Error> {
        // Image file handle 
        let file = fs::File::open(
            resource_name_to_path(&self.root_path, image_name)
        )?;

        // Get a (png) decoder
        png::PNGDecoder::new(file).or_else(|e| {
            println!("{:?}",e);
            Err(Error::ImageDecodeFailed)
        })
    }
}


fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    // this will allow us to specify posix style paths and have them work on windows
    for part in location.split("/") {
        path = path.join(part);
    }

    path
}
