use super::{Coord, Size};

/// Rectangle native type.
pub type RectCoordType = ::std::os::raw::c_int;

/// X-Plane 2D rectangle definiton.
#[derive(Debug, Default)]
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
        right: RectCoordType,
        bottom: RectCoordType,
    ) -> Self {
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

    /// Calculates the rectangle center.
    ///
    /// # Returns
    /// Returnt [`Coord`] that represents rectangle center.
    pub fn center(&self) -> Coord {
        let x = self.left + ((self.right - self.left) / 2);
        let y = self.bottom + ((self.top - self.bottom) / 2);
        Coord::default().x(x).y(y)
    }

    /// Shrinks rectangle to a size
    ///
    /// # Argumets
    /// * `size` - A size to shrink the rect to.
    ///
    /// # Returns
    /// Returns a shrinked rectangle.
    pub fn shrink(self, size: &Size) -> Self {
        let center = self.center();
        let half_width = size.width / 2;
        let half_height = size.height / 2;
        Self {
            left: center.x - half_width,
            top: center.y + half_height,
            right: center.x + half_width,
            bottom: center.y - half_height,
        }
    }
}
