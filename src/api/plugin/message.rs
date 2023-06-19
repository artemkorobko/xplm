/// A trait which declares convertion to message parameter.
pub trait AsMessageParam {
    /// Return the memory pointer to the message parameter.
    fn as_message_param(&self) -> *mut ::std::os::raw::c_void;
}

/// A message parameter that gets ignored when sending messages.
pub struct NoMessageParam;

impl AsMessageParam for NoMessageParam {
    fn as_message_param(&self) -> *mut std::os::raw::c_void {
        std::ptr::null_mut()
    }
}
