mod ppm;
mod ray {
    pub mod point;
    pub mod camera;
    pub mod vector;
}

use crate::ppm::Ppm;
use crate::ppm::pixel::Pixel;
use std::fs::File;
use std::io::Write;

fn main() {

    let mut ppm = Ppm::new("P3", 255);

    let width = 800_u32;
    let height = 800_u32;

    for row in 0..width {
        ppm.new_row();
        for col in 0..height {
            let r= (row as f64/width as f64)*255.0;
            let g= (col as f64/height as f64)*255.0;
            let b = 0.25*255.0;
            let pixel = Pixel(r as u8, g as u8, b as u8);
            ppm.add_pixel(pixel);
        }
    }

    let mut file = File::create("test.ppm").unwrap();
    file.write_all(ppm.render().as_bytes()).unwrap();
}


