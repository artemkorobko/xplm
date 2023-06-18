use std::{ffi, ops::DerefMut};

use super::utilities::VirtualKey;

/// An error returned from display API calls
#[derive(thiserror::Error, Debug)]
pub enum DisplayError {
    /// Invalid window id returned from X-Plane
    #[error("invalid window id")]
    InvalidWindowId,
    /// Invalid command name string passed to X-Plane
    #[error("invalid command name {0}")]
    InvalidCommandName(ffi::NulError),
    /// Unknown mouse status passed from X-Plane
    #[error("unknown mouse status {0}")]
    UnknownMouseStatuts(xplm_sys::XPLMMouseStatus),
    /// Unknown mouse wheel axis passed from X-Plane
    #[error("unknown mouse wheel axis {0}")]
    UnknownMouseWheelAxis(::std::os::raw::c_int),
}

pub type Result<T> = std::result::Result<T, DisplayError>;

/// Window identifier.
pub struct WindowId(xplm_sys::XPLMWindowID);

impl TryFrom<xplm_sys::XPLMWindowID> for WindowId {
    type Error = DisplayError;

    fn try_from(value: xplm_sys::XPLMWindowID) -> std::result::Result<Self, Self::Error> {
        if value.is_null() {
            Err(Self::Error::InvalidWindowId)
        } else {
            Ok(WindowId(value))
        }
    }
}

/// Mouse status passed in [`WindowHandler::mouse_click`].
pub enum MouseStatus {
    Up,
    Down,
    Drag,
}

impl TryFrom<xplm_sys::XPLMMouseStatus> for MouseStatus {
    type Error = DisplayError;

    fn try_from(value: xplm_sys::XPLMMouseStatus) -> std::result::Result<Self, Self::Error> {
        match value as _ {
            xplm_sys::xplm_MouseUp => Ok(Self::Up),
            xplm_sys::xplm_MouseDown => Ok(Self::Down),
            xplm_sys::xplm_MouseDrag => Ok(Self::Drag),
            _ => Err(Self::Error::UnknownMouseStatuts(value)),
        }
    }
}

/// Result returned from [`WindowHandler::mouse_click`] function.
pub enum EventStatus {
    /// Consume click.
    Consume,
    /// Propagate click to other consumers.
    Propagate,
}

impl From<EventStatus> for ::std::os::raw::c_int {
    fn from(value: EventStatus) -> Self {
        match value {
            EventStatus::Consume => 1,
            EventStatus::Propagate => 0,
        }
    }
}

/// The mouse wheel axis.
pub enum WheelAxis {
    Vertical,
    Horizontal,
}

impl TryFrom<::std::os::raw::c_int> for WheelAxis {
    type Error = DisplayError;

    fn try_from(value: ::std::os::raw::c_int) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Vertical),
            1 => Ok(Self::Horizontal),
            _ => Err(Self::Error::UnknownMouseWheelAxis(value)),
        }
    }
}

/// Modifier keys.
pub enum KeyFlag {
    Shift,
    OptionAlt,
    Control,
    Down,
    Up,
}

/// Key flags bitmap.
pub struct KeyFlags(xplm_sys::XPLMKeyFlags);

impl KeyFlags {
    pub fn contains(&self, flag: KeyFlag) -> bool {
        match flag {
            KeyFlag::Shift => (self.0 & xplm_sys::xplm_ShiftFlag as xplm_sys::XPLMKeyFlags) != 0,
            KeyFlag::OptionAlt => {
                (self.0 & xplm_sys::xplm_OptionAltFlag as xplm_sys::XPLMKeyFlags) != 0
            }
            KeyFlag::Control => {
                self.0 & (xplm_sys::xplm_ControlFlag as xplm_sys::XPLMKeyFlags) != 0
            }
            KeyFlag::Down => self.0 & (xplm_sys::xplm_DownFlag as xplm_sys::XPLMKeyFlags) != 0,
            KeyFlag::Up => self.0 & (xplm_sys::xplm_UpFlag as xplm_sys::XPLMKeyFlags) != 0,
        }
    }
}

/// Window handler
pub trait WindowHandler: 'static {
    /// A callback to handle 2-D drawing of a window.
    fn draw(&mut self);
    /// A callback for one of three events:
    /// - When the user clicks the mouse button down.
    /// - (optionally) when the user drags the mouse after a down-click, but before the up-click
    /// - When the user releases the down-clicked mouse button.
    fn mouse_click(&mut self, x: i32, y: i32, status: MouseStatus) -> EventStatus;
    /// This function is called when a key is pressed or keyboard focus is taken away from your window.
    fn handle_key(&mut self, key: char, virtual_key: VirtualKey, flags: KeyFlags);
    /// Get's called when the mouse is over the plugin window.
    fn handle_cursor(&mut self, x: i32, y: i32);
    /// Get's called when one of the mouse wheels is scrolled within the window.
    fn handle_mouse_wheel(
        &mut self,
        x: i32,
        y: i32,
        wheel_axis: WheelAxis,
        clicks: i32,
    ) -> EventStatus;
}

/// A link to [`WindowHandler`] for a given window.
pub struct WindowLink {
    handler: Box<dyn WindowHandler>,
}

/// A window handler record to keep a window alive.
pub struct WindowHandlerRecord {
    id: WindowId,
    _link: Box<WindowLink>,
}

impl Drop for WindowHandlerRecord {
    fn drop(&mut self) {
        destroy_window(&self.id)
    }
}

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
pub fn create_window_ex<H: WindowHandler>(
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
    handler: H,
) -> Result<WindowHandlerRecord> {
    unsafe extern "C" fn draw_window(
        _: xplm_sys::XPLMWindowID,
        refcon: *mut ::std::os::raw::c_void,
    ) {
        let link = refcon as *mut WindowLink;
        (*link).handler.draw();
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
                (*link).handler.mouse_click(x, y, status).into()
            }
            Err(err) => {
                crate::error!("{}", err);
                EventStatus::Propagate.into()
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
                (*link)
                    .handler
                    .handle_key(key as u8 as char, virtual_key, KeyFlags(flags))
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
        (*link).handler.handle_cursor(x, y);
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
            Ok(wheel_axis) => (*link)
                .handler
                .handle_mouse_wheel(x, y, wheel_axis, clicks)
                .into(),
            Err(err) => {
                crate::error!("{}", err);
                EventStatus::Propagate.into()
            }
        }
    }

    let mut link = Box::new(WindowLink {
        handler: Box::new(handler),
    });

    let link_ptr: *mut WindowLink = link.deref_mut();
    let mut params = xplm_sys::XPLMCreateWindow_t {
        structSize: std::mem::size_of::<xplm_sys::XPLMCreateWindow_t>() as _,
        left: left as _,
        top: top as _,
        right: right as _,
        bottom: bottom as _,
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

    Ok(WindowHandlerRecord {
        id: WindowId::try_from(id)?,
        _link: link,
    })
}

/// Destroys a window.
///
/// # Arguments
/// * `id` - a window identifier.
pub fn destroy_window(id: &WindowId) {
    unsafe { xplm_sys::XPLMDestroyWindow(id.0) };
}
