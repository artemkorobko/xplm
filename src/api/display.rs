pub mod coord;
pub mod error;
pub mod event;
pub mod gravity;
pub mod key;
pub mod mouse;
pub mod rect;
pub mod size;
pub mod window;

use std::ffi;
use std::ops::{Deref, DerefMut};

pub use self::coord::Coord;
pub use self::error::DisplayError;
pub use self::event::EventState;
use self::gravity::GravityRect;
pub use self::key::KeyFlags;
pub use self::mouse::{MouseStatus, WheelAxis};
pub use self::rect::Rect;
pub use self::size::Size;
pub use self::window::PositioningMode;
pub use self::window::{WindowHandler, WindowHandlerRecord, WindowLink};

use crate::api::display::window::WindowId;

use super::utilities::VirtualKey;

pub type Result<T> = std::result::Result<T, DisplayError>;

/// This routine creates a new “modern” window.
///
/// # Arguments
/// * `rect` - window rectangle.
/// * `handler` - window events handler.
///
/// # Returns
/// Returns [`WindowHandlerRecord`] on success. Otherwise returns [`DisplayError`].
pub fn create_window_ex<H: WindowHandler>(rect: &Rect, handler: H) -> Result<WindowHandlerRecord> {
    unsafe extern "C" fn draw_window(
        _: xplm_sys::XPLMWindowID,
        refcon: *mut ::std::os::raw::c_void,
    ) {
        let link = refcon as *mut WindowLink;
        (*link).draw();
    }

    unsafe extern "C" fn mouse_click(
        _: xplm_sys::XPLMWindowID,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        mouse: xplm_sys::XPLMMouseStatus,
        refcon: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int {
        match MouseStatus::try_from(mouse) {
            Ok(status) => {
                let link = refcon as *mut WindowLink;
                let coord = Coord::default().x(x).y(y);
                (*link).mouse_click(coord, status).into()
            }
            Err(err) => {
                crate::error!("{}", err);
                EventState::Propagate.into()
            }
        }
    }

    unsafe extern "C" fn handle_key(
        _: xplm_sys::XPLMWindowID,
        key: ::std::os::raw::c_char,
        flags: xplm_sys::XPLMKeyFlags,
        virtual_key: ::std::os::raw::c_char,
        refcon: *mut ::std::os::raw::c_void,
        _: ::std::os::raw::c_int,
    ) {
        let link = refcon as *mut WindowLink;
        match VirtualKey::try_from(virtual_key) {
            Ok(virtual_key) => {
                (*link).handle_key(key as u8 as char, virtual_key, KeyFlags::from(flags))
            }
            Err(err) => {
                crate::error!("{}", err);
            }
        }
    }

    unsafe extern "C" fn handle_cursor(
        _: xplm_sys::XPLMWindowID,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        refcon: *mut ::std::os::raw::c_void,
    ) -> xplm_sys::XPLMCursorStatus {
        let link = refcon as *mut WindowLink;
        let coord = Coord::default().x(x).y(y);
        (*link).handle_cursor(coord);
        xplm_sys::xplm_CursorDefault as _
    }

    unsafe extern "C" fn handle_mouse_wheel(
        _: xplm_sys::XPLMWindowID,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        wheel: ::std::os::raw::c_int,
        clicks: ::std::os::raw::c_int,
        refcon: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int {
        let link = refcon as *mut WindowLink;
        match WheelAxis::try_from(wheel) {
            Ok(wheel_axis) => {
                let coord = Coord::default().x(x).y(y);
                (*link).handle_mouse_wheel(coord, wheel_axis, clicks).into()
            }
            Err(err) => {
                crate::error!("{}", err);
                EventState::Propagate.into()
            }
        }
    }

    let mut link = Box::new(WindowLink::new(Box::new(handler)));
    let link_ptr: *mut WindowLink = link.deref_mut();
    let mut params = xplm_sys::XPLMCreateWindow_t {
        structSize: std::mem::size_of::<xplm_sys::XPLMCreateWindow_t>() as _,
        left: rect.left,
        top: rect.top,
        right: rect.right,
        bottom: rect.bottom,
        visible: 0,
        drawWindowFunc: Some(draw_window),
        handleMouseClickFunc: Some(mouse_click),
        handleKeyFunc: Some(handle_key),
        handleCursorFunc: Some(handle_cursor),
        handleMouseWheelFunc: Some(handle_mouse_wheel),
        refcon: link_ptr as _,
        decorateAsFloatingWindow: 0,
        layer: xplm_sys::xplm_WindowLayerFloatingWindows as _,
        handleRightClickFunc: Some(mouse_click),
    };

    let id = unsafe { xplm_sys::XPLMCreateWindowEx(&mut params) };
    Ok(WindowHandlerRecord::new(WindowId::try_from(id)?, link))
}

/// Destroys a window.
///
/// # Arguments
/// * `id` - a window identifier. See [`WindowId`] for more details.
pub fn destroy_window(id: &WindowId) {
    unsafe { xplm_sys::XPLMDestroyWindow(*id.deref()) };
}

/// Returns the size of the main X-Plane OpenGL window in pixels.
/// This number can be used to get a rough idea of the amount of
/// detail the user will be able to see when drawing in 3D.
///
/// # Returns
/// Returns the size of the main X-Plane OpenGL window.
pub fn get_screen_size() -> Size {
    let mut width = 0;
    let mut height = 0;
    unsafe { xplm_sys::XPLMGetScreenSize(&mut width, &mut height) };
    Size::default().width(width).height(height)
}

/// Returns the bounds of the “global” X-Plane desktop, in boxels.
/// Unlike the non-global version [`get_screen_size`], this is multi-monitor aware.
///
/// # Returns
/// Returns the bounds of the “global” X-Plane desktop.
pub fn get_screen_bounds_global() -> Rect {
    let mut left = 0;
    let mut top = 0;
    let mut right = 0;
    let mut bottom = 0;
    unsafe { xplm_sys::XPLMGetScreenBoundsGlobal(&mut left, &mut top, &mut right, &mut bottom) };
    Rect::default()
        .left(left)
        .top(top)
        .right(right)
        .bottom(bottom)
}

/// Returns the current mouse location in global desktop boxels.
///
/// # Returns
/// Returns mouse locatiopn coordinates.
pub fn get_mouse_location_global() -> Coord {
    let mut x = 0;
    let mut y = 0;
    unsafe { xplm_sys::XPLMGetMouseLocationGlobal(&mut x, &mut y) };
    Coord::default().x(x).y(y)
}

/// Returns the position and size of a window.
/// The units and coordinate system vary depending on the type of window you have.
///
/// If this is a legacy window (one compiled against a pre-XPLM300 version of the SDK,
/// or an XPLM300 window that was not created using [`create_window_ex`]), the units are
/// pixels relative to the main X-Plane display.
///
/// If, on the other hand, this is a new X-Plane 11-style window (compiled against the
/// XPLM300 SDK and created using [`create_window_ex`]), the units are global desktop boxels.
///
/// # Arguments
/// * `id` - a window identifier.
///
/// # Returns
/// Returns the bounding rect on a window.
pub fn get_window_geometry(id: &WindowId) -> Rect {
    let mut left = 0;
    let mut top = 0;
    let mut right = 0;
    let mut bottom = 0;
    unsafe {
        xplm_sys::XPLMGetWindowGeometry(*id.deref(), &mut left, &mut top, &mut right, &mut bottom)
    };
    Rect::default()
        .left(left)
        .top(top)
        .right(right)
        .bottom(bottom)
}

/// Set the position and size of a window.
/// The units and coordinate system match those of [`get_window_geometry`].
/// That is, modern windows use global desktop boxel coordinates,
/// while legacy windows use pixels relative to the main X-Plane display.
///
/// Note that this only applies to “floating” windows (that is, windows that are
/// drawn within the X-Plane simulation windows, rather than being “popped out”
/// into their own first-class operating system windows).
///
/// # Arguments
/// * `id` - a window identifier.
/// * `rect` - a bounding box rect of a window.
pub fn set_window_geometry(id: &WindowId, rect: &Rect) {
    unsafe {
        xplm_sys::XPLMSetWindowGeometry(*id.deref(), rect.left, rect.top, rect.right, rect.bottom)
    };
}

/// This routine returns the position and size of a “popped out” window,
/// in operating system pixels.
///
/// # Arguments
/// * `id` - a window identifier.
///
/// # Returns
/// Returns the bounding rect on a window.
pub fn get_window_geometry_os(id: &WindowId) -> Rect {
    let mut left = 0;
    let mut top = 0;
    let mut right = 0;
    let mut bottom = 0;
    unsafe {
        xplm_sys::XPLMGetWindowGeometryOS(*id.deref(), &mut left, &mut top, &mut right, &mut bottom)
    };
    Rect::default()
        .left(left)
        .top(top)
        .right(right)
        .bottom(bottom)
}

/// Set the position and size of a window in operating system pixel coordinates.
///
/// # Arguments
/// * `id` - a window identifier.
/// * `rect` - a bounding box rect of a window.
pub fn set_window_geometry_os(id: &WindowId, rect: &Rect) {
    unsafe {
        xplm_sys::XPLMSetWindowGeometryOS(*id.deref(), rect.left, rect.top, rect.right, rect.bottom)
    };
}

/// Check whether a specified window is visible or not.
///
/// # Arguments
/// * `id` - a window identifier
///
/// # Returns
/// Returns `true` if window is visible. Otherwise returns false.
pub fn get_window_is_visible(id: &WindowId) -> bool {
    unsafe { xplm_sys::XPLMGetWindowIsVisible(*id.deref()) == 1 }
}

/// Sets a window visible.
///
/// # Arguments
/// * `id` - a window identifier
pub fn set_window_visible(id: &WindowId) {
    unsafe { xplm_sys::XPLMSetWindowIsVisible(*id.deref(), 1) };
}

/// Sets a window hidden.
///
/// # Arguments
/// * `id` - a window identifier
pub fn set_window_hidden(id: &WindowId) {
    unsafe { xplm_sys::XPLMSetWindowIsVisible(*id.deref(), 0) };
}

/// Checks wether a window is poppet-out.
///
/// # Arguments
/// * `id` - a window identifier
///
/// # Returns
/// Returns `true` is window is popped-out. Otherwise returns `false`.
pub fn is_window_popped_out(id: &WindowId) -> bool {
    unsafe { xplm_sys::XPLMWindowIsPoppedOut(*id.deref()) == 1 }
}

/// A window's “gravity” controls how the window shifts as the whole X-Plane window resizes.
/// A gravity of 1 means the window maintains its positioning relative to the right or top edges,
/// 0 the left/bottom, and 0.5 keeps it centered.
///
/// Default gravity is (0, 1, 0, 1), meaning your window will maintain its position relative to the
/// top left and will not change size as its containing window grows.
///
/// # Arguments
/// * `id` - a window identifier.
/// * `rect` - a gravity options.
pub fn set_window_gravity(id: &WindowId, rect: &GravityRect) {
    unsafe {
        xplm_sys::XPLMSetWindowGravity(*id.deref(), rect.left, rect.top, rect.right, rect.bottom)
    }
}

/// Sets the minimum and maximum size of the client rectangle of the given window.
///
/// # Arguments
/// * `id` - a window identifier.
/// * `min` - a minimum size of a window.
/// * `max` - a maximum size of a window.
pub fn set_window_resizing_limits(id: &WindowId, min: &Size, max: &Size) {
    unsafe {
        xplm_sys::XPLMSetWindowResizingLimits(
            *id.deref(),
            min.width,
            min.height,
            max.width,
            max.height,
        )
    };
}

/// Sets the policy for how X-Plane will position your window.
/// Some positioning modes apply to a particular monitor.
/// For those modes, you can pass a negative monitor index to position
/// the window on the main X-Plane monitor (the screen with the X-Plane menu bar at the top).
///
/// # Arguments
/// * `id` - a window identifier.
/// * `mode` - a positioning mode.
/// * `monitor` - a monitor index. Specify 0 for default monitor.
pub fn set_window_positioning_mode(
    id: &WindowId,
    mode: PositioningMode,
    monitor: ::std::os::raw::c_int,
) {
    unsafe { xplm_sys::XPLMSetWindowPositioningMode(*id.deref(), mode.into(), monitor) };
}

/// Sets the title for a window.
/// This only applies to windows that opted-in to styling as an X-Plane 11 floating window.
///
/// # Arguments
/// * `id` - a window identifier.
/// * `title` - a window title.
///
/// # Returns
/// Returns empty result on success. Otherwise returns [`DisplayError`].
pub fn set_window_title<T: Into<String>>(id: &WindowId, title: T) -> Result<()> {
    let title_c = ffi::CString::new(title.into()).map_err(DisplayError::InvalidWindowTitle)?;
    unsafe { xplm_sys::XPLMSetWindowTitle(*id.deref(), title_c.as_ptr()) };
    Ok(())
}

/// Gives a specific window keyboard focus.
/// Keystrokes will be sent to that window.
///
/// # Arguments
/// * `id` - a window identifier.
pub fn take_keyboard_focus(id: &WindowId) {
    unsafe { xplm_sys::XPLMTakeKeyboardFocus(*id.deref()) };
}

/// Removes keyboard focus from any plugin-created windows and
/// instead pass keyboard strokes directly to X-Plane.
pub fn remove_keyboard_focus() {
    unsafe { xplm_sys::XPLMTakeKeyboardFocus(0 as xplm_sys::XPLMWindowID) };
}

/// Check wether a specified window has focus or not.
///
/// # Arguments
/// * `id` - a window identifier.
///
/// # Returns
/// Return `true` is specified window has focus. Otherwise returns `false`.
pub fn has_keyboard_focus(id: &WindowId) -> bool {
    unsafe { xplm_sys::XPLMHasKeyboardFocus(*id.deref()) == 1 }
}

/// Brings the window to the front of the Z-order for its layer.
/// Windows are brought to the front automatically when they are created.
/// Beyond that, you should make sure you are front before handling mouse clicks.
///
/// # Arguments
/// * `id` - a window identifier.
pub fn bring_window_to_front(id: &WindowId) {
    unsafe { xplm_sys::XPLMBringWindowToFront(*id.deref()) };
}

/// Check wether a given window in front or not.
///
/// # Arguments
/// * `id` - a window identifier.
///
/// # Returns
/// Returns `true` if specified window is in front. Otherwise returns `false`.
pub fn is_window_in_front(id: &WindowId) -> bool {
    unsafe { xplm_sys::XPLMIsWindowInFront(*id.deref()) == 1 }
}
