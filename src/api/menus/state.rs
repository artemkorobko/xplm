use super::MenusError;

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

    fn try_from(value: xplm_sys::XPLMMenuCheck) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MenuItemState::NoCheck),
            1 => Ok(MenuItemState::Unchecked),
            2 => Ok(MenuItemState::Checked),
            _ => Err(Self::Error::UnknownMenuItemState(value)),
        }
    }
}
