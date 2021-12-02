use crate::ppm::Ppm;
use crate::ppm::pixel::Pixel;
use std::fs::File;
use std::io::Write;

mod ppm;

fn main() {

    let mut ppm = Ppm::new("P3", 255);

    for _ in 0..800 {
        for _ in 0..800 {
           ppm.add_pixel(Pixel(10, 100, 10));
        }
        ppm.terminate_row();
    }

    let mut file = File::create("test.ppm").unwrap();
    file.write_all(ppm.render().as_bytes()).unwrap();
}


