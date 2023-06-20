use std::ffi;

use crate::api::plugin::PluginError;

/// An error returned from data access API calls.
#[derive(thiserror::Error, Debug)]
pub enum DataAccessError {
    /// Invalid data ref id returned from X-Plane.
    #[error("invalid data ref id")]
    InvalidDataRefId,
    /// Invalid data refs iterator returned from X-Plane.
    #[error("invalid data refs iterator")]
    InvalidDataRefsIterator,
    /// Invalid data ref name passed from X-Plane.
    #[error("invalid data ref name string {0}")]
    InvalidInfoName(ffi::IntoStringError),
    /// Invalid data ref name passed to X-Plane.
    #[error("invalid data ref name string {0}")]
    InvalidDataRefName(ffi::NulError),
    /// Plugin error.
    #[error("plugin error {0}")]
    Plugin(PluginError),
}

impl From<PluginError> for DataAccessError {
    fn from(value: PluginError) -> Self {
        Self::Plugin(value)
    }
}
