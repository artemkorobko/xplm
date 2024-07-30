use std::{ffi, ops::Deref};

use crate::api::plugin::PluginId;

use super::{DataAccessError, DataTypeId};

pub struct ReadOnly;
pub struct ReadWrite;

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

/// Contains information about a single data ref.
pub struct Info {
    pub name: String,
    pub data_type: DataTypeId,
    pub owner: PluginId,
}

impl TryFrom<xplm_sys::XPLMDataRefInfo_t> for Info {
    type Error = DataAccessError;

    fn try_from(value: xplm_sys::XPLMDataRefInfo_t) -> Result<Self, Self::Error> {
        Ok(Self {
            name: unsafe {
                ffi::CStr::from_ptr(value.name)
                    .to_owned()
                    .into_string()
                    .map_err(DataAccessError::InvalidInfoName)
            }?,
            data_type: DataTypeId::from(value.type_),
            owner: PluginId::try_from(value.owner)?,
        })
    }
}

/// Contains information about a single data ref base of access.
pub enum DataRefInfo {
    /// Read only data ref information.
    ReadOnly(Info),
    /// Read/Write data ref information.
    ReadWrite(Info),
}
