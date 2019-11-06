use std::ffi::{CString};
pub fn create_whitespace_cstring_of_len(len: usize) -> CString {
    // Create a a vector of appropriate length
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with spaces
    buffer.extend([b' '].iter().cycle().take(len));    // Fill with spaces (using a cycling iterator over an array of a single space)

    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }    // Safe to use `from_vec_unchecked` as we created it without NULLs (only spaces)
}
