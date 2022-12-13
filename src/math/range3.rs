use std::ops::Range;

pub struct Range3 {
    pub x: Range<f32>,
    pub y: Range<f32>,
    pub z: Range<f32>,
}

impl Range3 {
    pub fn new(x: Range<f32>, y: Range<f32>, z: Range<f32>) -> Self {
        Self { x, y, z }
    }

    pub fn from_radius(r: f32) -> Self {
        Self::new(-r..r, -r..r, -r..r)
    }
}
