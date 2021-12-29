use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

#[derive(Default, Debug)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: f64) -> Option<HitRecord>;
}
