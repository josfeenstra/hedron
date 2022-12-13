use crate::core::Geometry;
use glam::{Quat, Vec3};

pub struct Sphere {
    pub pos: Vec3,
    pub rad: f32,
}

impl Geometry for Sphere {
    fn mv(mut self, mv: &Vec3) -> Self {
        self.pos += *mv;
        self
    }

    fn rot(mut self, rot: &Quat) -> Self {
        self.pos = *rot * self.pos;
        self
    }

    fn scale(mut self, scale: &Vec3) -> Self {
        self.rad *= scale.x;
        self
    }

    fn scale_u(mut self, scale: f32) -> Self {
        self.rad *= scale;
        self
    }
}

impl Sphere {}
