/// Types of data files you can load or unload using the SDK.
#[repr(u32)]
pub enum DataFileType {
    /// A situation (.sit) file, which starts off a flight in a given configuration.
    Situation = xplm_sys::xplm_DataFile_Situation,
    /// A situation movie (.smo) file, which replays a past flight.
    ReplayMovie = xplm_sys::xplm_DataFile_ReplayMovie,
}
