pub mod error;

use std::{ffi, ops::Deref};

pub use self::error::MenusError;

use super::utilities::Command;

pub type Result<T> = std::result::Result<T, MenusError>;

/// Menu idenitifier.
pub struct MenuId(xplm_sys::XPLMMenuID);

impl TryFrom<xplm_sys::XPLMMenuID> for MenuId {
    type Error = MenusError;

    fn try_from(value: xplm_sys::XPLMMenuID) -> Result<Self> {
        if value.is_null() {
            Err(Self::Error::InvalidId)
        } else {
            Ok(Self(value))
        }
    }
}

/// Menu item identifier.
pub struct MenuItemId(::std::os::raw::c_int);

impl TryFrom<::std::os::raw::c_int> for MenuItemId {
    type Error = MenusError;

    fn try_from(value: ::std::os::raw::c_int) -> std::result::Result<Self, Self::Error> {
        if value < 0 {
            Err(Self::Error::InvalidMenuItemId)
        } else {
            Ok(Self(value))
        }
    }
}

/// Returns the ID of the plug-ins menu, which is created for you at startup.
///
/// # Returns
/// Return [`MenuId`] in case of success. Otherwise returns [`MenusError::InvalidId`]
pub fn find_plugins_menu() -> Result<MenuId> {
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
pub fn find_aircraft_menu() -> Result<MenuId> {
    let id = unsafe { xplm_sys::XPLMFindAircraftMenu() };
    MenuId::try_from(id)
}

/// Creates a top level menu and returns its ID.
///
/// # Arguments
/// * `name` - menu name.
///
/// # Returns
/// Returns a [`MenuId`] on success. Otherwise returns:
/// [`MenusError::InvalidMenuName`] in case manu name contains invalid characters.
/// [`MenusError::CreateError`] in case the menu can't be created.
pub fn create_menu<T: Into<String>>(name: T) -> Result<MenuId> {
    let name_c = ffi::CString::new(name.into()).map_err(MenusError::InvalidMenuName)?;
    let id = unsafe {
        xplm_sys::XPLMCreateMenu(
            name_c.as_ptr(),
            std::ptr::null_mut(),
            0,
            None,
            std::ptr::null_mut(),
        )
    };

    MenuId::try_from(id)
}

/// Creates a new sub-menu and returns its ID.
///
/// # Arguments
/// * `parent_menu` - parent menu to attach sub-menu to.
/// * `parent_item` - a menu item.
///
/// # Returns
/// Returns a [`MenuId`] on success. Otherwise returns [`MenusError::CreateError`].
pub fn create_sub_menu(parent_menu: &MenuId, parent_item: &MenuItemId) -> Result<MenuId> {
    unsafe extern "C" fn menu_handler(
        _menu_ref: *mut ::std::os::raw::c_void,
        _item_ref: *mut ::std::os::raw::c_void,
    ) {
        // let item = item_ref as *const Item;
        // (*item).handle_click();
    }

    let id = unsafe {
        xplm_sys::XPLMCreateMenu(
            std::ptr::null_mut(),
            parent_menu.0,
            parent_item.0,
            Some(menu_handler),
            std::ptr::null_mut(),
        )
    };

    MenuId::try_from(id)
}

/// This function destroys a menu that you have created. Use this to remove a submenu if necessary.
/// (Normally this function will not be necessary.)
///
/// # Arguments
/// * `id` - a menu id to destroy
pub fn destroy_menu(id: &MenuId) {
    unsafe { xplm_sys::XPLMDestroyMenu(id.0) };
}

/// Removes all menu items from a menu.
///
/// # Arguments
/// * `id` - a menu id to destroy
pub fn clear_all_menu_items(id: &MenuId) {
    unsafe { xplm_sys::XPLMClearAllMenuItems(id.0) };
}

/// Appends a new menu item to the bottom of a menu and returns its index.
///
/// # Arguments
/// * `parent` - parent menu to add item to.
/// * `text` - a menu text.
pub fn append_menu_item<T: Into<String>>(parent: &MenuId, text: T) -> Result<MenuItemId> {
    let text_c = ffi::CString::new(text.into()).map_err(MenusError::InvalidMenuName)?;
    let id =
        unsafe { xplm_sys::XPLMAppendMenuItem(parent.0, text_c.as_ptr(), std::ptr::null_mut(), 0) };
    MenuItemId::try_from(id)
}

/// Appends a new menu item to the bottom of a menu and returns its index but instead of the new menu
/// item triggering the handler of the containiner menu, it will simply execute the passed-in command.
///
/// # Arguments
/// * `parent` - parent menu to add item to.
/// * `text` - a menu text.
/// * `command` - a command to execute.
pub fn append_menu_item_with_command<T: Into<String>>(
    parent: &MenuId,
    text: T,
    command: &Command,
) -> Result<MenuItemId> {
    let text_c = ffi::CString::new(text.into()).map_err(MenusError::InvalidMenuName)?;
    let id = unsafe {
        xplm_sys::XPLMAppendMenuItemWithCommand(parent.0, text_c.as_ptr(), *command.deref())
    };
    MenuItemId::try_from(id)
}

/// Adds a separator to the end of a menu.
///
/// # Arguments
/// * `parent` - parent menu to add a separator to.
pub fn append_menu_separator(parent: &MenuId) {
    unsafe { xplm_sys::XPLMAppendMenuSeparator(parent.0) };
}

/// Changes the name of an existing menu item.
///
/// # Arguments
/// * `parent` - a parent menu id which contains an item.
/// * `item` - a menu item to update.
/// * `text` - new menu item text.
pub fn set_menu_item_name<T: Into<String>>(
    parent: &MenuId,
    item: &MenuItemId,
    text: T,
) -> Result<()> {
    let text_c = ffi::CString::new(text.into()).map_err(MenusError::InvalidMenuName)?;
    unsafe { xplm_sys::XPLMSetMenuItemName(parent.0, item.0, text_c.as_ptr(), 0) };
    Ok(())
}

/// Checks a menu item.
///
/// # Arguments
/// * `parent` - a parent menu id which contains an item.
/// * `item` - a menu item to update.
pub fn check_menu_item(parent: &MenuId, item: &MenuItemId) {
    unsafe { xplm_sys::XPLMCheckMenuItem(parent.0, item.0, xplm_sys::xplm_Menu_Checked as i32) };
}

/// Unchecks a menu item.
///
/// # Arguments
/// * `parent` - a parent menu id which contains an item.
/// * `item` - a menu item to update.
pub fn uncheck_menu_item(parent: &MenuId, item: &MenuItemId) {
    unsafe { xplm_sys::XPLMCheckMenuItem(parent.0, item.0, xplm_sys::xplm_Menu_Unchecked as i32) };
}

/// Menu item state.
pub enum MenuItemState {
    /// The menu has a mark next to it that is checked (lit).
    Checked,
    /// The menu has a mark next to it that is unmarked (not lit).
    Unchecked,
    /// There is no symbol to the left of the menu item.
    NoCheck,
}

impl TryFrom<xplm_sys::XPLMMenuCheck> for MenuItemState {
    type Error = MenusError;

    fn try_from(value: xplm_sys::XPLMMenuCheck) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(MenuItemState::NoCheck),
            1 => Ok(MenuItemState::Unchecked),
            2 => Ok(MenuItemState::Checked),
            _ => Err(Self::Error::UnknownMenuItemState(value)),
        }
    }
}

