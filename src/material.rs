#[derive(Debug)]
pub enum Material {
    Lambert {},
    Metal {},
    Dielectric {},
}

impl Default for Material {
    fn default() -> Self {
        Self::Lambert {}
    }
}
