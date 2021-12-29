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
//
// impl HitRecord {
//     pub fn set_t(&mut self, t: &f64) {
//         self.t = *t;
//     }
//
//     pub fn set_p(&mut self, p: &Vec3) {
//         self.p = *p;
//     }
//
//     pub fn set_normal(&mut self, normal: &Vec3) {
//         self.normal = *normal;
//     }
// }

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: f64) -> Option<HitRecord>;
}
