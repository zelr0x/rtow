use std::ops::{Add, Mul, AddAssign};

use crate::vec3::Vec3;

/// `Color` represents an RGB color.
/// Default is required to return RGB (0, 0, 0).
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Color(Vec3);

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TranslatedColor {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl Color {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Color(Vec3::new(r, g, b))
    }

    pub const fn r(&self) -> f64 {
        self.0.x()
    }

    pub const fn g(&self) -> f64 {
        self.0.y()
    }

    pub const fn b(&self) -> f64 {
        self.0.z()
    }

    pub fn translate(&self, samples_per_px: u32) -> TranslatedColor {
        let mut r = self.r();
        let mut g = self.g();
        let mut b = self.b();
        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        let scale = 1.0 / (samples_per_px as f64);
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();
        // Translate value of each color component to [0, 255].
        TranslatedColor {
            r: (256.0 * r.clamp(0.0, 0.999)) as u32,
            g: (256.0 * g.clamp(0.0, 0.999)) as u32,
            b: (256.0 * b.clamp(0.0, 0.999)) as u32,
        }
    }
}

impl const Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color(self * &rhs.0)
    }
}

impl const Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color(&self.0 * rhs)
    }
}

impl const Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        Color(self * &rhs.0)
    }
}

impl const Add<&Color> for Color {
    type Output = Color;

    fn add(self, rhs: &Color) -> Self::Output {
        Color(&self.0 + &rhs.0)
    }
}

impl const Add<Color> for &Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color(&self.0 + &rhs.0)
    }
}

impl const Add<&Color> for &Color {
    type Output = Color;

    fn add(self, rhs: &Color) -> Self::Output {
        Color(&self.0 + &rhs.0)
    }
}

impl const Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color(&self.0 + &rhs.0)
    }
}

impl const Add<&Color> for &Vec3 {
    type Output = Color;

    fn add(self, rhs: &Color) -> Self::Output {
        Color(self + &rhs.0)
    }
}

impl const Add<&Color> for Vec3 {
    type Output = Color;

    fn add(self, rhs: &Color) -> Self::Output {
        Color(&self + &rhs.0)
    }
}

impl const Add<Color> for &Vec3 {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color(self + &rhs.0)
    }
}

impl const Add<Color> for Vec3 {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color(&self + &rhs.0)
    }
}

impl const AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.0.add_assign(&rhs.0)
    }
}
