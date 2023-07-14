/// Graphics state configuration used in [`set_graphics_state`].
pub struct GraphicsState {
    /// Enables or disables fog, equivalent to: glEnable(GL_FOG).
    pub enable_fog: ::std::os::raw::c_int,
    /// Enables or disables a number of multitexturing units.
    /// If the number is 0, 2d texturing is disabled entirely, as in glDisable(GL_TEXTURE_2D).
    /// Otherwise, 2d texturing is enabled.
    pub number_tex_units: ::std::os::raw::c_int,
    /// Enables or disables OpenGL lighting, e.g. glEnable(GL_LIGHTING).
    pub enable_lighting: ::std::os::raw::c_int,
    /// Enables or disables the alpha test per pixel.
    pub enable_alpha_testing: ::std::os::raw::c_int,
    /// Enables or disables alpha blending per pixel, e.g. glEnable(GL_BLEND).
    pub enable_alpha_blending: ::std::os::raw::c_int,
    /// Enables per pixel depth testing, as in glEnable(GL_DEPTH_TEST).
    pub enable_depth_testing: ::std::os::raw::c_int,
    /// Enables writing back of depth information to the depth buffer, as in glDepthMask(GL_TRUE).
    pub enable_depth_writing: ::std::os::raw::c_int,
}

impl GraphicsState {
    pub fn ui() -> Self {
        Self {
            enable_fog: 0,
            number_tex_units: 0,
            enable_lighting: 0,
            enable_alpha_testing: 0,
            enable_alpha_blending: 0,
            enable_depth_testing: 0,
            enable_depth_writing: 0,
        }
    }
}

/// Changes OpenGL’s fixed function pipeline state.
///
/// # Arguments
/// * `state` - a [`GraphicsState`] properties struct.
pub fn set_graphics_state(state: &GraphicsState) {
    unsafe {
        xplm_sys::XPLMSetGraphicsState(
            state.enable_fog,
            state.number_tex_units,
            state.enable_lighting,
            state.enable_alpha_testing,
            state.enable_alpha_blending,
            state.enable_depth_testing,
            state.enable_depth_writing,
        )
    };
}

/// Changes what texture is bound to the 2d texturing target.
///
/// This routine caches the current 2d texture across all texturing units in the sim
/// and plug-ins, preventing extraneous binding. For example, consider several plug-ins
/// running in series. If they all use the ‘general interface’ bitmap to do UI, calling
/// this function will skip the rebinding of the general interface texture on all but the
/// first plug-in, which can provide better frame rates on some graphics cards.
///
/// # Arguments
/// * `num` - is the ID of the texture object to bind.
/// * `unit` - is a zero-based texture unit (e.g. 0 for the first one), up to a maximum of 4 units.
pub fn bind_texture_2d(num: ::std::os::raw::c_int, unit: ::std::os::raw::c_int) {
    unsafe { xplm_sys::XPLMBindTexture2d(num, unit) };
}
