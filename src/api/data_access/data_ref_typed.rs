use std::marker::PhantomData;

use crate::api::data_access::*;

/// Read-only data ref mode.
pub struct ReadOnly;
/// Read-write data ref mode.
///
/// This mode allows to modify the data ref.
pub struct ReadWrite;

/// Typed data ref.
///
/// This is a wrapper around [`DataRef`] that provides type safety.
///
/// # Type parameters
/// * `T` - a type of the data ref.
/// * `Mode` - a mode of the data ref.
pub struct TypedDataRef<T, Mode> {
    inner: DataRef,
    type_: PhantomData<T>,
    mode: PhantomData<Mode>,
}

impl<T> TypedDataRef<T, ReadOnly> {
    /// Looks up the typed data ref that is used to read the data.
    ///
    /// # Arguments
    /// * `name` - a data ref name.
    ///
    /// # Returns
    /// Returns a [`TypedDataRef`] in case of success. Otherwise, returns [`DataAccessError`].
    pub fn find<N: Into<String>>(name: N) -> Result<Self> {
        let inner = find_data_ref(name)?;

        if !is_data_ref_good(&inner) {
            return Err(DataAccessError::OrphanedDataRef);
        }

        Ok(Self {
            inner,
            type_: PhantomData,
            mode: PhantomData,
        })
    }

    /// Converts the [`ReadOnly`] typed data ref to a [`ReadWrite`].
    ///
    /// # Returns
    /// Returns a [`TypedDataRef`] in case of success. Otherwise, returns [`DataAccessError`].
    pub fn to_writeable(self) -> Result<TypedDataRef<T, ReadWrite>> {
        if !can_write_data_ref(&self.inner) {
            return Err(DataAccessError::ReadOnlyDataRef);
        }

        Ok(TypedDataRef {
            inner: self.inner,
            type_: PhantomData,
            mode: PhantomData,
        })
    }
}

macro_rules! impl_array_read_api {
    ($type:ty, $default:expr, $read_func:ident) => {
        /// Reads the element at the specified offset from the typed array data ref. When there is
        /// a need to read multiple values it's recommended to use [`read_all`] function instead.
        ///
        /// # Arguments
        /// * `offset` - an offset of the element to read.
        ///
        /// # Returns
        /// Returns the element at the specified offset in case of success. Otherwise, returns [`DataAccessError`].
        pub fn read(&self, offset: usize) -> Result<$type> {
            if offset >= MAX_ARRAY_SIZE {
                return Err(DataAccessError::OutOfBounds);
            }

            let mut array = [$default; MAX_ARRAY_SIZE];
            self.read_all(&mut array);
            Ok(array[offset])
        }

        /// Reads the data from the typed array data ref.
        ///
        /// # Arguments
        /// * `array` - an array to read the data into.
        ///
        /// # Returns
        /// Returns the number of elements read.
        pub fn read_all(&self, array: &mut [$type]) -> usize {
            $read_func(&self.inner, 0, array)
        }
    };
}

macro_rules! impl_array_write_api {
    ($type:ty, $write_func:ident) => {
        /// Writes the element at the specified offset into the typed array data ref. When there
        /// is a need to write multiple values it's recommended to use [`write_all`] or [`write_all_at`]
        /// instead.
        ///
        /// # Arguments
        /// * `offset` - an offset of the element to write.
        /// * `value` - a value to write.
        ///
        /// # Returns
        /// Returns unit in case of success. Otherwise, returns [`DataAccessError`].
        pub fn write(&self, offset: usize, value: $type) -> Result<()> {
            self.write_all_at(offset, &[value])
        }

        /// Writes the data to the typed array data ref.
        ///
        /// # Arguments
        /// * `array` - an array to write the data from.
        pub fn write_all(&self, array: &[$type]) -> Result<()> {
            self.write_all_at(0, array)
        }

        /// Writes the data to specified offset into the typed array data ref.
        ///
        /// # Arguments
        /// * `array` - an array to write the data from.
        pub fn write_all_at(&self, offset: usize, array: &[$type]) -> Result<()> {
            if offset >= MAX_ARRAY_SIZE {
                return Err(DataAccessError::OutOfBounds);
            }

            $write_func(&self.inner, offset, array);
            Ok(())
        }
    };
}

macro_rules! impl_typed_array_data_ref {
    ($type:ty, $default:expr, $read_func:ident, $write_func:ident) => {
        impl TypedDataRef<[$type; MAX_ARRAY_SIZE], ReadOnly> {
            impl_array_read_api!($type, $default, $read_func);
        }

        impl TypedDataRef<[$type; MAX_ARRAY_SIZE], ReadWrite> {
            impl_array_read_api!($type, $default, $read_func);
            impl_array_write_api!($type, $write_func);
        }
    };
}

pub const MAX_ARRAY_SIZE: usize = 16;

impl_typed_array_data_ref!(f32, 0.0, get_data_vf, set_data_vf);
