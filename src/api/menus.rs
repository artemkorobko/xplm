use std::{ffi, string};

use thiserror::Error;

/// An error returned from plugin API calls
#[derive(Error, Debug)]
pub enum MenusError {
    /// Invalid menu ID
    #[error("invalid menu id")]
    InvalidId(xplm_sys::XPLMMenuID),
    /// Invalid output string passed from Rust to C
    #[error("invalid output string {0}")]
    InvalidOutputString(ffi::NulError),
    /// Invalid input string passed from C to Rust
    #[error("invalid input string {0}")]
    InvalidInputString(string::FromUtf8Error),
}

/// Menu idenitifier.
pub struct MenuId(xplm_sys::XPLMMenuID);

impl TryFrom<xplm_sys::XPLMMenuID> for MenuId {
    type Error = MenusError;

    fn try_from(value: xplm_sys::XPLMMenuID) -> Result<Self, Self::Error> {
        if value.is_null() {
            Err(Self::Error::InvalidId(value))
        } else {
            Ok(MenuId(value))
        }
    }
}

/// Returns the ID of the plug-ins menu, which is created for you at startup.
///
/// # Returns
/// Return [`MenuId`] in case of success. Otherwise returns [`MenusError::InvalidId`]
pub fn find_plugins_menu() -> Result<MenuId, MenusError> {
    let id = unsafe { xplm_sys::XPLMFindPluginsMenu() };
    MenuId::try_from(id)
}

/// Returns the ID of the menu for the currently-loaded aircraft, used for showing
/// aircraft-specific commands. Only plugins loaded with the user’s current aircraft
/// are allowed to access the aircraft menu. For all other plugins, this will return
/// [`MenusError`], and any attempts to add menu items to it will fail.
///
/// # Returns
/// Return [`MenuId`] in case of success. Otherwise returns [`MenusError::InvalidId`]
pub fn find_aircraft_menu() -> Result<MenuId, MenusError> {
    let id = unsafe { xplm_sys::XPLMFindAircraftMenu() };
    MenuId::try_from(id)
}

// /// Creates a new menu and returns its ID.
// pub fn create_menu(name: &str) -> Result<MenuId, MenusError> {
//     extern "C" fn callback(menu_ref: i32, item_ref: i32) {}
// }

/// This function destroys a menu that you have created. Use this to remove a submenu if necessary.
/// (Normally this function will not be necessary.)
///
/// # Arguments
/// * `id` - a menu id to destroy
pub fn destroy_menu(id: MenuId) {
    unsafe { xplm_sys::XPLMDestroyMenu(id.0) };
}

// /// Appends a new menu item to the bottom of a menu and returns its index.
// pub fn append_menu_item(id: &MenuId, name: &str) {
// }