/// Returns whether a menu item is checked or not.
///
/// # Arguments
/// * `parent` - a parent menu id which contains an item.
/// * `item` - a menu item to update.
pub fn check_menu_item_state(parent: &MenuId, item: &MenuItemId) -> Result<MenuItemState> {
    let mut state = 0;
    unsafe { xplm_sys::XPLMCheckMenuItemState(parent.0, item.0, &mut state) };
    MenuItemState::try_from(state)
}

/// Enables a menu item.
///
/// # Arguments
/// * `parent` - a parent menu id which contains an item.
/// * `item` - a menu item to update.
pub fn enable_menu_item(parent: &MenuId, item: &MenuItemId) {
    unsafe { xplm_sys::XPLMEnableMenuItem(parent.0, item.0, 1) };
}

/// Disables a menu item.
///
/// # Arguments
/// * `parent` - a parent menu id which contains an item.
/// * `item` - a menu item to update.
pub fn disable_menu_item(parent: &MenuId, item: &MenuItemId) {
    unsafe { xplm_sys::XPLMEnableMenuItem(parent.0, item.0, 0) };
}

/// Removes a menu item from a menu.
///
/// # Arguments
/// * `parent` - a parent menu id which contains an item.
/// * `item` - a menu item to update.
pub fn remove_menu_item(parent: &MenuId, item: &MenuItemId) {
    unsafe { xplm_sys::XPLMRemoveMenuItem(parent.0, item.0) };
}
