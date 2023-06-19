/// Coordinate native type.
pub type SizeType = ::std::os::raw::c_int;

/// X-Plane size definition.
#[derive(Default)]
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
}
