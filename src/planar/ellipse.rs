use std::cmp::Ordering;
use crate::kernel::{fxx, Vec2, vec2};

pub struct Ellipse {
    center: Vec2,
    rx: fxx, // a
    ry: fxx, // b
}

impl Ellipse {
    pub fn new(center: Vec2, rx: fxx, ry: fxx) -> Self {
        Self {center, rx, ry}
    }

    pub fn new_xyab(x: fxx, y: fxx, rx: fxx, ry: fxx) -> Self {
        Self::new(vec2(x, y), rx, ry)
    }

    /// evaluate a point using the general formula of an ellipse
    pub fn eval(&self, pt: Vec2) -> fxx {
        let n = pt - self.center;
        (n.x * n.x) / (self.rx * self.rx) + (n.y * n.y) / (self.ry * self.ry) 
    }

    /// Test if the point falls within the radii of this ellipse
    /// Returns an ordering: 
    /// Less -> within ellipse, 
    /// Equal -> touching ellipse, 
    /// Greater -> outside ellipse 
    pub fn includes_point(&self, pt: Vec2) -> Ordering {
        fxx::total_cmp(&self.eval(pt), &1.0_f64)
    }
}