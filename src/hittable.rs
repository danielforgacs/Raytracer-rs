use crate::vec3::*;
use crate::ray::*;

#[derive(Default)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f64, p: Vec3, normal: Vec3) -> Self {
        Self { t, p, normal }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool;
}
