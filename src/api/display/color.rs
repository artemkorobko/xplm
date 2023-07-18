/// Display color representation.
/// A default color is white.
#[derive(Debug, Copy, Clone)]
pub struct Color {
    /// Red color value.
    pub r: f32,
    /// Green color value.
    pub g: f32,
    /// Blue color value.
    pub b: f32,
}

impl Color {
    /// Creates a new white color.
    ///
    /// # Returns
    /// Return a white color.
    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    /// Creates a new black color.
    ///
    /// # Returns
    /// Return a black color.
    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::white()
    }
}
