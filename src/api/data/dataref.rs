use std::ops::Deref;

use super::DataAccessError;

/// An opaque handle to data provided by the simulator or another plugin.
pub struct DataRef(xplm_sys::XPLMDataRef);

impl Deref for DataRef {
    type Target = xplm_sys::XPLMDataRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<xplm_sys::XPLMDataRef> for DataRef {
    type Error = DataAccessError;

    fn try_from(value: xplm_sys::XPLMDataRef) -> std::result::Result<Self, Self::Error> {
        if value.is_null() {
            Err(Self::Error::InvalidDataRefId)
        } else {
            Ok(DataRef(value))
        }
    }
}
