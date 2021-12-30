use crate::vec3::{Vec3, dot};
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Self { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = dot(&r.direction, &r.direction);
        let b = dot(&oc, &r.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;

            if temp < t_max && temp > t_min {
                return Some(
                    HitRecord {
                        t: temp,
                        p: r.point_at_parameter(temp),
                        normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                        material: self.material,
                    }
                );
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(
                    HitRecord {
                        t: temp,
                        p: r.point_at_parameter(temp),
                        normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                        material: self.material,
                    }
                );
            }
        }

        None
    }
}
