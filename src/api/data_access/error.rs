use std::ffi;

use crate::api::plugin::PluginError;

/// An error returned from data access API calls.
#[derive(thiserror::Error, Debug)]
pub enum DataAccessError {
    /// Invalid data ref id returned from X-Plane.
    #[error("invalid data ref id")]
    InvalidDataRefId,
    /// Data ref is orphaned and can't be used.
    #[error("data ref is orphaned")]
    OrphanedDataRef,
    /// Data ref has invalid type.
    #[error("invalid data ref type")]
    InvalidType,
    /// Data ref does not support write operations.
    #[error("data ref is read only")]
    ReadOnlyDataRef,
    /// Array index out of bounds.
    #[error("index out of bounds")]
    OutOfBounds,
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
