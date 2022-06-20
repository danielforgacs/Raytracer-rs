use crate::vec3::{Point3};
use crate::vec3::{dot};
use crate::hittable::{HitRecord};
use crate::ray::{Ray};

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    fn new() -> Self {
        Self {
            center: Point3::new(),
            radius: 1.0,
        }
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.get_origin() - self.center;
        let a = ray.get_direction().length_squared();
        let half_b = dot(&oc, &ray.get_direction());
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if (root < t_min) || (t_max < root) {
            root = (-half_b + sqrtd) / a;
            if (root < t_min) || (t_max < root) {
                return false;
            }
        }
        rec.t = root;
        rec.p = ray.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        return true;
    }
}
