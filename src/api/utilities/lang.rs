use super::UtilitiesError;

/// Defines what language the sim is running in.
pub enum Language {
    Unknown,
    English,
    French,
    German,
    Italian,
    Spanish,
    Korean,
    Russian,
    Greek,
    Japanese,
    Chinese,
}

impl TryFrom<xplm_sys::XPLMLanguageCode> for Language {
    type Error = UtilitiesError;

    fn try_from(value: xplm_sys::XPLMLanguageCode) -> std::result::Result<Self, Self::Error> {
        match value as ::std::os::raw::c_uint {
            xplm_sys::xplm_Language_Unknown => Ok(Self::Unknown),
            xplm_sys::xplm_Language_English => Ok(Self::English),
            xplm_sys::xplm_Language_French => Ok(Self::French),
            xplm_sys::xplm_Language_German => Ok(Self::German),
            xplm_sys::xplm_Language_Italian => Ok(Self::Italian),
            xplm_sys::xplm_Language_Spanish => Ok(Self::Spanish),
            xplm_sys::xplm_Language_Korean => Ok(Self::Korean),
            xplm_sys::xplm_Language_Russian => Ok(Self::Russian),
            xplm_sys::xplm_Language_Greek => Ok(Self::Greek),
            xplm_sys::xplm_Language_Japanese => Ok(Self::Japanese),
            xplm_sys::xplm_Language_Chinese => Ok(Self::Chinese),
            _ => Err(Self::Error::UnknownLanguageCode(value)),
        }
    }
}
