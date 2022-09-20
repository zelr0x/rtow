use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub},
};

use crate::util;

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn rand() -> Vec3 {
        Vec3::new(util::rand(), util::rand(), util::rand())
    }

    pub fn rand_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            util::rand_range(min, max),
            util::rand_range(min, max),
            util::rand_range(min, max)
        )
    }

    pub fn rand_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::rand_range(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p
            }
        }
    }

    pub const fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub const fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub const fn dot(&self, rhs: &Vec3) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub const fn x(&self) -> f64 {
        self.e[0]
    }

    pub const fn y(&self) -> f64 {
        self.e[1]
    }

    pub const fn z(&self) -> f64 {
        self.e[2]
    }
}

impl const Default for Vec3 {
    fn default() -> Vec3 {
        Vec3::new(0., 0., 0.)
    }
}

impl const Clone for Vec3 {
    fn clone(&self) -> Self {
        Self {
            e: [self.x(), self.y(), self.z()],
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.e[0] = source.x();
        self.e[1] = source.y();
        self.e[2] = source.z();
    }
}

impl const Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl const Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        -(&self)
    }
}

impl const Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.e[idx]
    }
}

impl const Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl const Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        self.add(&rhs)
    }
}

impl const Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        rhs.add(self)
    }
}

impl const Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        self.add(&rhs)
    }
}

impl const Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl const Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        &self - rhs
    }
}

impl const Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self - &rhs
    }
}

impl const Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        &self - &rhs
    }
}

impl const Mul<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl const Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl const Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl const Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1. / rhs) * self
    }
}

impl const Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl const AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.e[0] += rhs.x();
        self.e[1] += rhs.y();
        self.e[2] += rhs.z();
    }
}

impl const MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl const DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1. / t;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_eq_works() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(1., 2., 3.);
        assert_eq!(a, b);
    }

    #[test]
    fn default_works() {
        let v = Vec3::default();
        assert_eq!(Vec3::new(0., 0., 0.), v);
    }

    #[test]
    fn length_works() {
        let v = Vec3::new(3., 4., 5.);
        assert_eq!(v.length(), (3f64 * 3. + 4. * 4. + 5. * 5.).sqrt());
    }

    #[test]
    fn index_works() {
        let v = Vec3::new(1., 2., 3.);
        let first: f64 = v[0];
        assert_eq!(first, 1.);
    }

    #[test]
    fn neg_works() {
        let a = Vec3::new(1., 2., 3.);
        let b = Vec3::new(-1., -2., -3.);
        assert_eq!(-a, b);
    }

    #[test]
    fn neg_ref_works() {
        let a = &Vec3::new(1., 2., 3.);
        let b = Vec3::new(-1., -2., -3.);
        assert_eq!(-a, b);
    }

    #[test]
    fn add_assign_works() {
        let mut a = Vec3::new(2.0, 3.0, 10.0);
        let b = Vec3::new(1.0, 2.0, 5.0);
        a += &b;
        assert_eq!(Vec3::new(3.0, 5.0, 15.0), a);
    }

    #[test]
    fn display_works() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(format!("{}", v), "1 2 3")
    }
}
