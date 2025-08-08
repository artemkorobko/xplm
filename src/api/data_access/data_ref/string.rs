use std::marker::PhantomData;

use crate::api::data_access::*;
use crate::ffi::FromCStringBytes;

/// A data ref string.
///
/// This is a wrapper around [`DataRef`] that provides type safety for data ref string.
///
/// # Type parameters
/// * `Mode` - a mode of the data ref.
pub struct DataRefString<const SIZE: usize, Mode> {
    inner: DataRefArray<u8, Mode>,
    mode_type: PhantomData<Mode>,
}

impl<const SIZE: usize> DataRefString<SIZE, ReadOnly> {
    /// Looks up the actual readable data ref that is used to read and write the data.
    ///
    /// # Arguments
    /// * `name` - a data ref name.
    ///
    /// # Returns
    /// Returns a [`DataRefArray`] in case of success. Otherwise, returns [`DataAccessError`].
    pub fn find<N: Into<String>>(name: N) -> Result<Self> {
        Ok(Self {
            inner: DataRefArray::find(name)?,
            mode_type: PhantomData,
        })
    }

    pub fn try_read(&self) -> Result<String> {
        let mut bytes = [0u8; SIZE];
        self.inner.read(&mut bytes);
        String::from_cstring_buffer(&bytes).map_err(|_| DataAccessError::InvalidValueString)
    }
}

impl<const SIZE: usize> TryFrom<DataRef> for DataRefString<SIZE, ReadOnly> {
    type Error = DataAccessError;

    fn try_from(inner: DataRef) -> Result<Self> {
        let inner = DataRefArray::try_from(inner)?;

        Ok(Self {
            inner,
            mode_type: PhantomData,
        })
    }
}

impl<const SIZE: usize> DataRead<String> for DataRefString<SIZE, ReadOnly> {
    fn read(&self) -> String {
        self.try_read().unwrap_or_default()
    }
}
