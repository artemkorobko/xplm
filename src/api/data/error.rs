/// An error returned from data API calls.
#[derive(thiserror::Error, Debug)]
pub enum DataAccessError {
    /// Invalid dataref id returned from X-Plane.
    #[error("invalid dataref id")]
    InvalidDataRefId,
    /// Unknown data type id returned from X-Plane.
    #[error("unknown data type id")]
    UnknownDataTypeId(xplm_sys::XPLMDataTypeID),
}
