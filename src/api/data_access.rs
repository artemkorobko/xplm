pub mod data_ref;
pub mod data_refs;
pub mod data_type;
pub mod error;

use std::ffi;
use std::ops::Deref;

pub use self::data_ref::DataRef;
pub use self::data_ref::DataRefInfo;
pub use self::data_ref::Info;
pub use self::data_refs::DataRefsIter;
pub use self::data_type::DataType;
pub use self::data_type::DataTypeId;
pub use self::error::DataAccessError;

pub type Result<T> = std::result::Result<T, DataAccessError>;

/// Returns the total number of datarefs that have been registered in X-Plane.
pub fn count_data_refs() -> usize {
    unsafe { xplm_sys::XPLMCountDataRefs() as _ }
}

/// Returns an array of [`DataRef`] in the given range.
///
/// # Arguments
/// * `from` - an offset from which enumeration starts.
/// * `count` - an amount of data refs to read.
///
/// # Returns
/// Returns and iterator over datarefs starting from an offset.
/// See [`DataRefsIter`] for more details.
pub fn get_data_refs_by_index(from: usize, count: usize) -> Result<DataRefsIter> {
    let data_refs_count = count_data_refs();
    let from = std::cmp::min(data_refs_count, from);
    let count = if (from + count) > data_refs_count {
        data_refs_count - from
    } else {
        count
    };

    let data_refs: *mut xplm_sys::XPLMDataRef = std::ptr::null_mut();
    unsafe { xplm_sys::XPLMGetDataRefsByIndex(from as _, count as _, data_refs) };
    DataRefsIter::try_from(data_refs)
}

/// Returns available information about the dataref.
///
/// # Argument
/// * `data_ref` - a data ref.
///
/// # Returns
/// Returns [`DataRefInfo`] if reading completed successfully. Otherwise returns [`DataAccessError`].
pub fn get_data_ref_info(data_ref: &DataRef) -> Result<DataRefInfo> {
    let mut info_c = xplm_sys::XPLMDataRefInfo_t {
        structSize: std::mem::size_of::<xplm_sys::XPLMDataRefInfo_t>() as _,
        name: std::ptr::null_mut(),
        type_: xplm_sys::xplmType_Unknown as _,
        writable: 0,
        owner: 0,
    };

    unsafe { xplm_sys::XPLMGetDataRefInfo(*data_ref.deref(), &mut info_c) };
    let info = Info::try_from(info_c)?;

    if info_c.writable == 1 {
        Ok(DataRefInfo::ReadWrite(info))
    } else {
        Ok(DataRefInfo::ReadOnly(info))
    }
}

/// Looks up the actual opaque data ref that is used to read and write the data.
///
/// # Arguments
/// * `name` - a data ref name.
///
/// # Returns
/// Returns a [`DataRef`] in case of success. Otherwise returns [`DataAccessError`].
pub fn find_data_ref<T: Into<String>>(name: T) -> Result<DataRef> {
    let name_c = ffi::CString::new(name.into()).map_err(DataAccessError::InvalidDataRefName)?;
    let data_ref = unsafe { xplm_sys::XPLMFindDataRef(name_c.as_ptr()) };
    DataRef::try_from(data_ref)
}

/// Checks wether a data ref can be written to.
///
/// # Arguments
/// * `data_ref` - a data ref.
///
/// # Returns
/// Returns `true` if can write to data ref. Otherwise returns `false`.
pub fn can_write_data_ref(data_ref: &DataRef) -> bool {
    unsafe { xplm_sys::XPLMCanWriteDataRef(*data_ref.deref()) == 1 }
}

/// Check wether a data ref is a valid data ref that is not orphaned.
///
/// # Arguments
/// * `data_ref` - a data ref.
///
/// # Returns
/// Returns `true` if data ref is good and ready to use. Otherwise returns `false`.
pub fn is_data_ref_good(data_ref: &DataRef) -> bool {
    unsafe { xplm_sys::XPLMIsDataRefGood(*data_ref.deref()) == 1 }
}

/// Returns the types of the dataref for accessor use.
///
/// # Arguments
/// * `data_ref` - a data ref.
///
/// # Returns
/// Returns a [`DataTypeId`] for a given data ref.
pub fn get_data_ref_types(data_ref: &DataRef) -> DataTypeId {
    let id = unsafe { xplm_sys::XPLMGetDataRefTypes(*data_ref.deref()) };
    DataTypeId::from(id)
}

/// Reads an integer data ref and return its value.
///
/// # Arguments
/// * `data_ref` - a data ref.
///
/// # Returns
/// Returns data ref value.
pub fn get_data_i(data_ref: &DataRef) -> ::std::os::raw::c_int {
    unsafe { xplm_sys::XPLMGetDatai(*data_ref.deref()) }
}

/// Writes a new value to an integer data ref.
///
/// # Arguments
/// * `data_ref` - a data ref.
/// * `value` - a data ref value.
pub fn set_data_i(data_ref: &DataRef, value: ::std::os::raw::c_int) {
    unsafe { xplm_sys::XPLMSetDatai(*data_ref.deref(), value) }
}

/// Reads an single precision floating point data ref and return its value.
///
/// # Arguments
/// * `data_ref` - a data ref.
///
/// # Returns
/// Returns data ref value.
pub fn get_data_f(data_ref: &DataRef) -> f32 {
    unsafe { xplm_sys::XPLMGetDataf(*data_ref.deref()) }
}

/// Writes a new value to an single precision floating point data ref.
///
/// # Arguments
/// * `data_ref` - a data ref.
/// * `value` - a data ref value.
pub fn set_data_f(data_ref: &DataRef, value: f32) {
    unsafe { xplm_sys::XPLMSetDataf(*data_ref.deref(), value) }
}

/// Reads an double precision floating point data ref and return its value.
///
/// # Arguments
/// * `data_ref` - a data ref.
///
/// # Returns
/// Returns data ref value.
pub fn get_data_d(data_ref: &DataRef) -> f64 {
    unsafe { xplm_sys::XPLMGetDatad(*data_ref.deref()) }
}

/// Writes a new value to an double precision floating point data ref.
///
/// # Arguments
/// * `data_ref` - a data ref.
/// * `value` - a data ref value.
pub fn set_data_d(data_ref: &DataRef, value: f64) {
    unsafe { xplm_sys::XPLMSetDatad(*data_ref.deref(), value) }
}
