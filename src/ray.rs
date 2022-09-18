use crate::point::Point3;
use crate::vec3::Vec3;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub const fn new(origin: &Point3, direction: &Vec3) -> Ray {
        Ray {
            origin: origin.clone(),
            direction: direction.clone(),
        }
    }

    pub const fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub const fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub const fn at(&self, t: f64) -> Point3 {
        self.origin() + t * self.direction()
    }

    pub fn hit_sphere(&self, center: &Point3, radius: f64) -> f64 {
        let oc = self.origin() - center;
        let a = self.direction().length_squared();
        let half_b = Vec3::dot(&oc, self.direction());
        let c = oc.length_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }
}
