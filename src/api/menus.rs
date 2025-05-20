pub mod error;
pub mod handler;
pub mod menu;
pub mod state;

use std::{ffi, ops::Deref};

use handler::Menu;

pub use self::error::MenusError;
pub use self::handler::MenuHandler;
pub use self::menu::MenuId;
pub use self::menu::MenuItem;
pub use self::menu::MenuItemId;
pub use self::state::MenuItemState;

use super::utilities::Command;

pub type Result<T> = std::result::Result<T, MenusError>;

/// Returns the ID of the plug-ins menu, which is created for you at startup.
///
/// # Returns
/// Return [`MenuId`] in case of success. Otherwise returns [`MenusError`]
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
/// Return [`MenuId`] in case of success. Otherwise returns [`MenusError`]
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
/// Returns a [`MenuId`] on success. Otherwise returns [`MenusError`].
pub fn create_menu<T: Into<String>, H: MenuHandler>(
    name: T,
    parent: &MenuId,
    parent_item: &MenuItemId,
    handler: H,
) -> Result<Menu> {
    let handler_box = Box::new(handler);
    let handler_ptr = &*handler_box as *const dyn MenuHandler;
    let name_c = ffi::CString::new(name.into()).map_err(MenusError::InvalidMenuName)?;
    let id = unsafe {
        xplm_sys::XPLMCreateMenu(
            name_c.as_ptr(),
            parent.native(),
            parent_item.native(),
            Some(menu_handler::<H>),
            handler_ptr as _,
        )
    };

    let menu_id = MenuId::try_from(id)?;
    Ok(Menu::new(menu_id, handler_box))
}

/// Creates a new sub-menu and returns its ID.
///
/// # Arguments
/// * `parent_menu` - parent menu to attach sub-menu to.
/// * `parent_item` - a menu item.
///
/// # Returns
/// Returns a [`MenuId`] on success. Otherwise returns [`MenusError`].
pub fn create_sub_menu<H: MenuHandler>(
    parent_menu: &MenuId,
    parent_item: &MenuItemId,
    handler: H,
) -> Result<Menu> {
    let handler_box = Box::new(handler);
    let handler_ptr = &*handler_box as *const dyn MenuHandler;
    let id = unsafe {
        xplm_sys::XPLMCreateMenu(
            std::ptr::null_mut(),
            parent_menu.native(),
            parent_item.native(),
            Some(menu_handler::<H>),
            handler_ptr as _,
        )
    };

    let menu_id = MenuId::try_from(id)?;
    Ok(Menu::new(menu_id, handler_box))
}

unsafe extern "C" fn menu_handler<H: MenuHandler>(
    menu_ref: *mut ::std::os::raw::c_void,
    item_ref: *mut ::std::os::raw::c_void,
) {
    if menu_ref.is_null() || item_ref.is_null() {
        return;
    }

    let handler_ptr = unsafe { &mut *menu_ref.cast::<H>() };
    let index = item_ref.cast::<usize>();
    handler_ptr.handle_click(index as usize);
}

/// This function destroys a menu that you have created. Use this to remove a submenu if necessary.
/// (Normally this function will not be necessary.)
///
/// # Arguments
/// * `id` - a menu id to destroy
pub fn destroy_menu(id: &MenuId) {
    unsafe { xplm_sys::XPLMDestroyMenu(id.native()) };
}

/// Removes all menu items from a menu.
///
/// # Arguments
/// * `id` - a menu id to destroy
pub fn clear_all_menu_items(id: &MenuId) {
    unsafe { xplm_sys::XPLMClearAllMenuItems(id.native()) };
}

/// Appends a new menu item to the bottom of a menu and returns its index.
///
/// # Arguments
/// * `parent` - parent menu to add item to.
/// * `text` - a menu text.
///
/// # Returns
/// Return a new [`MenuItemId`] on success. Otherwise return [`MenusError`].
pub fn append_menu_item<T: Into<String>>(
    parent: &MenuId,
    text: T,
    index: usize,
) -> Result<MenuItem> {
    let text_c = ffi::CString::new(text.into()).map_err(MenusError::InvalidMenuName)?;
    let id =
        unsafe { xplm_sys::XPLMAppendMenuItem(parent.native(), text_c.as_ptr(), index as _, 0) };
    Ok(MenuItem::new(*parent, MenuItemId::try_from(id)?))
}

