mod img;

use img::*;

fn main() {
    let mut image = Image::new(800, 2.0 / 1.0);
    _generate_test_image(&mut image);
    image.write();
}
