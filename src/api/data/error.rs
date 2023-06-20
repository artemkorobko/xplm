use std::ffi;

use crate::api::plugin::PluginError;

/// An error returned from data API calls.
#[derive(thiserror::Error, Debug)]
pub enum DataAccessError {
    /// Invalid dataref id returned from X-Plane.
    #[error("invalid dataref id")]
    InvalidDataRefId,
    /// Unknown data type id returned from X-Plane.
    #[error("unknown data type id")]
    UnknownDataTypeId(xplm_sys::XPLMDataTypeID),
    /// Invalid datarefs iterator returned from X-Plane.
    #[error("invalid datarefs iterator")]
    InvalidDataRefsIterator,
    /// Invalid dataref name passed from X-Plane.
    #[error("invalid dataref name string {0}")]
    InvalidInfoName(ffi::IntoStringError),
    /// Plugin error.
    #[error("plugin error {0}")]
    Plugin(PluginError),
}

impl From<PluginError> for DataAccessError {
    fn from(value: PluginError) -> Self {
        Self::Plugin(value)
    }
}
