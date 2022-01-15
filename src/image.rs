use std::io::Write;
use std::fs::File;

type PixelVec = Vec<Vec<Pixel>>;

#[derive(Clone, Copy)]
pub struct Pixel {
    r: f64,
    g: f64,
    b: f64,
}

pub struct Image {
    width: usize,
    height: usize,
    aspect_ratio: f64,
    pixels: PixelVec,
    file_name: String,
}

impl Image {
    pub fn new() -> Self {
        let width = 64;
        let height = 64;
        let pixels = vec![vec![Pixel::new((0.2, 0.15, 0.1)); width]; height];
        let file_name = String::from("render");
        Self {
            width,
            height,
            pixels,
            aspect_ratio: 1.0,
            file_name,
        }
    }

    pub fn set_width(mut self, value: usize) -> Self {
        self.width = value;
        self.pixels = vec![vec![Pixel::new((0.2, 0.15, 0.1)); value]; self.height];
        self
    }

    pub fn set_aspect_ratio(mut self, ratio: f64) -> Self {
        self.aspect_ratio = ratio;
        self.height = ((self.width as f64) / ratio) as usize;
        self.pixels = vec![vec![Pixel::new((0.2, 0.15, 0.1)); self.width]; self.height];
        self
    }

    pub fn set_height(mut self, value: usize) -> Self {
        self.height = value;
        self.pixels = vec![vec![Pixel::new((0.2, 0.15, 0.1)); self.width]; value];
        self
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn set_pixel(&mut self, r: f64, g: f64, b: f64, x: usize, y: usize) {
        self.pixels[y][x].set_rgb(r, g, b);
    }

    pub fn set_filename(mut self, name: String) -> Self {
        self.file_name = name;
        self
    }

    fn get_filename(&self) -> String {
        self.file_name.clone()
    }

    pub fn save_ppm(&self) {
        let mut image_file_content = String::new();
        image_file_content.push_str("P3");
        image_file_content.push_str(&format!("\n{} {}", self.width, self.height));
        image_file_content.push_str("\n255");
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let (r, g, b) = self.pixels[y][x].as_8bit();
                image_file_content.push_str(&format!("\n{} {} {}", r, g, b));
            }
        }
        let file_name = format!("{}.ppm", self.get_filename());
        File::create(file_name)
            .unwrap()
            .write_all(image_file_content.as_bytes())
            .unwrap();
    }

    pub fn get_aspect_ratio(&self) -> f64 {
        self.get_width() as f64 / self.get_height() as f64
    }
}

impl Pixel {
    fn new(rgb: (f64, f64, f64)) -> Self {
        Self {
            r: rgb.0,
            g: rgb.1,
            b: rgb.2,
        }
    }

    pub fn set_rgb(&mut self, r: f64, g: f64, b: f64) {
        self.r = r;
        self.g = g;
        self.b = b;
    }

    fn as_8bit(&self) -> (u8, u8, u8) {
        (
            (self.r * 255.9) as u8,
            (self.g * 255.9) as u8,
            (self.b * 255.9) as u8,
        )
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn gen_test_image() {
        let mut image = Image::new()
            .set_width(128)
            .set_height(128)
            .set_filename(String::from("test_gen_test_image"));
        for y in (0..image.get_height()).rev() {
            for x in 0..image.get_width() {
                let r = x as f64 / (image.get_width() - 1) as f64;
                let g = y as f64 / (image.get_height() - 1) as f64;
                println!("{:<15} {}", r, g);
                image.set_pixel(r, g, 0.25, x, y);
            }
        }
        image.save_ppm();
    }

    #[test]
    fn image_set_pixel_coordinates_work() {
        let mut image = Image::new()
            .set_width(60)
            .set_height(5)
            .set_filename(String::from("test_image_set_pixel_coordinates_work"));
        image.set_pixel(1.0, 0.0, 0.0, 50, 1);
        image.save_ppm();
    }

    #[test]
    fn image_builder_pattern() {
        let image = Image::new();
        let image = Image::new()
            .set_width(100);
        assert_eq!(image.get_width(), 100);
        let image = Image::new()
            .set_height(100)
            .set_width(200);
        assert_eq!(image.get_height(), 100);
        assert_eq!(image.get_width(), 200);
        let image = Image::new()
            .set_height(200)
            .set_width(300)
            .set_filename(String::from("blabla"));
        assert_eq!(image.get_height(), 200);
        assert_eq!(image.get_width(), 300);
        assert_eq!(image.get_filename(), String::from("blabla"));
    }
}
