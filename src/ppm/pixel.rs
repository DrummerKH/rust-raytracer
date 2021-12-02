use std::fmt::{Display, Formatter};

pub struct Pixel(pub u8, pub u8, pub u8);

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{} {} {}", self.0, self.1, self.0).as_str())
    }
}
