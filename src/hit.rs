use crate::{point::Point3, vec3::Vec3, ray::Ray};

#[derive(Debug, Clone)]
pub struct HitData {
    point: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitData {
    pub const fn new(point: Point3, t: f64, ray: &Ray, outward_normal: &Vec3) -> HitData {
        let front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        let neg: Vec3;
        let normal = if front_face {
            outward_normal.clone()
        } else {
            neg = -outward_normal;
            neg
        };
        HitData { point, t, normal, front_face }
    }

    pub const fn point(&self) -> &Point3 {
        &self.point
    }

    pub const fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub const fn t(&self) -> f64 {
        self.t
    }

    pub const fn front_face(&self) -> bool {
        self.front_face
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitData>;
}

pub struct HitList<'a>(Vec<&'a dyn Hit>);

impl<'a> HitList<'a> {
    pub fn new(obj: &'a dyn Hit) -> HitList<'a> {
        HitList(vec![obj])
    }

    pub fn with_capacity(obj: &'a dyn Hit, cap: usize) -> HitList<'a> {
        let mut res = HitList(Vec::with_capacity(cap));
        res.add(obj);
        res
    }

    pub fn add(&mut self, obj: &'a dyn Hit) {
        self.0.push(obj);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl<'a> Hit for HitList<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitData> {
        let mut res: Option<HitData> = None;
        let mut closest_so_far = t_max;
        for obj in &self.0 {
            if let Some(hd) = obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = hd.t();
                res = Some(hd);
            }
        }
        res
    }
}
