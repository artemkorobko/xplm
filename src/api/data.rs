pub mod data_ref;
pub mod data_refs;
pub mod error;

pub use self::data_ref::DataRef;
pub use self::data_refs::DataRefsIter;
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
