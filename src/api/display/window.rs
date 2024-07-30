use std::ops::Deref;

use crate::api::utilities::VirtualKey;

use super::{destroy_window, Coord, DisplayError, EventState, KeyFlags, MouseStatus, WheelAxis};

/// X-Plane window identifier.
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
    fn mouse_click(&mut self, coord: Coord, status: MouseStatus) -> EventState;

    /// This function is called when a key is pressed or keyboard focus is taken away from your window.
    ///
    /// # Arguments
    /// * `key` - the key character which has been pressed or released.
    /// * `virtual_key` - the virtual key which has been pressed or released.
    /// * `flags` - the key flags bitmap which contains state for special keys and whether the key
    ///     has been pressed or released.
    fn handle_key(&mut self, key: char, virtual_key: VirtualKey, flags: KeyFlags);

    /// Get's called when the mouse is over the plugin window.
    ///
    /// # Arguments
    /// * `coord` - coordinates at which cursor event occured.
    fn handle_cursor(&mut self, coord: Coord);

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
        coord: Coord,
        wheel_axis: WheelAxis,
        clicks: i32,
    ) -> EventState;
}

/// A link to [`WindowHandler`] for a given window.
pub struct WindowLink(Box<dyn WindowHandler>);

impl WindowLink {
    /// Creates a new [`WindowLink`] instance.
    ///
    /// # Arguments
    /// * `value` - a pointer to the [`WindowHandler`] instance.
    ///
    /// # Returns
    /// Return the window link instance.
    pub fn new(value: Box<dyn WindowHandler>) -> Self {
        Self(value)
    }
}

impl WindowHandler for WindowLink {
    fn draw(&mut self, id: &WindowId) {
        self.0.draw(id);
    }

    fn mouse_click(&mut self, coord: Coord, status: MouseStatus) -> EventState {
        self.0.mouse_click(coord, status)
    }

    fn handle_key(&mut self, key: char, virtual_key: VirtualKey, flags: KeyFlags) {
        self.0.handle_key(key, virtual_key, flags);
    }

    fn handle_cursor(&mut self, coord: Coord) {
        self.0.handle_cursor(coord);
    }

    fn handle_mouse_wheel(
        &mut self,
        coord: Coord,
        wheel_axis: WheelAxis,
        clicks: i32,
    ) -> EventState {
        self.0.handle_mouse_wheel(coord, wheel_axis, clicks)
    }
}

/// A window handler record to keep a window alive.
pub struct WindowHandlerRecord {
    /// A window identifier.
    pub id: WindowId,
    /// A window link to event handler.
    pub link: Box<WindowLink>,
}

impl WindowHandlerRecord {
    /// Creates a new window handler record instance.
    ///
    /// # Arguments
    /// * `id` - the window identifier.
    /// * `link` - a pointer to the window link.
    ///
    /// # Return
    /// Return the new window handler record instance.
    pub fn new(id: WindowId, link: Box<WindowLink>) -> Self {
        Self { id, link }
    }
}

impl Drop for WindowHandlerRecord {
    fn drop(&mut self) {
        destroy_window(&self.id)
    }
}

/// A window positioning mode.
pub enum PositioningMode {
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

impl From<PositioningMode> for xplm_sys::XPLMWindowPositioningMode {
    fn from(value: PositioningMode) -> Self {
        value as xplm_sys::XPLMWindowPositioningMode
    }
}
