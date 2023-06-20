use std::ops::Deref;

use super::DataAccessError;

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

    fn try_from(value: xplm_sys::XPLMDataRef) -> std::result::Result<Self, Self::Error> {
        if value.is_null() {
            Err(Self::Error::InvalidDataRefId)
        } else {
            Ok(DataRef(value))
        }
    }
}

/// Enumeration that defines the type of the data behind a data reference.
pub enum DataTypeId {
    /// Data of a type the current XPLM doesn't do.
    Unknown,
    /// A single 4-byte integer, native endian.
    Int,
    /// A single 4-byte float, native endian.
    Float,
    /// A single 8-byte double, native endian.
    Double,
    /// An array of 4-byte floats, native endian.
    FloatArray,
    /// An array of 4-byte integers, native endian.
    IntArray,
    /// A variable block of data.
    Data,
}

impl TryFrom<xplm_sys::XPLMDataTypeID> for DataTypeId {
    type Error = DataAccessError;

    fn try_from(value: xplm_sys::XPLMDataTypeID) -> Result<Self, Self::Error> {
        match value as _ {
            xplm_sys::xplmType_Unknown => Ok(DataTypeId::Unknown),
            xplm_sys::xplmType_Int => Ok(DataTypeId::Int),
            xplm_sys::xplmType_Float => Ok(DataTypeId::Float),
            xplm_sys::xplmType_Double => Ok(DataTypeId::Double),
            xplm_sys::xplmType_FloatArray => Ok(DataTypeId::FloatArray),
            xplm_sys::xplmType_IntArray => Ok(DataTypeId::IntArray),
            xplm_sys::xplmType_Data => Ok(DataTypeId::Data),
            _ => Err(Self::Error::UnknownDataTypeId(value)),
        }
    }
}

impl From<DataTypeId> for xplm_sys::XPLMDataTypeID {
    fn from(value: DataTypeId) -> Self {
        match value {
            DataTypeId::Unknown => xplm_sys::xplmType_Unknown as _,
            DataTypeId::Int => xplm_sys::xplmType_Int as _,
            DataTypeId::Float => xplm_sys::xplmType_Float as _,
            DataTypeId::Double => xplm_sys::xplmType_Double as _,
            DataTypeId::FloatArray => xplm_sys::xplmType_FloatArray as _,
            DataTypeId::IntArray => xplm_sys::xplmType_IntArray as _,
            DataTypeId::Data => xplm_sys::xplmType_Data as _,
        }
    }
}
