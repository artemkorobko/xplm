pub mod coord;
pub mod error;
pub mod event;
pub mod key;
pub mod mouse;
pub mod rect;
pub mod window;

use std::ops::{Deref, DerefMut};

pub use coord::Coord;
pub use error::DisplayError;
pub use event::EventState;
pub use key::KeyFlags;
pub use mouse::{MouseStatus, WheelAxis};
pub use rect::Rect;
pub use window::{WindowHandler, WindowHandlerRecord, WindowLink};

use crate::api::display::window::WindowId;

use super::utilities::VirtualKey;

pub type Result<T> = std::result::Result<T, DisplayError>;

/// This routine creates a new “modern” window.
///
/// # Arguments
/// * `left` - left position of the window in pixels.
/// * `top` - top position of the window in pixels.
/// * `right` - right position of the window in pixels.
/// * `bottom` - bottom position of the window in pixels.
/// * `hanndler` - window events handler.
///
/// # Returns
/// Returns [`WindowHandlerRecord`] on success. Otherwise returns [`DisplayError::InvalidWindowId`];
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
/// * `id` - a window identifier.
pub fn destroy_window(id: &WindowId) {
    unsafe { xplm_sys::XPLMDestroyWindow(*id.deref()) };
}
