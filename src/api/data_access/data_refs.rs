use super::{DataAccessError, DataRef};

pub struct DataRefsIter(*mut xplm_sys::XPLMDataRef);

impl TryFrom<*mut xplm_sys::XPLMDataRef> for DataRefsIter {
    type Error = DataAccessError;

    fn try_from(value: *mut xplm_sys::XPLMDataRef) -> Result<Self, Self::Error> {
        if value.is_null() {
            Err(Self::Error::InvalidDataRefsIterator)
        } else {
            Ok(Self(value))
        }
    }
}

impl Iterator for DataRefsIter {
    type Item = DataRef;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = unsafe { self.0.add(1) };
        if self.0.is_null() {
            None
        } else {
            let data_ref_ptr = unsafe { *self.0 };
            DataRef::try_from(data_ref_ptr).ok()
        }
    }
}
