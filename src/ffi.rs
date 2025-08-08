use std::ffi::{CStr, IntoStringError};

pub trait FromCStringBytes: Sized {
    fn from_cstring_buffer(bytes: &[u8]) -> Result<Self, IntoStringError>;
}

impl FromCStringBytes for String {
    fn from_cstring_buffer(bytes: &[u8]) -> Result<Self, IntoStringError> {
        let ptr = bytes.as_ptr().cast::<i8>();
        // Safety: the ptr argument is coming from a safe context and always correct
        unsafe { CStr::from_ptr(ptr).to_owned().into_string() }
    }
}
