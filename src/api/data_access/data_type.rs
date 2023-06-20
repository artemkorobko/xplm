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

/// Data type flags bitmap.
pub struct DataTypeId(xplm_sys::XPLMDataTypeID);

impl DataTypeId {
    /// Checks whether the flags bitmap contains a specific flag.
    ///
    /// # Arguments
    /// * `flag` - a flag to check.
    ///
    /// # Returns
    /// Return `true` if flags contains specific flag. Otherwise returns `false`.
    pub fn contains(&self, data_type: DataType) -> bool {
        match data_type {
            DataType::Unknown => self.is_unknown_type(),
            DataType::Int => self.is_int_type(),
            DataType::Float => self.is_float_type(),
            DataType::Double => self.is_double_type(),
            DataType::FloatArray => self.is_float_array_type(),
            DataType::IntArray => self.is_int_array_type(),
            DataType::Data => self.is_data_type(),
        }
    }

    /// Checks whether the flags bitmap contains unknown type flag.
    ///
    /// # Returns
    /// Return `true` if flags contains unknown type. Otherwise returns `false`.
    pub fn is_unknown_type(&self) -> bool {
        self.0 == xplm_sys::xplmType_Unknown as _
    }

    /// Checks whether the flags bitmap contains int type flag.
    ///
    /// # Returns
    /// Return `true` if flags contains int type. Otherwise returns `false`.
    pub fn is_int_type(&self) -> bool {
        self.0 & (xplm_sys::xplmType_Int as xplm_sys::XPLMDataTypeID) != 0
    }

    /// Checks whether the flags bitmap contains float type flag.
    ///
    /// # Returns
    /// Return `true` if flags contains float type. Otherwise returns `false`.
    pub fn is_float_type(&self) -> bool {
        self.0 & (xplm_sys::xplmType_Float as xplm_sys::XPLMDataTypeID) != 0
    }

    /// Checks whether the flags bitmap contains double type flag.
    ///
    /// # Returns
    /// Return `true` if flags contains double type. Otherwise returns `false`.
    pub fn is_double_type(&self) -> bool {
        self.0 & (xplm_sys::xplmType_Double as xplm_sys::XPLMDataTypeID) != 0
    }

    /// Checks whether the flags bitmap contains float array type flag.
    ///
    /// # Returns
    /// Return `true` if flags contains float array type. Otherwise returns `false`.
    pub fn is_float_array_type(&self) -> bool {
        self.0 & (xplm_sys::xplmType_FloatArray as xplm_sys::XPLMDataTypeID) != 0
    }

    /// Checks whether the flags bitmap contains int array type flag.
    ///
    /// # Returns
    /// Return `true` if flags contains int array type. Otherwise returns `false`.
    pub fn is_int_array_type(&self) -> bool {
        self.0 & (xplm_sys::xplmType_IntArray as xplm_sys::XPLMDataTypeID) != 0
    }

    /// Checks whether the flags bitmap contains data type flag.
    ///
    /// # Returns
    /// Return `true` if flags contains data type. Otherwise returns `false`.
    pub fn is_data_type(&self) -> bool {
        self.0 & (xplm_sys::xplmType_Data as xplm_sys::XPLMDataTypeID) != 0
    }
}

impl From<xplm_sys::XPLMDataTypeID> for DataTypeId {
    fn from(value: xplm_sys::XPLMDataTypeID) -> Self {
        Self(value)
    }
}
