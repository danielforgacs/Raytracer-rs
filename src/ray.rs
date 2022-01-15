use crate::vec3;

pub struct Ray {
    origin: vec3::Point3,
    direction: vec3::Vec3,
}

impl Ray {
    pub fn new(origin: vec3::Point3, direction: vec3::Vec3) -> Self {
        Self {
            origin,
            direction,
        }
    }

    fn get_origin(&self) -> vec3::Vec3 {
        self.origin
    }

    fn get_direction(&self) -> vec3::Vec3 {
        self.direction
    }

    fn at(&self, t: f64) -> vec3::Point3 {
        self.origin + t * self.get_direction()
    }
}

pub fn ray_colour(ray: &Ray) -> vec3::Colour {
    let t = hit_sphere(
        &vec3::Point3::new().set_xyz(0.0, 0.0, -1.0),
        0.5,
        &ray,
    );
    if t > 0.0 {
        let Normal = vec3::unit_vector(
            &(ray.at(t) - vec3::Vec3::new().set_xyz(0.0, 0.0, -1.0))
        );
        return 0.5 * vec3::Colour::new().set_xyz(
            Normal.get_x() + 1.0,
            Normal.get_y() + 1.0,
            Normal.get_z() + 1.0,
        );
    }
    let unit_direction = vec3::unit_vector(&ray.get_direction());
    let t = 0.5 * (unit_direction.get_y() + 1.0);
    (1.0 - t)
        * vec3::Colour::new().set_xyz(1.0, 1.0, 1.0)
        + (t * vec3::Colour::new().set_xyz(0.5, 0.7, 1.0))
}

pub fn hit_sphere(center: &vec3::Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.get_origin() - *center;
    let a = vec3::dot(&ray.get_direction(), &ray.get_direction());
    let b = 2.0 * vec3::dot(&oc, &ray.get_direction());
    let c = vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}
