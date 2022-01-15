use crate::vec3;

struct Ray {
    origin: vec3::Point3,
    direction: vec3::Vec3,
}

impl Ray {
    fn new(origin: vec3::Point3, direction: vec3::Vec3) -> Self {
        Self {
            origin,
            direction,
        }
    }
}
