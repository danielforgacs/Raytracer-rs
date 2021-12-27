use std::ops::*;
#[derive(Debug, PartialEq)]
#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(
            self.e[0] * rhs,
            self.e[1] * rhs,
            self.e[2] * rhs,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec3_new() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.e[0], 1.0);
        assert_eq!(v.e[1], 2.0);
        assert_eq!(v.e[2], 3.0);
    }

    #[test]
    fn vec3_add_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(11.0, 22.0, 33.0);
        assert_eq!(v1 + v2, Vec3::new(12.0, 24.0, 36.0));
    }

    #[test]
    fn vec3_mul_f64() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v * 3.3, Vec3::new(1.0 * 3.3, 2.0 * 3.3, 3.0 * 3.3));
    }
}
