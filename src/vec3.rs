use std::ops;

pub type Point3 = Vec3;
pub type Colour = Vec3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn set_xyz(mut self, x: f64, y: f64, z: f64) -> Self {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }

    pub fn white(mut self) -> Self {
        self.x = 1.0;
        self.y = 1.0;
        self.z = 1.0;
        self
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_z(&self) -> f64 {
        self.z
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.get_x().powi(2)
        + self.get_y().powi(2)
        + self.get_z().powi(2)
    }
}

pub fn dot(vec_l: &Vec3, vec_r: &Vec3) -> f64 {
    vec_l.get_x() * vec_r.get_x()
    + vec_l.get_y() * vec_r.get_y()
    + vec_l.get_z() * vec_r.get_z()
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self::Output::new()
            .set_xyz(
                rhs.get_x() * self,
                rhs.get_y() * self,
                rhs.get_z() * self,
            )
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output::new().set_xyz(
            self.get_x() / rhs,
            self.get_y() / rhs,
            self.get_z() / rhs,
        )
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output::new().set_xyz(
            self.get_x() + rhs.get_x(),
            self.get_y() + rhs.get_y(),
            self.get_z() + rhs.get_z(),
        )
    }

}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output::new().set_xyz(
            self.get_x() - rhs.get_x(),
            self.get_y() - rhs.get_y(),
            self.get_z() - rhs.get_z(),
        )
    }

}

pub fn unit_vector(vec: &Vec3) -> Vec3 {
    *vec / vec.length()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec3_constructor() {
        let vec = Vec3::new().set_xyz(1.0, 2.0, 3.0);
        assert_eq!(vec.get_x(), 1.0);
        assert_eq!(vec.get_y(), 2.0);
        assert_eq!(vec.get_z(), 3.0);
    }

    #[test]
    fn scalar_mult_vec() {
        let vector = Vec3::new().set_xyz(1.1, 2.2, 3.3);
        let scalar: f64 = 2.2;
        let expected = Vec3::new().set_xyz(
            1.1 * scalar,
            2.2 * scalar,
            3.3 * scalar,
        );
        assert_eq!(scalar * vector, expected);
    }

    #[test]
    fn vector_div_scalar() {
        let vector = Vec3::new().set_xyz(4.4, 6.6, 8.8);
        let scalar: f64 = 2.2;
        let expected = Vec3::new().set_xyz(
            4.4 / scalar,
            6.6 / scalar,
            8.8 / scalar,
        );
        assert_eq!(vector / scalar, expected);
    }

    #[test]
    fn vectod_add_vector() {
        assert_eq!(
            Colour::new().set_xyz(1.1, 2.2, 3.3) + Colour::new().set_xyz(0.5, 0.7, 1.0),
            Colour::new().set_xyz(1.1 + 0.5, 2.2 + 0.7, 3.3 + 1.0),
        );
    }

    #[test]
    fn vectod_sub_vector() {
        assert_eq!(
            Colour::new().set_xyz(1.1, 2.2, 3.3) - Colour::new().set_xyz(0.5, 0.7, 1.0),
            Colour::new().set_xyz(1.1 - 0.5, 2.2 - 0.7, 3.3 - 1.0),
        );
    }

    #[test]
    fn dot_product() {
        let vec1 = Vec3::new().set_xyz(0.1, 0.2, 0.3);
        let vec2 = Vec3::new().set_xyz(1.1, 2.2, 3.3);
        assert_eq!(dot(&vec1, &vec2),
            0.1 * 1.1 + 0.2 * 2.2 + 0.3 * 3.3
        );
    }
}
