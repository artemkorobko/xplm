pub mod coord;
pub mod error;
pub mod event;
pub mod key;
pub mod mouse;
pub mod rect;
pub mod size;
pub mod window;

use std::ops::{Deref, DerefMut};

pub use self::coord::Coord;
pub use self::error::DisplayError;
pub use self::event::EventState;
pub use self::key::KeyFlags;
pub use self::mouse::{MouseStatus, WheelAxis};
pub use self::rect::Rect;
pub use self::size::Size;
pub use self::window::{WindowHandler, WindowHandlerRecord, WindowLink};

use crate::api::display::window::WindowId;

use super::utilities::VirtualKey;

pub type Result<T> = std::result::Result<T, DisplayError>;

/// This routine creates a new “modern” window.
///
/// # Arguments
/// * `rect` - window rectangle. See [`Rect`] for more details.
/// * `handler` - window events handler. See [`WindowHandler`] for more details.
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
