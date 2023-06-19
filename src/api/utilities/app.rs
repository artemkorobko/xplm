use super::UtilitiesError;

/// While the plug-in SDK is only accessible to plugins running inside X-Plane,
/// the original authors considered extending the API to other applications that
/// shared basic infrastructure with X-Plane. These enumerations are hold-overs
/// from that original roadmap; all values other than X-Plane are deprecated.
/// Your plugin should never need this enumeration.
pub enum HostApplicationId {
    /// Unknown application identifier.
    Unknown,
    /// Xpplication is X-Plane.
    XPlane,
}

impl TryFrom<xplm_sys::XPLMHostApplicationID> for HostApplicationId {
    type Error = UtilitiesError;

    fn try_from(value: xplm_sys::XPLMHostApplicationID) -> std::result::Result<Self, Self::Error> {
        match value as ::std::os::raw::c_uint {
            xplm_sys::xplm_Host_Unknown => Ok(Self::Unknown),
            xplm_sys::xplm_Host_XPlane => Ok(Self::XPlane),
            _ => Err(Self::Error::UnknownHostApplicationId(value)),
        }
    }
}

/// X-Plane and XPLM versions.
pub struct Versions {
    /// Host ID of the app running the plugin.
    pub app_id: HostApplicationId,
    /// X-Plane version.
    pub xplane: i32,
    /// XPLM version.
    pub xplm: i32,
}
