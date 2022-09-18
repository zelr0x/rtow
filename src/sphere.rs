use crate::{point::Point3, hit::{Hit, HitData}, ray::Ray, vec3::Vec3};

#[derive(Debug, Clone, Default)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub fn center(&self) -> &Point3 {
        &self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitData> {
        let oc = ray.origin() - self.center();
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(&oc, ray.direction());
        let c = oc.length_squared() - self.radius() * self.radius();
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None
        }
        let t = nearest_root(a, half_b, discriminant, t_min, t_max)?;
        let point = ray.at(t);
        let normal = &(&point - self.center()) / self.radius;
        Some(HitData::new(point, normal, t))
    }
}

/// Finds the nearest root that lies in the specified range.
fn nearest_root(
    a: f64,
    half_b: f64,
    discriminant: f64,
    t_min: f64,
    t_max: f64,
) -> Option<f64> {
    let sqrtd = discriminant.sqrt();
    let mut root = (-half_b - sqrtd) / a;
    if root < t_min || root > t_max {
        root = (-half_b + sqrtd) / a;
        if root < t_min || root > t_max {
            return None
        }
    }
    Some(root)
}
