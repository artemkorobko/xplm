use super::DisplayError;

/// The mouse status.
pub enum MouseStatus {
    /// The mouse button is up.
    Up,
    /// The mouse button is down.
    Down,
    /// The mouse started drag move.
    Drag,
}

impl TryFrom<xplm_sys::XPLMMouseStatus> for MouseStatus {
    type Error = DisplayError;

    fn try_from(value: xplm_sys::XPLMMouseStatus) -> Result<Self, Self::Error> {
        match value as _ {
            xplm_sys::xplm_MouseUp => Ok(Self::Up),
            xplm_sys::xplm_MouseDown => Ok(Self::Down),
            xplm_sys::xplm_MouseDrag => Ok(Self::Drag),
            _ => Err(Self::Error::UnknownMouseStatuts(value)),
        }
    }
}

/// The mouse wheel axis.
pub enum WheelAxis {
    /// Vertical mouse wheel axis.
    Vertical,
    /// Horizontal mouse wheel axis.
    Horizontal,
}

impl TryFrom<::std::os::raw::c_int> for WheelAxis {
    type Error = DisplayError;

    fn try_from(value: ::std::os::raw::c_int) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Vertical),
            1 => Ok(Self::Horizontal),
            _ => Err(Self::Error::UnknownMouseWheelAxis(value)),
        }
    }
}
