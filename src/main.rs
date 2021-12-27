mod img;
mod vec3;
mod ray;

use img::*;
use vec3::*;
use ray::*;

fn main() {
    let mut image = Image::new(800, 2.0 / 1.0);
    image.write();
}
