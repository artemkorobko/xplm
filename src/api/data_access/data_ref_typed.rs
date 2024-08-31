use std::marker::PhantomData;

use crate::api::data_access::*;

/// Read-only data ref mode.
pub struct ReadOnly;
/// Read-write data ref mode.
///
/// This mode allows to modify the data ref.
pub struct ReadWrite;

/// A data ref value.
///
/// This is a wrapper around [`DataRef`] that provides type safety for data ref value.
///
/// # Type parameters
/// * `T` - a type of the data ref.
/// * `Mode` - a mode of the data ref.
pub struct DataRefValue<T, Mode> {
    inner: DataRef,
    data_type: PhantomData<T>,
    mode_type: PhantomData<Mode>,
}

impl<T> DataRefValue<T, ReadOnly> {
    /// Converts the [`DataRefValue`] with [`ReadOnly`] access mode
    /// to a [`DataRefValue`] with [`ReadWrite`] access mode.
    ///
    /// # Returns
    /// Returns a [`DataRefValue`] in case of success.
    /// Otherwise, returns [`DataAccessError`].
    pub fn to_writeable(self) -> Result<DataRefValue<T, ReadWrite>> {
        if !can_write_data_ref(&self.inner) {
            return Err(DataAccessError::ReadOnlyDataRef);
        }

        Ok(DataRefValue {
            inner: self.inner,
            data_type: PhantomData,
            mode_type: PhantomData,
        })
    }
}

impl<T> TryFrom<DataRef> for DataRefValue<T, ReadOnly> {
    type Error = DataAccessError;

    fn try_from(inner: DataRef) -> Result<Self> {
        if !is_data_ref_good(&inner) {
            return Err(DataAccessError::OrphanedDataRef);
        }

        Ok(Self {
            inner,
            data_type: PhantomData,
            mode_type: PhantomData,
        })
    }
}

pub trait DataRead<T> {
    fn read(&self) -> T;
}

pub trait DataWrite<T> {
    fn write(&self, value: T);
}

macro_rules! impl_data_ref_value {
    ({
        type: $type:ty,
        read: $read_func:ident,
        write: $write_func:ident,
    }) => {
        impl DataRead<$type> for DataRefValue<$type, ReadOnly> {
            #[doc = concat!("Reads a value of ", stringify!($type), " from a data ref.")]
            fn read(&self) -> $type {
                $read_func(&self.inner)
            }
        }

        impl DataWrite<$type> for DataRefValue<$type, ReadWrite> {
            fn write(&self, value: $type) {
                $write_func(&self.inner, value);
            }
        }
    };
}

impl_data_ref_value!({
    type: f64,
    read: get_data_d,
    write: set_data_d,
});
impl_data_ref_value!({
    type: f32,
    read: get_data_f,
    write: set_data_f,
});
impl_data_ref_value!({
    type: i32,
    read: get_data_i,
    write: set_data_i,
});

/// A data ref array.
///
/// This is a wrapper around [`DataRef`] that provides type safety for data ref array.
///
/// # Type parameters
/// * `T` - a type of the data ref.
/// * `Mode` - a mode of the data ref.
pub struct DataRefArray<T, Mode> {
    inner: DataRef,
    data_type: PhantomData<T>,
    mode_type: PhantomData<Mode>,
}

impl<T> DataRefArray<T, ReadOnly> {
    /// Converts the [`DataRefArray`] with [`ReadOnly`] access mode
    /// to a [`DataRefArray`] with [`ReadWrite`] access mode.
    ///
    /// # Returns
    /// Returns a [`DataRefArray`] in case of success.
    /// Otherwise, returns [`DataAccessError`].
    pub fn to_writeable(self) -> Result<DataRefArray<T, ReadWrite>> {
        if !can_write_data_ref(&self.inner) {
            return Err(DataAccessError::ReadOnlyDataRef);
        }

        Ok(DataRefArray {
            inner: self.inner,
            data_type: PhantomData,
            mode_type: PhantomData,
        })
    }
}

impl<T> TryFrom<DataRef> for DataRefArray<T, ReadOnly> {
    type Error = DataAccessError;

    fn try_from(inner: DataRef) -> Result<Self> {
        if !is_data_ref_good(&inner) {
            return Err(DataAccessError::OrphanedDataRef);
        }

        Ok(Self {
            inner,
            data_type: PhantomData,
            mode_type: PhantomData,
        })
    }
}

pub trait ArrayRead<T> {
    fn read(&self, array: &mut [T]) -> usize;
    fn read_at<const SIZE: usize>(&self, offset: usize) -> Result<T>;
}

pub trait ArrayWrite<T> {
    fn write(&self, array: &[T]);
    fn write_at<const SIZE: usize>(&self, offset: usize, value: T) -> Result<()>;
}

macro_rules! impl_data_ref_array {
    ({
        type: $type:ty,
        default: $default:expr,
        read: $read_func:ident,
        write: $write_func:ident,
    }) => {
        impl ArrayRead<$type> for DataRefArray<$type, ReadOnly> {
            #[doc = concat!("Reads an array of ", stringify!($type), " from a data ref.")]
            #[doc = "# Arguments"]
            #[doc = "* `dest` - a mutable reference to a destination array."]
            #[doc = "# Returns"]
            #[doc = "Returns the amount of elements written into the `dest` array."]
            fn read(&self, dest: &mut [$type]) -> usize {
                $read_func(&self.inner, 0, dest)
            }

            #[doc = concat!("Reads ", stringify!($type), " from an array at specific offset.")]
            #[doc = "# Arguments"]
            #[doc = "* `offset` - an offset in the data ref array to start read from."]
            #[doc = "# Returns"]
            #[doc = concat!("Returns ", stringify!($type), " value in case of success. Otherwise returns [`DataAccessError`]")]
            fn read_at<const SIZE: usize>(&self, offset: usize) -> Result<$type> {
                if offset >= SIZE {
                    return Err(DataAccessError::OutOfBounds);
                }

                let mut array = [$default; SIZE];
                if self.read(&mut array) != array.len() {
                    return Err(DataAccessError::OutOfBounds);
                }

                Ok(array[offset])
            }
        }

        impl ArrayWrite<$type> for DataRefArray<$type, ReadWrite> {
            fn write(&self, array: &[$type]) {
                $write_func(&self.inner, 0, array);
            }

            fn write_at<const SIZE: usize>(&self, offset: usize, value: $type) -> Result<()> {
                if offset >= SIZE {
                    return Err(DataAccessError::OutOfBounds);
                }

                $write_func(&self.inner, offset, &[value]);
                Ok(())
            }
        }
    };
}

impl_data_ref_array!({
    type: f32,
    default: 0.0f32,
    read: get_data_vf,
    write: set_data_vf,
});
