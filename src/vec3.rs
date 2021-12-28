use std::ops::*;

pub type Colour = Vec3;
pub type Point3 = Vec3;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        (self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }

    pub fn black() -> Self {
        Self { e: [0.0, 0.0, 0.0 ]}
    }

    pub fn white() -> Self {
        Self { e: [1.0, 1.0, 1.0 ]}
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z(),
        )
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.x() - rhs.x(),
            self.y() - rhs.y(),
            self.z() - rhs.z(),
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs,
        )
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(
            self.x() / rhs,
            self.y() / rhs,
            self.z() / rhs,
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
