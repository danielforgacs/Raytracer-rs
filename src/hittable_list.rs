use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(list: Vec<Box<dyn Hittable>>) -> Self {
        Self { list }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: f64) -> Option<HitRecord> {
        // let mut temp_rec = HitRecord::default();
        let mut hit_record = None;
        // let mut hit_anything = false;
        let mut closet_so_far = t_max;

        for object in &self.list {
            if let Some(rec) = object.hit(&r, &t_min, closet_so_far) {
                closet_so_far = rec.t;
                hit_record = Some(rec);
                // rec.set_t(&temp_rec.t);
                // rec.set_p(&temp_rec.p);
                // rec.set_normal(&temp_rec.normal);
            }
        }
        hit_record
    }
}
