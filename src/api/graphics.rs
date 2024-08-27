pub mod error;
pub mod font;
pub mod position;
pub mod state;

use std::ffi;

pub use error::GraphicsError;
pub use font::{Font, FontDimensions};
pub use position::{LocalPosition, WorldPosition};
pub use state::GraphicsState;

use super::display::{Color, Coord, Rect};

pub type Result<T> = std::result::Result<T, GraphicsError>;

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

/// Draws a translucent dark box, partially obscuring parts of the screen
/// but making text easy to read. This is the same graphics primitive used
/// by X-Plane to show text files.
///
/// # Arguments
/// * `rect` - a translucent box rectangle. See [`Rect`] for more details.
pub fn draw_translucent_dark_box(rect: &Rect) {
    unsafe { xplm_sys::XPLMDrawTranslucentDarkBox(rect.left, rect.top, rect.right, rect.bottom) };
}

/// Draws a string in a given font.
///
/// # Arguments
/// * `value` - a string to draw.
/// * `font` - a font to use.
/// * `color` - a color to use.
/// * `coord` - a coordinate to use.
///
/// # Returns
/// Returns `Ok(())` if successful, or an [`GraphicsError`] otherwise.
pub fn draw_string<T: Into<String>>(
    value: T,
    font: Font,
    color: &Color,
    coord: &Coord,
) -> Result<()> {
    let value_c = ffi::CString::new(value.into()).map_err(GraphicsError::InvalidString)?;
    let mut xplm_color = [color.r, color.g, color.b];
    unsafe {
        xplm_sys::XPLMDrawString(
            xplm_color.as_mut_ptr(),
            coord.x,
            coord.y,
            value_c.as_ptr() as _,
            core::ptr::null_mut(),
            font.into(),
        )
    };

    Ok(())
}

/// Draws a number similar to the digit editing fields in PlaneMaker and data output display in X-Plane.
///
/// # Arguments
/// * `value` - a number to draw.
/// * `font` - a font to use.
/// * `color` - a color to use.
/// * `coord` - a coordinate to use.
/// * `digits` - the number of digits to display.
/// * `decimals` - the number of decimal places to display.
pub fn draw_number_with_digits(
    value: f64,
    font: Font,
    color: &Color,
    coord: &Coord,
    digits: u8,
    decimals: u8,
) {
    let mut xplm_color = [color.r, color.g, color.b];
    unsafe {
        xplm_sys::XPLMDrawNumber(
            xplm_color.as_mut_ptr(),
            coord.x,
            coord.y,
            value,
            digits as _,
            decimals as _,
            1,
            font.into(),
        )
    };
}

/// Returns the width and height of a character in a given font.
/// It also tells if the font only supports numeric digits.
///
/// #Arguments
/// * `font` - a font to determine dimensions on.
///
/// # Returns
/// Returns tuple representing font dimensions and a boolean indicating if the font only supports numeric digits.
pub fn get_font_dimensions(font: Font) -> (FontDimensions, bool) {
    let mut width = 0;
    let mut height = 0;
    let mut digits = 0;
    unsafe { xplm_sys::XPLMGetFontDimensions(font.into(), &mut width, &mut height, &mut digits) };
    (FontDimensions { width, height }, digits != 0)
}

/// Returns the height in pixels of a string using a given font.
///
/// #Arguments
/// * `value` - a string to measure.
/// * `font` - a font to use.
/// * `len` - the length of the string.
///
/// # Returns
/// Returns the height of the string in pixels if successful, or an [`GraphicsError`] otherwise.`
pub fn measure_string<T: Into<String>>(value: T, font: Font, len: usize) -> Result<f32> {
    let value_c = ffi::CString::new(value.into()).map_err(GraphicsError::InvalidString)?;
    Ok(unsafe { xplm_sys::XPLMMeasureString(font.into(), value_c.as_ptr() as _, len as _) })
}
