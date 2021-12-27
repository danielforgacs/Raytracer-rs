struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn Vec3_new() {
        let v = Vec3::new(1.0, 2.0, 3.0);
    }
}
