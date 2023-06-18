/// Modifier keys.
pub enum KeyFlag {
    Shift,
    OptionAlt,
    Control,
    Down,
    Up,
}

/// Key flags bitmap.
pub struct KeyFlags(xplm_sys::XPLMKeyFlags);

impl KeyFlags {
    pub fn contains(&self, flag: KeyFlag) -> bool {
        match flag {
            KeyFlag::Shift => (self.0 & xplm_sys::xplm_ShiftFlag as xplm_sys::XPLMKeyFlags) != 0,
            KeyFlag::OptionAlt => {
                (self.0 & xplm_sys::xplm_OptionAltFlag as xplm_sys::XPLMKeyFlags) != 0
            }
            KeyFlag::Control => {
                self.0 & (xplm_sys::xplm_ControlFlag as xplm_sys::XPLMKeyFlags) != 0
            }
            KeyFlag::Down => self.0 & (xplm_sys::xplm_DownFlag as xplm_sys::XPLMKeyFlags) != 0,
            KeyFlag::Up => self.0 & (xplm_sys::xplm_UpFlag as xplm_sys::XPLMKeyFlags) != 0,
        }
    }
}

impl From<xplm_sys::XPLMKeyFlags> for KeyFlags {
    fn from(value: xplm_sys::XPLMKeyFlags) -> Self {
        Self(value)
    }
}
