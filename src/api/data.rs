pub mod dataref;
pub mod error;

pub use self::error::DataAccessError;

pub type Result<T> = std::result::Result<T, DataAccessError>;
