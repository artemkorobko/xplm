use super::DataAccessError;

/// Enumeration that defines the type of the data behind a data reference.
pub enum DataType {
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

impl TryFrom<xplm_sys::XPLMDataTypeID> for DataType {
    type Error = DataAccessError;

    fn try_from(value: xplm_sys::XPLMDataTypeID) -> Result<Self, Self::Error> {
        match value as _ {
            xplm_sys::xplmType_Unknown => Ok(DataType::Unknown),
            xplm_sys::xplmType_Int => Ok(DataType::Int),
            xplm_sys::xplmType_Float => Ok(DataType::Float),
            xplm_sys::xplmType_Double => Ok(DataType::Double),
            xplm_sys::xplmType_FloatArray => Ok(DataType::FloatArray),
            xplm_sys::xplmType_IntArray => Ok(DataType::IntArray),
            xplm_sys::xplmType_Data => Ok(DataType::Data),
            _ => Err(Self::Error::UnknownDataTypeId(value)),
        }
    }
}

impl From<DataType> for xplm_sys::XPLMDataTypeID {
    fn from(value: DataType) -> Self {
        match value {
            DataType::Unknown => xplm_sys::xplmType_Unknown as _,
            DataType::Int => xplm_sys::xplmType_Int as _,
            DataType::Float => xplm_sys::xplmType_Float as _,
            DataType::Double => xplm_sys::xplmType_Double as _,
            DataType::FloatArray => xplm_sys::xplmType_FloatArray as _,
            DataType::IntArray => xplm_sys::xplmType_IntArray as _,
            DataType::Data => xplm_sys::xplmType_Data as _,
        }
    }
}

/// Data type bitmap.
pub struct DataTypeId(xplm_sys::XPLMDataTypeID);

impl From<xplm_sys::XPLMDataTypeID> for DataTypeId {
    fn from(value: xplm_sys::XPLMDataTypeID) -> Self {
        Self(value)
    }
}
