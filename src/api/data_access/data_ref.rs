pub mod array;
mod string;
pub mod value;

use std::ops::Deref;

use super::{DataAccessError, DataType};

pub use array::DataRefArray;
pub use string::DataRefString;
pub use value::DataRefValue;

/// An opaque handle to data provided by the simulator or another plugin.
pub struct DataRef(xplm_sys::XPLMDataRef);

impl Deref for DataRef {
    type Target = xplm_sys::XPLMDataRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<xplm_sys::XPLMDataRef> for DataRef {
    type Error = DataAccessError;

    fn try_from(value: xplm_sys::XPLMDataRef) -> Result<Self, Self::Error> {
        if value.is_null() {
            Err(Self::Error::InvalidDataRefId)
        } else {
            Ok(DataRef(value))
        }
    }
}

/// Read-only data ref mode.
pub struct ReadOnly;

/// Read-write data ref mode.
///
/// This mode allows modifying the data ref.
pub struct ReadWrite;

/// A trait that provides an abstraction for reading data of a specific type.
///
/// This trait defines a generic method `read` which retrieves data of type `T`.
/// It is implemented by structures that hold or reference data and need to
/// provide a way to extract or access the underlying value.
///
/// # Type Parameters
/// - `T`: The type of data that will be read by the implementation of this trait.
///
/// # Required Methods
///
/// - `fn read(&self) -> T`
///
///   Reads and returns a value of type `T` from the implementing data reference.
///   The exact behavior of this method depends on the specific implementation
///   and context in which the `DataRead` trait is used.
pub trait DataRead<T> {
    #[doc = concat!("Reads a value of ", stringify!($type), " from a data ref.")]
    fn read(&self) -> T;
}

/// A trait for writing data of a specific type into a data reference.
///
/// This trait is generic over the type `T` and provides an abstraction
/// for writing values of the associated type into some underlying data
/// structure or resource.
///
/// # Type Parameters
///
/// - `T`: The type of the data that will be written.
///
/// # Required Methods
///
/// ## `write`
///
/// Writes a value of type `T` into the underlying data reference.
///
/// ### Parameters
/// - `value`: The value of type `T` that will be written.
///
/// ### Examples
///
/// ```rust
/// struct DataContainer<T> {
///     data: Option<T>,
/// }
///
/// impl<T> DataWrite<T> for DataContainer<T> {
///     fn write(&mut self, value: T) {
///         self.data = Some(value);
///     }
/// }
///
/// let mut container = DataContainer { data: None };
/// container.write(42);
/// assert_eq!(container.data, Some(42));
/// ```
pub trait DataWrite<T> {
    #[doc = concat!("Writes a value of ", stringify!($type), " into a data ref.")]
    fn write(&mut self, value: T);
}

pub trait ArrayRead<T> {
    #[doc = concat!("Reads an array of ", stringify!($type), " from a data ref.")]
    #[doc = "# Arguments"]
    #[doc = "* `dest` - a mutable reference to a destination array."]
    #[doc = "# Returns"]
    #[doc = "Returns the amount of elements written into the `dest` array."]
    fn read(&self, array: &mut [T]) -> usize;

    #[doc = concat!("Reads ", stringify!($type), " from an array at specific offset.")]
    #[doc = "# Arguments"]
    #[doc = "* `offset` - an offset in the data ref array to start read from."]
    #[doc = "# Returns"]
    #[doc = concat!("Returns ", stringify!($type), " value in case of success. Otherwise returns [`DataAccessError`]")]
    fn read_at<const SIZE: usize>(&self, offset: usize) -> super::Result<T>;
}

pub trait ArrayWrite<T> {
    #[doc = concat!("Writes an array of ", stringify!($type), " into a data ref.")]
    #[doc = "# Arguments"]
    #[doc = "* `array` - a reference to a source array."]
    fn write(&mut self, array: &[T]);
    #[doc = concat!("Writes a ", stringify!($type), " value into a data ref at the specific offset.")]
    #[doc = "# Arguments"]
    #[doc = "* `offset` - an offset in the data ref array to write the value to."]
    #[doc = "* `value` - a value to write into the data ref array."]
    fn write_at<const SIZE: usize>(&mut self, offset: usize, value: T) -> super::Result<()>;
}

pub trait IntoDataType {
    #[doc = concat!("Returns a type of the data ref.")]
    fn data_type() -> DataType;
}

impl IntoDataType for i32 {
    fn data_type() -> DataType {
        DataType::Int
    }
}

impl IntoDataType for f32 {
    fn data_type() -> DataType {
        DataType::Float
    }
}

impl IntoDataType for f64 {
    fn data_type() -> DataType {
        DataType::Double
    }
}

impl IntoDataType for u8 {
    fn data_type() -> DataType {
        DataType::Data
    }
}
