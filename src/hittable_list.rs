use crate::hittable::*;
// use crate::vec3::*;
use crate::ray::*;

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self {
        Self { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closet_so_far = t_max;

        for object in &self.list {
            if object.hit(&r, &t_min, &closet_so_far, &mut temp_rec) {
                hit_anything = true;
                // closet_so_far = temp_rec.t;
                // rec = temp_rec;
            }
        }
        hit_anything
    }
}
