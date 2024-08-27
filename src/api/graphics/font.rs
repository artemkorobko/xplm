/// An X-Plane font.
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum Font {
    /// Mono-spaced font for user interface. Available in all versions of the SDK.
    Basic = xplm_sys::xplmFont_Basic,
    /// Proportional UI font.
    Proportional = xplm_sys::xplmFont_Proportional,
}

impl From<Font> for xplm_sys::XPLMFontID {
    fn from(value: Font) -> Self {
        value as _
    }
}

/// A font dimensions.
pub struct FontDimensions {
    /// Font width.
    pub width: i32,
    /// Font height.
    pub height: i32,
}
