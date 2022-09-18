use crate::{point::Point3, vec3::Vec3, ray::Ray};

#[derive(Debug, Clone)]
pub struct HitResult {
    point: Point3,
    normal: Vec3,
    t: f64,
}

impl HitResult {
    pub fn new(point: Point3, normal: Vec3, t: f64) -> HitResult {
        HitResult { point, normal, t }
    }

    pub fn point(&self) -> &Point3 {
        &self.point
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitResult>;
}
