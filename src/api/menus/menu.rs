use super::MenusError;

/// Menu idenitifier.
pub struct MenuId(xplm_sys::XPLMMenuID);

impl MenuId {
    /// Returns the X-Plane menu idenitifier.
    pub fn native(&self) -> xplm_sys::XPLMMenuID {
        self.0
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

/// Menu item identifier.
pub struct MenuItemId(::std::os::raw::c_int);

impl MenuItemId {
    /// Returns the X-Plane menu item idenitifier.
    pub fn native(&self) -> ::std::os::raw::c_int {
        self.0
    }
}

impl TryFrom<::std::os::raw::c_int> for MenuItemId {
    type Error = MenusError;

    fn try_from(value: ::std::os::raw::c_int) -> std::result::Result<Self, Self::Error> {
        if value < 0 {
            Err(Self::Error::InvalidMenuItemId)
        } else {
            Ok(Self(value))
        }
    }
}
