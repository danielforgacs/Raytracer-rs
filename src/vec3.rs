type CoordType = f32;
pub type Point3 = Vec3;

pub struct Vec3 {
    x: CoordType,
    y: CoordType,
    z: CoordType,
}

impl Vec3 {
    fn new(x: CoordType, y: CoordType, z: CoordType) -> Self {
        Self { x, y, z }
    }

    fn get_x(&self) -> CoordType {
        self.x
    }

    fn get_y(&self) -> CoordType {
        self.y
    }

    fn get_z(&self) -> CoordType {
        self.z
    }
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
}
