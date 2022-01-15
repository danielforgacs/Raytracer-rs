type CoordType = f32;
pub type Point3 = Vec3;
pub type Colour = Vec3;

pub struct Vec3 {
    x: CoordType,
    y: CoordType,
    z: CoordType,
}

impl Vec3 {
    fn new(x: CoordType, y: CoordType, z: CoordType) -> Self {
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

// pub fn unit_vector(vec: Vec3) -> Vec3 {
//     vec / vec.length()
// }

// trait std::ops::Mul {
//     fn mul(&self, rhs: Vec3);
// }

// impl std::ops::Mul for CoordType {
//     fn mul(&self, rhs: Vec3) -> Vec3 {
//         Vec3::new(
//             rhs.get_x() * self,
//             rhs.get_y() * self,
//             rhs.get_z() * self,
//         )
//     }
// }

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
}
