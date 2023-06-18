/// X-Plane 2D coordinate definition.
#[derive(Default)]
pub struct Coord {
    /// The X coordinate.
    pub x: ::std::os::raw::c_int,
    /// The Y coordinate.
    pub y: ::std::os::raw::c_int,
}

impl Coord {
    /// Created a new coordinate.
    ///
    /// # Arguments
    /// * `x` - an X coordinate.
    /// * `y` - an Y coordinate.
    ///
    /// # Returns
    /// Returns newly created coordinate representation.
    pub fn new(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int) -> Self {
        Self { x, y }
    }

    /// Sets the X coordinate.
    ///
    /// # Arguments
    /// * `value` - the X coordinate.
    ///
    /// # Returns
    /// Returns new instance of the coordinate with modified parameter.
    pub fn x(mut self, value: ::std::os::raw::c_int) -> Self {
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
    pub fn y(mut self, value: ::std::os::raw::c_int) -> Self {
        self.y = value;
        self
    }
}
