use crate::api::utilities::VirtualKey;

use super::{destroy_window, Coord, DisplayError, EventState, KeyFlags, MouseStatus, WheelAxis};

/// X-Plane window identifier.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WindowId(xplm_sys::XPLMWindowID);

impl WindowId {
    /// Returns the X-Plane window ID as a raw pointer.
    pub fn native(&self) -> xplm_sys::XPLMWindowID {
        self.0
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

/// Window handler trait.
pub trait WindowHandler: 'static {
    /// A callback to handle 2-D drawing of a window.
    fn draw(&mut self, id: &WindowId);

    /// A callback for one of three events:
    /// - When the user clicks the mouse button down.
    /// - (optionally) when the user drags the mouse after a down-click, but before the up-click.
    /// - When the user releases the down-clicked mouse button.
    ///
    /// # Arguments
    /// * `coord` - coordinates at which mouse event occured.
    /// * `status` - the mouse status.
    ///
    /// # Returns
    /// Returns an event state telling X-Plane what to do with this event.
    fn mouse_click(&mut self, id: &WindowId, coord: Coord, status: MouseStatus) -> EventState;

    /// This function is called when a key is pressed or keyboard focus is taken away from your window.
    ///
    /// # Arguments
    /// * `key` - the key character which has been pressed or released.
    /// * `virtual_key` - the virtual key which has been pressed or released.
    /// * `flags` - the key flags bitmap which contains state for special keys and whether the key
    ///     has been pressed or released.
    fn handle_key(&mut self, id: &WindowId, key: char, virtual_key: VirtualKey, flags: KeyFlags);

    /// Get's called when the mouse is over the plugin window.
    ///
    /// # Arguments
    /// * `coord` - coordinates at which cursor event occured.
    fn handle_cursor(&mut self, id: &WindowId, coord: Coord);

    /// Get's called when one of the mouse wheels is scrolled within the window.
    ///
    /// # Arguments
    /// * `coord` - coordinates at which mouse event occured.
    /// * `wheel_axis` - the direction of wheel axis.
    /// * `clicks` - number of clicks wheel performed after the last event.
    ///
    /// # Returns
    /// Returns an event state telling X-Plane what to do with this event.
    fn handle_mouse_wheel(
        &mut self,
        id: &WindowId,
        coord: Coord,
        wheel_axis: WheelAxis,
        clicks: i32,
    ) -> EventState;
}

/// A window.
pub struct Window {
    /// A window identifier.
    pub id: WindowId,
    /// A window handler.
    pub handler: Box<dyn WindowHandler>,
}

impl Window {
    /// Creates a new window instance.
    ///
    /// # Arguments
    /// * `id` - the window identifier.
    /// * `handler` - a pointer to the window handler.
    ///
    /// # Return
    /// Return the new window instance.
    pub fn new(id: WindowId, handler: Box<dyn WindowHandler>) -> Self {
        Self { id, handler }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        destroy_window(&self.id)
    }
}

/// A window positioning mode.
pub enum WindowPositioningMode {
    /// The default positioning mode. Set the window geometry and its
    /// future position will be determined by its window gravity,
    /// resizing limits, and user interactions.
    Free = 0,
    /// Keep the window centered on the monitor you specify.
    CenterOnMonitor = 1,
    /// Keep the window full screen on the monitor you specify.
    FullScreenOnMonitor = 2,
    /// Like `FullScreenOnMonitor`, but stretches over *all* monitors and popout windows.
    /// This is an obscure one... unless you have a very good reason to need it, you probably don't!
    FullScreenOnAllMonitors = 3,
    /// A first-class window in the operating system, completely separate from the X-Plane window(s).
    WindowPopOut = 4,
    /// A floating window visible on the VR headset.
    WindowVR = 5,
}

impl From<WindowPositioningMode> for xplm_sys::XPLMWindowPositioningMode {
    fn from(value: WindowPositioningMode) -> Self {
        value as xplm_sys::XPLMWindowPositioningMode
    }
}
