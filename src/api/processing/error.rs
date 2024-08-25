/// An error returned from processing API calls.
#[derive(thiserror::Error, Debug)]
pub enum ProcessingError {
    /// Invalid flight loop ID.
    #[error("invalid flight loop id")]
    InvalidFlightLoopId,
}
