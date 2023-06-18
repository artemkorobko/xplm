use std::ops::Deref;

use super::error::PluginError;

/// A plugin identifier
#[derive(Copy, Clone, Debug)]
pub struct PluginId(xplm_sys::XPLMPluginID);

impl Deref for PluginId {
    type Target = xplm_sys::XPLMPluginID;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<xplm_sys::XPLMPluginID> for PluginId {
    type Error = PluginError;

    fn try_from(value: xplm_sys::XPLMPluginID) -> std::result::Result<Self, PluginError> {
        if value < 0 {
            Err(Self::Error::InvalidId(value))
        } else {
            Ok(PluginId(value))
        }
    }
}
