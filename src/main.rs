mod img;
mod vec3;

use img::*;
use vec3::*;

fn main() {
    let mut image = Image::new(800, 2.0 / 1.0);
    image.write();
}
