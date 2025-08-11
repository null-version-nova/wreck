use std::error::Error;
use std::ffi::CStr;
use std::fmt;
use crate::ffi::sdl::*;

#[derive(Debug)]
pub struct SdlError {
    pub message: String
}

impl SdlError {
    pub fn get() -> SdlError {
        unsafe {
            let cstring = CStr::from_ptr(SDL_GetError() as *mut i8);
            SdlError { 
                message: String::from(cstring.to_str().unwrap())
            }
        }
    }
}

impl Error for SdlError {}

impl fmt::Display for SdlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ code: {} }}", self)
    }
}
