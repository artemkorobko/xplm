use std::ffi;

use thiserror::Error;

/// An error returned from plugin API calls
#[derive(Error, Debug)]
pub enum MenusError {
    /// Invalid menu ID
    #[error("invalid menu id")]
    InvalidId(xplm_sys::XPLMMenuID),
    /// Can't create menu
    #[error("can't create menu")]
    CreateError,
    /// Invalid menu name string passed to X-Plane
    #[error("invalid menu name {0}")]
    InvalidMenuName(ffi::NulError),
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

/// Menu item.
pub struct MenuItem(::std::os::raw::c_int);

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

/// Creates a top level menu and returns its ID.
///
/// # Arguments
/// * `name` - menu name.
///
/// # Returns
/// Returns a [`MenuId`] on success. Otherwise returns:
/// [`MenusError::InvalidMenuName`] in case manu name contains invalid characters.
/// [`MenusError::CreateError`] in case the menu can't be created.
pub fn create_menu<T: Into<String>>(name: T) -> Result<MenuId, MenusError> {
    let name_c = ffi::CString::new(name.into()).map_err(MenusError::InvalidMenuName)?;
    let menu_id = unsafe {
        xplm_sys::XPLMCreateMenu(
            name_c.as_ptr(),
            std::ptr::null_mut(),
            0,
            None,
            std::ptr::null_mut(),
        )
    };

    if menu_id.is_null() {
        Err(MenusError::CreateError)
    } else {
        Ok(MenuId(menu_id))
    }
}

/// Creates a new sub-menu and returns its ID.
///
/// # Arguments
/// * `parent_menu` - parent menu to attach sub-menu to.
/// * `parent_item` - a menu item.
///
/// # Returns
/// Returns a [`MenuId`] on success. Otherwise returns [`MenusError::CreateError`].
pub fn create_sub_menu(parent_menu: &MenuId, parent_item: &MenuItem) -> Result<MenuId, MenusError> {
    unsafe extern "C" fn menu_handler(
        _menu_ref: *mut ::std::os::raw::c_void,
        _item_ref: *mut ::std::os::raw::c_void,
    ) {
        // let item = item_ref as *const Item;
        // (*item).handle_click();
    }

    let menu_id = unsafe {
        xplm_sys::XPLMCreateMenu(
            std::ptr::null_mut(),
            parent_menu.0,
            parent_item.0,
            Some(menu_handler),
            std::ptr::null_mut(),
        )
    };

    if menu_id.is_null() {
        Err(MenusError::CreateError)
    } else {
        Ok(MenuId(menu_id))
    }
}

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
