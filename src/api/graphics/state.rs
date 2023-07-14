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
