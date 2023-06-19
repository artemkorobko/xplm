/// Rectangle native type.
pub type RectCoordType = ::std::os::raw::c_int;

/// X-Plane 2D rectangle definiton.
#[derive(Default)]
pub struct Rect {
    /// The left coordinate.
    pub left: RectCoordType,
    /// The top coordinate.
    pub top: RectCoordType,
    /// The right coordinate.
    pub right: RectCoordType,
    /// The bottom coordinate.
    pub bottom: RectCoordType,
}

impl Rect {
    /// Constructs a new rectange.
    ///
    /// # Arguments
    /// * `left` - the left coordinate of the rectangle.
    /// * `top` - the top coordinate of the rectangle.
    /// * `width` - the width of the rectangle.
    /// * `height` - the height of the rectangle.
    ///
    /// # Returns
    /// Returns newly create rectangle.
    pub fn new(
        left: RectCoordType,
        top: RectCoordType,
        width: RectCoordType,
        height: RectCoordType,
    ) -> Self {
        Self {
            left,
            top,
            right: width,
            bottom: height,
        }
    }

    /// Sets the left coordinate of the rectangle.
    ///
    /// # Arguments
    /// * `value` - the left coordinate of the rectangle.
    ///
    /// # Returns
    /// Returns new instance of the rectangle with modified parameter.
    pub fn left(mut self, value: RectCoordType) -> Self {
        self.left = value;
        self
    }

    /// Sets the top coordinate of the rectangle.
    ///
    /// # Arguments
    /// * `value` - the top coordinate of the rectangle.
    ///
    /// # Returns
    /// Returns new instance of the rectangle with modified parameter.
    pub fn top(mut self, value: RectCoordType) -> Self {
        self.top = value;
        self
    }

    /// Sets the right coordinate of the rectangle.
    ///
    /// # Arguments
    /// * `value` - the right coordinate of the rectangle.
    ///
    /// # Returns
    /// Returns new instance of the rectangle with modified parameter.
    pub fn right(mut self, value: RectCoordType) -> Self {
        self.right = value;
        self
    }

    /// Sets the bottom coordinate of the rectangle.
    ///
    /// # Arguments
    /// * `value` - the bottom coordinate of the rectangle.
    ///
    /// # Returns
    /// Returns new instance of the rectangle with modified parameter.
    pub fn bottom(mut self, value: RectCoordType) -> Self {
        self.bottom = value;
        self
    }
}
