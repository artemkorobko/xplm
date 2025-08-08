use std::marker::PhantomData;

use crate::api::data_access::*;

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

impl<T: IntoDataType> DataRefArray<T, ReadOnly> {
    /// Looks up the actual readable data ref that is used to read and write the data.
    ///
    /// # Arguments
    /// * `name` - a data ref name.
    ///
    /// # Returns
    /// Returns a [`DataRefArray`] in case of success. Otherwise, returns [`DataAccessError`].
    pub fn find<N: Into<String>>(name: N) -> Result<Self> {
        let data_ref = find_data_ref(name.into())?;
        Self::try_from(data_ref)
    }

    /// Converts the [`DataRefArray`] with [`ReadOnly`] access mode
    /// to a [`DataRefArray`] with [`ReadWrite`] access mode.
    ///
    /// # Returns
    /// Returns a [`DataRefArray`] in case of success.
    /// Otherwise, returns [`DataAccessError`].
    pub fn writeable(self) -> Result<DataRefArray<T, ReadWrite>> {
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

impl<T: IntoDataType> TryFrom<DataRef> for DataRefArray<T, ReadOnly> {
    type Error = DataAccessError;

    fn try_from(inner: DataRef) -> Result<Self> {
        if !is_data_ref_good(&inner) {
            return Err(DataAccessError::OrphanedDataRef);
        }

        let data_type_id = get_data_ref_types(&inner);
        let data_type_matches = match T::data_type() {
            DataType::Int => data_type_id.contains(DataType::IntArray),
            DataType::Float => data_type_id.contains(DataType::FloatArray),
            DataType::Data => data_type_id.contains(DataType::Data),
            _ => false,
        };

        if !data_type_matches {
            return Err(DataAccessError::InvalidType);
        }

        Ok(Self {
            inner,
            data_type: PhantomData,
            mode_type: PhantomData,
        })
    }
}

macro_rules! impl_data_ref_array {
    ({
        type: $type:ty,
        default: $default:expr,
        read: $read_func:ident,
        write: $write_func:ident,
    }) => {
        impl ArrayRead<$type> for DataRefArray<$type, ReadOnly> {
            fn read(&self, dest: &mut [$type]) -> usize {
                $read_func(&self.inner, 0, dest)
            }

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

        impl ArrayRead<$type> for DataRefArray<$type, ReadWrite> {
            fn read(&self, dest: &mut [$type]) -> usize {
                $read_func(&self.inner, 0, dest)
            }

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
            fn write(&mut self, array: &[$type]) {
                $write_func(&self.inner, 0, array);
            }

            fn write_at<const SIZE: usize>(&mut self, offset: usize, value: $type) -> Result<()> {
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

impl_data_ref_array!({
    type: u8,
    default: 0u8,
    read: get_data_b,
    write: set_data_b,
});
