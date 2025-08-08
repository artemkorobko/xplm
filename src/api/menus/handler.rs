use super::{MenuId, destroy_menu};

/// A menu callback handler.
pub trait MenuHandler: 'static {
    /// Menu handler.
    ///
    /// # Arguments
    /// * `index` - menu item index.
    fn on_menu_selected(&mut self, index: usize);
}

impl<F> MenuHandler for F
where
    F: 'static + FnMut(usize),
{
    fn on_menu_selected(&mut self, index: usize) {
        self(index)
    }
}

pub struct Menu {
    pub id: MenuId,
    pub handler: Box<dyn MenuHandler>,
}

impl Menu {
    pub fn new(id: MenuId, handler: Box<dyn MenuHandler>) -> Self {
        Self { id, handler }
    }
}

impl Drop for Menu {
    fn drop(&mut self) {
        destroy_menu(&self.id);
    }
}
