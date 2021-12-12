use core::ops::{Add, Sub};

#[derive(Debug, Clone)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T: Copy> Copy for Vec3<T> {
}

impl<T: PartialEq> PartialEq for Vec3<T> {
    fn eq(&self, rhs: &Vec3<T>) -> bool {
        self.x == rhs.x
        && self.y == rhs.y
        && self.z == rhs.z
    }
}

impl<T: Default> Default for Vec3<T> {
    fn default() -> Vec3<T> {
        Vec3 {
            x: Default::default(),
            y: Default::default(),
            z: Default::default()
        }
    }
}

impl<T: Add<Output=T>> Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl<T: Sub<Output=T>> Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<T> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 {x, y, z}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_eq_simple_works() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(1, 2, 3);
        assert_eq!(a, b);
    }

    #[test]
    fn partial_eq_recursive_works() {
        let a = Vec3::new(
            Vec3::new(1, 2, 3),
            Vec3::new(4, 5, 6),
            Vec3::new(7, 8, 9));
        let b = Vec3::new(
            Vec3::new(1, 2, 3),
            Vec3::new(4, 5, 6),
            Vec3::new(7, 8, 9));
        assert_eq!(a, b);
    }

    #[test]
    fn default_works() {
        let d: Vec3<f64> = Vec3::default();
        assert_eq!(Vec3::new(0.0, 0.0, 0.0), d);
    }

    #[test]
    fn add_simple_works() {
        let a: Vec3<f64> = Vec3::new(2.0, 3.0, 10.0);
        let b: Vec3<f64> = Vec3::new(1.0, 2.0, 5.0);
        assert_eq!(Vec3::new(3.0, 5.0, 15.0), a + b);
    }
}
