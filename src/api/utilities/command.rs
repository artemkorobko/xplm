use std::ops::Deref;

use super::{unregister_command_handler, UtilitiesError};

/// An opaque identifier for an X-Plane command
pub struct Command(xplm_sys::XPLMCommandRef);

impl TryFrom<xplm_sys::XPLMCommandRef> for Command {
    type Error = UtilitiesError;

    fn try_from(value: xplm_sys::XPLMCommandRef) -> std::result::Result<Self, Self::Error> {
        if value.is_null() {
            Err(Self::Error::InvalidCommand)
        } else {
            Ok(Command(value))
        }
    }
}

impl Deref for Command {
    type Target = xplm_sys::XPLMCommandRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Command handler.
pub trait CommandHandler: 'static {
    /// Called when the command begins (corresponds to a button being pressed down)
    fn command_begin(&mut self);
    /// Called frequently while the command button is held down
    fn command_continue(&mut self);
    /// Called when the command ends (corresponds to a button being released)
    fn command_end(&mut self);
}

/// A link to [`CommandHandler`] for a given [`Command`].
pub struct CommandLink {
    pub command: xplm_sys::XPLMCommandRef,
    pub handler: Box<dyn CommandHandler>,
}

impl CommandLink {
    /// Check whether link is pointing to specified command.
    ///
    /// # Arguments
    /// * `command` - a command to validate with. See [`xplm_sys::XPLMCommandRef`] for more details.
    pub fn links_with(&self, command: xplm_sys::XPLMCommandRef) -> bool {
        self.command == command
    }
}

impl CommandHandler for CommandLink {
    /// Invoke command begin on inner handle.
    fn command_begin(&mut self) {
        self.handler.command_begin();
    }

    /// Invoke command continue on inner handle.
    fn command_continue(&mut self) {
        self.handler.command_continue();
    }

    /// Invoke command end on inner handle.
    fn command_end(&mut self) {
        self.handler.command_end();
    }
}

/// A command handler record to keep a registration alive.
pub struct CommandHandlerRecord {
    pub link: Box<CommandLink>,
    pub execution_time: CommandExecutionTime,
}

impl Drop for CommandHandlerRecord {
    fn drop(&mut self) {
        unregister_command_handler(self);
    }
}

/// A command execution time.
#[derive(Copy, Clone)]
pub enum CommandExecutionTime {
    /// A callback will run before X-Plane.
    BeforeXPlane = 1,
    /// A callback will run after X-Plane.
    AfterXPlane = 0,
}

impl From<CommandExecutionTime> for ::std::os::raw::c_int {
    fn from(value: CommandExecutionTime) -> Self {
        value as ::std::os::raw::c_int
    }
}
