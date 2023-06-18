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
    /// Checks whether flags contains the specific flag.
    ///
    /// # Arguments
    /// * `flag` - the flag to check.
    ///
    /// # Returns
    /// Return `true` if flags contains specific flag. Otherwise returns `false`.
    pub fn contains(&self, flag: KeyFlag) -> bool {
        match flag {
            KeyFlag::Shift => self.contains_shift_flag(),
            KeyFlag::OptionAlt => self.contains_option_alt_flag(),
            KeyFlag::Control => self.contains_control_flag(),
            KeyFlag::Down => self.contains_down_flag(),
            KeyFlag::Up => self.contains_up_flag(),
        }
    }

    /// Checks whether flags contains shift flag.
    ///
    /// # Returns
    /// Return `true` if flags contains shift. Otherwise returns `false`.
    pub fn contains_shift_flag(&self) -> bool {
        self.0 & (xplm_sys::xplm_ShiftFlag as xplm_sys::XPLMKeyFlags) != 0
    }

    /// Checks whether flags contains option or alt flag.
    ///
    /// # Returns
    /// Return `true` if flags contains option or alt. Otherwise returns `false`.
    pub fn contains_option_alt_flag(&self) -> bool {
        self.0 & (xplm_sys::xplm_OptionAltFlag as xplm_sys::XPLMKeyFlags) != 0
    }

    /// Checks whether flags contains control flag.
    ///
    /// # Returns
    /// Return `true` if flags contains control. Otherwise returns `false`.
    pub fn contains_control_flag(&self) -> bool {
        self.0 & (xplm_sys::xplm_OptionAltFlag as xplm_sys::XPLMKeyFlags) != 0
    }

    /// Checks whether flags contains down flag.
    ///
    /// # Returns
    /// Return `true` if flags contains down. Otherwise returns `false`.
    pub fn contains_down_flag(&self) -> bool {
        self.0 & (xplm_sys::xplm_DownFlag as xplm_sys::XPLMKeyFlags) != 0
    }

    /// Checks whether flags contains up flag.
    ///
    /// # Returns
    /// Return `true` if flags contains up. Otherwise returns `false`.
    pub fn contains_up_flag(&self) -> bool {
        self.0 & (xplm_sys::xplm_UpFlag as xplm_sys::XPLMKeyFlags) != 0
    }
}

impl From<xplm_sys::XPLMKeyFlags> for KeyFlags {
    fn from(value: xplm_sys::XPLMKeyFlags) -> Self {
        Self(value)
    }
}
