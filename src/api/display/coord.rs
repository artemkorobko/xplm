/// Coordinate native type.
pub type CoordType = ::std::os::raw::c_int;

/// X-Plane 2D coordinate definition.
#[derive(Debug, Default)]
pub struct Coord {
    /// The X coordinate.
    pub x: CoordType,
    /// The Y coordinate.
    pub y: CoordType,
}

impl Coord {
    /// Created a new coordinate.
    ///
    /// # Arguments
    /// * `x` - the X coordinate.
    /// * `y` - the Y coordinate.
    ///
    /// # Returns
    /// Returns newly created coordinate representation.
    pub fn new(x: CoordType, y: CoordType) -> Self {
        Self { x, y }
    }

    /// Sets the X coordinate.
    ///
    /// # Arguments
    /// * `value` - the X coordinate.
    ///
    /// # Returns
    /// Returns new instance of the coordinate with modified parameter.
    pub fn x(mut self, value: CoordType) -> Self {
        self.x = value;
        self
    }

    /// Sets the Y coordinate.
    ///
    /// # Arguments
    /// * `value` - the Y coordinate.
    ///
    /// # Returns
    /// Returns new instance of the coordinate with modified parameter.
    pub fn y(mut self, value: CoordType) -> Self {
        self.y = value;
        self
    }
}
