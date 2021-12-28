use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(a: &Vec3, b: &Vec3) -> Self {
        Self {origin: *a, direction: *b}
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ray_new() {
        let v1 = Vec3::new(1.1, 2.2, 3.3);
        let v2 = Vec3::new(11.1, 22.2, 33.3);
        let r = Ray::new(&v1, &v2);
        assert_eq!(r.origin, v1);
        assert_eq!(r.direction, v2);
    }

    #[test]
    fn ray_point_at_param() {
        let v1 = Vec3::new(1.1, 2.2, 3.3);
        let v2 = Vec3::new(11.1, 22.2, 33.3);
        let t = 4.4;
        let r = Ray::new(&v1, &v2);
        assert_eq!(r.point_at_parameter(t), Vec3::new(1.1 + 11.1 * t, 2.2 + 22.2 * t, 3.3 + 33.3 * t));
    }
}
