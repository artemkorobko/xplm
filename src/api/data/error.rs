/// An error returned from data API calls.
#[derive(thiserror::Error, Debug)]
pub enum DataAccessError {
    /// Invalid dataref id returned from X-Plane.
    #[error("invalid dataref id")]
    InvalidDataRefId,
}
