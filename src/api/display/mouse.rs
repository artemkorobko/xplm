use super::DisplayError;

/// Mouse status passed in [`WindowHandler::mouse_click`].
pub enum MouseStatus {
    Up,
    Down,
    Drag,
}

impl TryFrom<xplm_sys::XPLMMouseStatus> for MouseStatus {
    type Error = DisplayError;

    fn try_from(value: xplm_sys::XPLMMouseStatus) -> std::result::Result<Self, Self::Error> {
        match value as _ {
            xplm_sys::xplm_MouseUp => Ok(Self::Up),
            xplm_sys::xplm_MouseDown => Ok(Self::Down),
            xplm_sys::xplm_MouseDrag => Ok(Self::Drag),
            _ => Err(Self::Error::UnknownMouseStatuts(value)),
        }
    }
}
