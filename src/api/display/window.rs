use std::ops::Deref;

use crate::api::utilities::VirtualKey;

use super::{destroy_window, DisplayError, EventState, KeyFlags, MouseStatus, WheelAxis};

/// Window identifier.
pub struct WindowId(xplm_sys::XPLMWindowID);

impl Deref for WindowId {
    type Target = xplm_sys::XPLMWindowID;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

/// Window handler
pub trait WindowHandler: 'static {
    /// A callback to handle 2-D drawing of a window.
    fn draw(&mut self);
    /// A callback for one of three events:
    /// - When the user clicks the mouse button down.
    /// - (optionally) when the user drags the mouse after a down-click, but before the up-click
    /// - When the user releases the down-clicked mouse button.
    fn mouse_click(
        &mut self,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        status: MouseStatus,
    ) -> EventState;
    /// This function is called when a key is pressed or keyboard focus is taken away from your window.
    fn handle_key(&mut self, key: char, virtual_key: VirtualKey, flags: KeyFlags);
    /// Get's called when the mouse is over the plugin window.
    fn handle_cursor(&mut self, x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);
    /// Get's called when one of the mouse wheels is scrolled within the window.
    fn handle_mouse_wheel(
        &mut self,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        wheel_axis: WheelAxis,
        clicks: i32,
    ) -> EventState;
}

/// A link to [`WindowHandler`] for a given window.
pub struct WindowLink(Box<dyn WindowHandler>);

impl WindowLink {
    pub fn new(value: Box<dyn WindowHandler>) -> Self {
        Self(value)
    }
}

impl WindowHandler for WindowLink {
    fn draw(&mut self) {
        self.0.draw();
    }

    fn mouse_click(
        &mut self,
        x: std::os::raw::c_int,
        y: std::os::raw::c_int,
        status: MouseStatus,
    ) -> EventState {
        self.0.mouse_click(x, y, status)
    }

    fn handle_key(&mut self, key: char, virtual_key: VirtualKey, flags: KeyFlags) {
        self.0.handle_key(key, virtual_key, flags);
    }

    fn handle_cursor(&mut self, x: std::os::raw::c_int, y: std::os::raw::c_int) {
        self.0.handle_cursor(x, y);
    }

    fn handle_mouse_wheel(
        &mut self,
        x: std::os::raw::c_int,
        y: std::os::raw::c_int,
        wheel_axis: WheelAxis,
        clicks: i32,
    ) -> EventState {
        self.0.handle_mouse_wheel(x, y, wheel_axis, clicks)
    }
}

/// A window handler record to keep a window alive.
pub struct WindowHandlerRecord {
    id: WindowId,
    _link: Box<WindowLink>,
}

impl WindowHandlerRecord {
    pub fn new(id: WindowId, link: Box<WindowLink>) -> Self {
        Self { id, _link: link }
    }
}

impl Drop for WindowHandlerRecord {
    fn drop(&mut self) {
        destroy_window(&self.id)
    }
}
