use std::fmt;

use crate::vec3::Vec3;

/// `Color` represents an RGB color.
pub struct Color(Vec3);

impl Color {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Color(Vec3::new(r, g, b))
    }
}

impl fmt::Display for Color {
    /// Write the translated [0,255] value of each color component.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (255.999 * self.0.x()) as i64,
            (255.999 * self.0.y()) as i64,
            (255.999 * self.0.z()) as i64
        )
    }
}
