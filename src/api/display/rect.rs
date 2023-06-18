/// A simple rectangle representation.
#[derive(Default)]
pub struct Rect {
    pub left: ::std::os::raw::c_int,
    pub top: ::std::os::raw::c_int,
    pub right: ::std::os::raw::c_int,
    pub bottom: ::std::os::raw::c_int,
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
        left: ::std::os::raw::c_int,
        top: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
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
    pub fn left(mut self, value: ::std::os::raw::c_int) -> Self {
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
    pub fn top(mut self, value: ::std::os::raw::c_int) -> Self {
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
    pub fn right(mut self, value: ::std::os::raw::c_int) -> Self {
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
    pub fn bottom(mut self, value: ::std::os::raw::c_int) -> Self {
        self.bottom = value;
        self
    }
}
