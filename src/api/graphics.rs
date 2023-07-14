pub mod position;
pub mod state;

pub use position::{LocalPosition, WorldPosition};
pub use state::GraphicsState;

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

/// Translates coordinates from latitude, longitude, and altitude to local scene coordinates.
/// Latitude and longitude are in decimal degrees, and altitude is in meters MSL (mean sea level).
/// The XYZ coordinates are in meters in the local OpenGL coordinate system.
///
/// # Arguments
/// * `world` - a world position. See [`WorldPosition`] for more details.
///
/// # Returns
/// Returns a local position. See [`LocalPosition`] for more details.
pub fn world_to_local(world: &WorldPosition) -> LocalPosition {
    let mut local = LocalPosition::default();
    unsafe {
        xplm_sys::XPLMWorldToLocal(
            world.latitude,
            world.longitude,
            world.altitude,
            &mut local.x,
            &mut local.y,
            &mut local.z,
        )
    };
    local
}

/// Translates a local coordinate triplet back into latitude, longitude, and altitude.
/// Latitude and longitude are in decimal degrees, and altitude is in meters MSL (mean sea level).
/// The XYZ coordinates are in meters in the local OpenGL coordinate system.
///
/// NOTE: world coordinates are less precise than local coordinates.
/// You should try to avoid round tripping from local to world and back.
///
/// # Arguments
/// * `local` - a local position. See [`LocalPosition`] for more details.
///
/// # Returns
/// Returns a world position. See [`WorldPosition`] for more details.
pub fn local_to_world(local: &LocalPosition) -> WorldPosition {
    let mut world = WorldPosition::default();
    unsafe {
        xplm_sys::XPLMLocalToWorld(
            local.x,
            local.y,
            local.z,
            &mut world.latitude,
            &mut world.longitude,
            &mut world.altitude,
        )
    };
    world
}
