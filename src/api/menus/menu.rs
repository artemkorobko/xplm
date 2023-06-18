use std::ops::Deref;

use super::MenusError;

/// Menu idenitifier.
pub struct MenuId(xplm_sys::XPLMMenuID);

impl Deref for MenuId {
    type Target = xplm_sys::XPLMMenuID;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<xplm_sys::XPLMMenuID> for MenuId {
    type Error = MenusError;

    fn try_from(value: xplm_sys::XPLMMenuID) -> std::result::Result<Self, MenusError> {
        if value.is_null() {
            Err(Self::Error::InvalidId)
        } else {
            Ok(Self(value))
        }
    }
}
