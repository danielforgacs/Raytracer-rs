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

    fn get_direction(&self) -> vec3::Vec3 {
        self.direction
    }
}

// fn ray_colour(ray: &Ray) -> vec3::Colour {
//     let unit_direction = vec3::unit_vector(&ray.get_direction());
// }
