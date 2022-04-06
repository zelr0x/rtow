use crate::point::Point3;
use crate::vec3::Vec3;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub const fn new(origin: &Point3, direction: &Vec3) ->  Ray {
        Ray { origin: origin.clone(), direction: direction.clone() }
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
}
