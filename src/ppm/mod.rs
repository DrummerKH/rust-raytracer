pub mod pixel;
use pixel::Pixel;

pub struct Ppm {
    width: u32,
    height: u32,
    max_color: u8,
    matrix_data: Vec<Vec<Pixel>>,
    rendered_data: String,
}

impl Ppm {

    pub fn new(magic_number: &str, max_color: u8) -> Self {
        Ppm {
            max_color,
            width: 0,
            height: 0,
            matrix_data: Vec::new(),
            rendered_data: format!("{}\n", magic_number),
        }
    }

    pub fn add_pixel(&mut self, pixel: Pixel) {
        match self.last_row_mut() {
            Some(val) => val.push(pixel),
            None => self.new_row(),
        };
    }

    pub fn new_row(&mut self) {
        let last_len =  self.last_row().unwrap_or(&Vec::new()).len() as u32;

        if self.width < last_len {
            self.width = last_len;
        }

        self.matrix_data.push(Vec::new());
    }

    pub fn render(&mut self) -> &str {
        self.height = self.last_row().unwrap().len() as u32;
        self.rendered_data += &format!("{} {}\n{}\n", self.width, self.height, self.max_color).as_str();

        for row in &self.matrix_data {
            for pixel in row {
                self.rendered_data += &(pixel.to_string() + " ");
            }
            self.rendered_data += "\n";
        }

        &self.rendered_data
    }

    fn last_row_mut(&mut self) ->  Option<&mut Vec<Pixel>> {
        self.matrix_data.last_mut()
    }

    fn last_row(&self) ->  Option<&Vec<Pixel>> {
        self.matrix_data.last()
    }
}
