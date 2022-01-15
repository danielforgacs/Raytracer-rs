use std::io::Write;
use std::fs::File;

type PixelVec = Vec<Vec<Pixel>>;
type PxChanType = f32;

#[derive(Clone, Copy)]
pub struct Pixel {
    r: PxChanType,
    g: PxChanType,
    b: PxChanType,
}

pub struct Image {
    width: usize,
    height: usize,
    pixels: PixelVec,
}

impl Image {
    pub fn new(width: usize, height: usize, pixels: Option<PixelVec>) -> Self {
        let pixels = match pixels {
            Some(pixels) => pixels,
            None => vec![vec![Pixel::new((0.1, 0.2, 0.3)); width]; height]
        };
        Self { width, height, pixels }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn set_pixel(&mut self, r: PxChanType, g: PxChanType, b: PxChanType, x: usize, y: usize) {
        self.pixels[x][y].set_rgb(r, g, b);
    }

    pub fn save_ppm(&self) {
        let mut image_file_content = String::new();
        image_file_content.push_str("P3");
        image_file_content.push_str(&format!("\n{} {}", self.width, self.height));
        image_file_content.push_str("\n256");
        for y in 0..self.height {
            for x in 0..self.width {
                let r = self.pixels[y][x].r;
                let r = (r * 255.9) as u8;
                let g = self.pixels[y][x].g;
                let g = (g * 255.9) as u8;
                let b = self.pixels[y][x].b;
                let b = (b * 255.9) as u8;
                image_file_content.push_str(&format!("\n{} {} {}", r, g, b));
            }
        }
        File::create("render.ppm")
            .unwrap()
            .write_all(image_file_content.as_bytes())
            .unwrap();

    }
}

impl Pixel {
    fn new(rgb: (f32, f32, f32)) -> Self {
        Self {
            r: rgb.0,
            g: rgb.1,
            b: rgb.2,
        }
    }

    pub fn set_rgb(&mut self, r: PxChanType, g: PxChanType, b: PxChanType) {
        self.r = r;
        self.g = g;
        self.b = b;
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gen_test_image() {
        let mut image = Image::new(5, 3, None);
        for y in 0..image.get_height() {
            for x in 0..image.get_width() {
                let r = x as f32 / (image.get_width() - 1) as f32;
                let b = y as f32 / (image.get_height() - 1) as f32;
                println!("{:<15} {}", r, b);
                image.set_pixel(r, 0.0, b, y, x);
            }
        }
        image.save_ppm();
    }
}
