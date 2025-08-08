use std::marker::PhantomData;

use crate::api::data_access::*;

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

impl<T: IntoDataType> DataRefValue<T, ReadOnly> {
    /// Looks up the actual readable data ref that is used to read and write the data.
    ///
    /// # Arguments
    /// * `name` - a data ref name.
    ///
    /// # Returns
    /// Returns a [`DataRefValue`] in case of success. Otherwise, returns [`DataAccessError`].
    pub fn find<N: Into<String>>(name: N) -> Result<Self> {
        let data_ref = find_data_ref(name.into())?;
        Self::try_from(data_ref)
    }

    /// Converts the [`DataRefValue`] with [`ReadOnly`] access mode
    /// to a [`DataRefValue`] with [`ReadWrite`] access mode.
    ///
    /// # Returns
    /// Returns a [`DataRefValue`] in case of success.
    /// Otherwise, returns [`DataAccessError`].
    pub fn writeable(self) -> Result<DataRefValue<T, ReadWrite>> {
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

impl<T: IntoDataType> TryFrom<DataRef> for DataRefValue<T, ReadOnly> {
    type Error = DataAccessError;

    fn try_from(inner: DataRef) -> Result<Self> {
        if !is_data_ref_good(&inner) {
            return Err(DataAccessError::OrphanedDataRef);
        }

        let data_type_matches = get_data_ref_types(&inner).contains(T::data_type());
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

macro_rules! impl_data_ref_value {
    ({
        type: $type:ty,
        read: $read_func:ident,
        write: $write_func:ident,
    }) => {
        impl DataRead<$type> for DataRefValue<$type, ReadOnly> {
            fn read(&self) -> $type {
                $read_func(&self.inner)
            }
        }

        impl DataRead<$type> for DataRefValue<$type, ReadWrite> {
            fn read(&self) -> $type {
                $read_func(&self.inner)
            }
        }

        impl DataWrite<$type> for DataRefValue<$type, ReadWrite> {
            fn write(&mut self, value: $type) {
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
