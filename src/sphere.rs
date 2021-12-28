use crate::vec3::*;
use crate::ray::*;
use crate::hittable::*;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = dot(&r.direction(), &r.direction());
        let b = dot(&oc, &r.direction()) * 2.0;
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c * 4.0;

        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;

            if (temp < *t_max) && (temp > *t_min) {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if (temp < *t_max) && (temp > *t_min) {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }

        false
    }
}
