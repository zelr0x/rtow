use std::ops::{Index, Neg, AddAssign, MulAssign, DivAssign};

#[derive(Debug, Clone, PartialEq)]
pub struct Vec3 {
    e: [f64; 3]
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {e: [x, y, z]}
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub const fn length_squared(&self) -> f64 {
        self.x() * self.x()
        + self.y() * self.y()
        + self.z() * self.z()
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

impl const Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl const Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        &self.e[idx]
    }
}

impl AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.e[0] += rhs.x();
        self.e[1] += rhs.y();
        self.e[2] += rhs.z();
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1. / t;
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
        let v= Vec3::default();
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
    fn add_assign_works() {
        let mut a = Vec3::new(2.0, 3.0, 10.0);
        let b = Vec3::new(1.0, 2.0, 5.0);
        a += &b;
        assert_eq!(Vec3::new(3.0, 5.0, 15.0), a);
    }
}
