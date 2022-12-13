use glam::Vec3;

use super::Curve;

pub struct Line {
    pub from: Vec3,
    pub to: Vec3,
}

impl Line {
    pub fn new(from: Vec3, to: Vec3) -> Self {
        Self { from, to }
    }
}

impl Curve for Line {
    fn eval(&self, t: f32) -> Vec3 {
        self.from.lerp(self.to, t)
    }
}
