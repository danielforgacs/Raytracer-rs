use std::ops;

type CoordType = f32;
pub type Point3 = Vec3;
pub type Colour = Vec3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    x: CoordType,
    y: CoordType,
    z: CoordType,
}

impl Vec3 {
    pub fn new(x: CoordType, y: CoordType, z: CoordType) -> Self {
        Self { x, y, z }
    }

    pub fn get_x(&self) -> CoordType {
        self.x
    }

    pub fn get_y(&self) -> CoordType {
        self.y
    }

    pub fn get_z(&self) -> CoordType {
        self.z
    }

    fn length(&self) -> CoordType {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> CoordType {
        self.get_x().powi(2)
        + self.get_y().powi(2)
        + self.get_z().powi(2)
    }
}

impl ops::Mul<Vec3> for CoordType {
    type Output = Vec3;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self::Output::new(
            rhs.get_x() * self,
            rhs.get_y() * self,
            rhs.get_z() * self,
        )
    }
}

impl ops::Div<CoordType> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: CoordType) -> Self::Output {
        Self::Output::new(
            self.get_x() / rhs,
            self.get_y() / rhs,
            self.get_z() / rhs,
        )
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(
            self.get_x() + rhs.get_x(),
            self.get_y() + rhs.get_y(),
            self.get_z() + rhs.get_z(),
        )
    }

}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(
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
        let vec = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(vec.get_x(), 1.0);
        assert_eq!(vec.get_y(), 2.0);
        assert_eq!(vec.get_z(), 3.0);
    }

    #[test]
    fn scalar_mult_vec() {
        let vector = Vec3::new(1.1, 2.2, 3.3);
        let scalar: CoordType = 2.2;
        let expected = Vec3::new(
            1.1 * scalar,
            2.2 * scalar,
            3.3 * scalar,
        );
        assert_eq!(scalar * vector, expected);
    }

    #[test]
    fn vector_div_scalar() {
        let vector = Vec3::new(4.4, 6.6, 8.8);
        let scalar: CoordType = 2.2;
        let expected = Vec3::new(
            4.4 / scalar,
            6.6 / scalar,
            8.8 / scalar,
        );
        assert_eq!(vector / scalar, expected);
    }

    #[test]
    fn vectod_add_vector() {
        assert_eq!(
            Colour::new(1.1, 2.2, 3.3) + Colour::new(0.5, 0.7, 1.0),
            Colour::new(1.1 + 0.5, 2.2 + 0.7, 3.3 + 1.0),
        );
    }

    #[test]
    fn vectod_sub_vector() {
        assert_eq!(
            Colour::new(1.1, 2.2, 3.3) - Colour::new(0.5, 0.7, 1.0),
            Colour::new(1.1 - 0.5, 2.2 - 0.7, 3.3 - 1.0),
        );
    }
}
