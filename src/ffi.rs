use std::string::FromUtf8Error;

pub trait FromStringBuf: Sized {
    fn from_string_buf<const SIZE: usize>(buf: [i8; SIZE]) -> Result<Self, FromUtf8Error>;
}

impl FromStringBuf for String {
    fn from_string_buf<const SIZE: usize>(buf: [i8; SIZE]) -> Result<Self, FromUtf8Error> {
        let chars_before_null = buf.into_iter().map(|c| c as u8).take_while(|&c| c != b'\0');
        Self::from_utf8(chars_before_null.collect())
    }
}