/// Appends a new menu item to the bottom of a menu and returns its index but instead of the new menu
/// item triggering the handler of the containiner menu, it will simply execute the passed-in command.
///
/// # Arguments
/// * `parent` - parent menu to add item to.
/// * `text` - a menu text.
/// * `command` - a command to execute.
///
/// # Returns
/// Returns a new [`MenuItemId`] on success. Otherwuse returns [`MenusError`].
pub fn append_menu_item_with_command<T: Into<String>>(
    parent: &MenuId,
    text: T,
    command: &Command,
) -> Result<MenuItemId> {
    let text_c = ffi::CString::new(text.into()).map_err(MenusError::InvalidMenuName)?;
    let id = unsafe {
        xplm_sys::XPLMAppendMenuItemWithCommand(parent.native(), text_c.as_ptr(), *command.deref())
    };
    MenuItemId::try_from(id)
}

/// Adds a separator to the end of a menu.
///
/// # Arguments
/// * `parent` - parent menu to add a separator to.
pub fn append_menu_separator(parent: &MenuId) {
    unsafe { xplm_sys::XPLMAppendMenuSeparator(parent.native()) };
}

/// Changes the name of an existing menu item.
///
/// # Arguments
/// * `item` - a menu item to update.
/// * `text` - new menu item text.
///
/// # Returns
/// Returns empty result on success. Otherwise return [`MenusError`].
pub fn set_menu_item_name<T: Into<String>>(item: &MenuItem, text: T) -> Result<()> {
    let text_c = ffi::CString::new(text.into()).map_err(MenusError::InvalidMenuName)?;
    unsafe {
        xplm_sys::XPLMSetMenuItemName(
            item.parent().native(),
            item.id().native(),
            text_c.as_ptr(),
            0,
        )
    };
    Ok(())
}

/// Checks a menu item.
///
/// # Arguments
/// * `parent` - a parent menu id which contains an item.
/// * `item` - a menu item to update.
pub fn check_menu_item(parent: &MenuId, item: &MenuItemId) {
    unsafe {
        xplm_sys::XPLMCheckMenuItem(
            parent.native(),
            item.native(),
            xplm_sys::xplm_Menu_Checked as i32,
        )
    };
}

/// Unchecks a menu item.
///
/// # Arguments
/// * `parent` - a parent menu id which contains an item.
/// * `item` - a menu item to update.
pub fn uncheck_menu_item(parent: &MenuId, item: &MenuItemId) {
    unsafe {
        xplm_sys::XPLMCheckMenuItem(
            parent.native(),
            item.native(),
            xplm_sys::xplm_Menu_Unchecked as i32,
        )
    };
}

/// Returns whether a menu item is checked or not.
///
/// # Arguments
/// * `parent` - a parent menu id which contains an item.
/// * `item` - a menu item to update.
///
/// # Returns
/// Returns [`MenuItemState`] on success. Otherwise returns [`MenusError`].
pub fn check_menu_item_state(parent: &MenuId, item: &MenuItemId) -> Result<MenuItemState> {
    let mut state = 0;
    unsafe { xplm_sys::XPLMCheckMenuItemState(parent.native(), item.native(), &mut state) };
    MenuItemState::try_from(state)
}

/// Enables a menu item.
///
/// # Arguments
/// * `parent` - a parent menu id which contains an item.
/// * `item` - a menu item to update.
pub fn enable_menu_item(parent: &MenuId, item: &MenuItemId) {
    unsafe { xplm_sys::XPLMEnableMenuItem(parent.native(), item.native(), 1) };
}

/// Disables a menu item.
///
/// # Arguments
/// * `parent` - a parent menu id which contains an item.
/// * `item` - a menu item to update.
pub fn disable_menu_item(parent: &MenuId, item: &MenuItemId) {
    unsafe { xplm_sys::XPLMEnableMenuItem(parent.native(), item.native(), 0) };
}

/// Removes a menu item from a menu.
///
/// # Arguments
/// * `parent` - a parent menu id which contains an item.
/// * `item` - a menu item to update.
pub fn remove_menu_item(parent: &MenuId, item: &MenuItemId) {
    unsafe { xplm_sys::XPLMRemoveMenuItem(parent.native(), item.native()) };
}
