/// X-Plane 2D rectangle definiton.
#[derive(Default)]
pub struct WindowGravityRect {
    /// The left coordinate.
    pub left: f32,
    /// The top coordinate.
    pub top: f32,
    /// The right coordinate.
    pub right: f32,
    /// The bottom coordinate.
    pub bottom: f32,
}

impl WindowGravityRect {
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
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    /// Sets the left coordinate of the rectangle.
    ///
    /// # Arguments
    /// * `value` - the left coordinate of the rectangle.
    ///
    /// # Returns
    /// Returns new instance of the rectangle with modified parameter.
    pub fn left(mut self, value: f32) -> Self {
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
    pub fn top(mut self, value: f32) -> Self {
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
    pub fn right(mut self, value: f32) -> Self {
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
    pub fn bottom(mut self, value: f32) -> Self {
        self.bottom = value;
        self
    }
}
