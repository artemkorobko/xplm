use std::ops::Deref;

use super::UtilitiesError;

/// An opaque identifier for an X-Plane command
pub struct Command(xplm_sys::XPLMCommandRef);

impl TryFrom<xplm_sys::XPLMCommandRef> for Command {
    type Error = UtilitiesError;

    fn try_from(value: xplm_sys::XPLMCommandRef) -> std::result::Result<Self, Self::Error> {
        if value.is_null() {
            Err(Self::Error::InvalidCommand)
        } else {
            Ok(Command(value))
        }
    }
}

impl Deref for Command {
    type Target = xplm_sys::XPLMCommandRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
