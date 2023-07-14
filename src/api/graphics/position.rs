/// An X-Plane world position.
#[derive(Default)]
pub struct WorldPosition {
    /// World position latitude.
    pub latitude: f64,
    /// World position longitude.
    pub longitude: f64,
    /// World position altitude.
    pub altitude: f64,
}

impl WorldPosition {
    /// Sets the latitude of the world position.
    ///
    /// # Arguments
    /// * `value` - a latitude value to set.
    ///
    /// # Returns
    /// Returns a modified world position with new latitude.
    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = value;
        self
    }

    /// Sets the longitude of the world position.
    ///
    /// # Arguments
    /// * `value` - a longitude value to set.
    ///
    /// # Returns
    /// Returns a modified world position with new longitude.
    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = value;
        self
    }

    /// Sets the altitude of the world position.
    ///
    /// # Arguments
    /// * `value` - a altitude value to set.
    ///
    /// # Returns
    /// Returns a modified world position with new altitude.
    pub fn altitude(mut self, value: f64) -> Self {
        self.altitude = value;
        self
    }
}

/// An X-Plane local position.
#[derive(Default)]
pub struct LocalPosition {
    /// Local X coordinate.
    pub x: f64,
    /// Local Y coordinate.
    pub y: f64,
    /// Local Z coordinate.
    pub z: f64,
}

impl LocalPosition {
    /// Sets the X coordinate of the local position.
    ///
    /// # Arguments
    /// * `value` - an X coordinate to set.
    ///
    /// # Returns
    /// Returns a modified local position with new X coordinate.
    pub fn x(mut self, value: f64) -> Self {
        self.x = value;
        self
    }

    /// Sets the Y coordinate of the local position.
    ///
    /// # Arguments
    /// * `value` - a Y coordinate to set.
    ///
    /// # Returns
    /// Returns a modified local position with new Y coordinate.
    pub fn y(mut self, value: f64) -> Self {
        self.y = value;
        self
    }

    /// Sets the Z coordinate of the local position.
    ///
    /// # Arguments
    /// * `value` - a Z coordinate to set.
    ///
    /// # Returns
    /// Returns a modified local position with new Z coordinate.
    pub fn z(mut self, value: f64) -> Self {
        self.z = value;
        self
    }
}
