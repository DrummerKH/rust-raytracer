pub mod point;

use crate::ray::point::Point;

pub struct Ray {
    origin: Point,
    direction: Point,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Self{origin, direction}
    }

    pub fn at(self, spread: f32) -> Point {
        self.origin + self.direction* spread
    }
}

