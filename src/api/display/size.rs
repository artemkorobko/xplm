/// Coordinate native type.
pub type SizeType = ::std::os::raw::c_int;

/// X-Plane size definition.
#[derive(Default, Copy, Clone)]
pub struct Size {
    /// The size width.
    pub width: SizeType,
    /// The size height.
    pub height: SizeType,
}

impl Size {
    /// Created a new size.
    ///
    /// # Arguments
    /// * `width` - the size width.
    /// * `height` - the size height.
    ///
    /// # Returns
    /// Returns newly created size representation.
    pub fn new(width: SizeType, height: SizeType) -> Self {
        Self { width, height }
    }

    /// Sets the width.
    ///
    /// # Arguments
    /// * `value` - the width.
    ///
    /// # Returns
    /// Returns new instance of the size with modified parameter.
    pub fn width(mut self, value: SizeType) -> Self {
        self.width = value;
        self
    }

    /// Sets the height.
    ///
    /// # Arguments
    /// * `value` - the height.
    ///
    /// # Returns
    /// Returns new instance of the size with modified parameter.
    pub fn height(mut self, value: SizeType) -> Self {
        self.height = value;
        self
    }

    /// Adds width to size.
    ///
    /// # Argumets
    /// * `value` - width to add to the size.
    ///
    /// # Returns
    /// Returns a size with added width.
    pub fn add_width(mut self, value: SizeType) -> Self {
        self.width += value;
        self
    }

    /// Adds height to size.
    ///
    /// # Argumets
    /// * `value` - height to add to the size.
    ///
    /// # Returns
    /// Returns a size with added height.
    pub fn add_height(mut self, value: SizeType) -> Self {
        self.height += value;
        self
    }
}
