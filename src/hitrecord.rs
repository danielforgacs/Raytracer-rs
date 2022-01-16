use crate::vec3::{Point3, Vec3, dot};
use crate::ray::{Ray};

struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
}

impl HitRecord {
    fn new() -> Self {
        Self {
            p: Point3::new(),
            normal: Vec3::new(),
            t: 1.0,
        }
    }

    // fn set_face_normal(&self, ray: &Ray, outward_normal: Vec3) {
    //     let front_face = dot(&ray.get_direction(), &outward_normal) < 0.0;
    //     let normal = match front_face {
    //         true => outward_normal,
    //         false => -1.0 * outward_normal,
    //     };
    // }
}

trait Hittable {
    fn hit(ray: &Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> bool;
}
