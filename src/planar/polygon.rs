use glam::Vec3;
use std::f32::consts;

use crate::{lines::LineStrip, solid::Mesh};

pub struct Polygon {
    verts: Vec<Vec3>,
}

impl Polygon {
    pub fn new(verts: Vec<Vec3>) -> Self {
        Self { verts }
    }

    pub fn new_regular(radius: f32, sides: usize) -> Self {
        debug_assert!(sides > 2, "RegularPolygon requires at least 3 sides.");

        let mut verts: Vec<Vec3> = Vec::with_capacity(sides);

        let step = consts::TAU / sides as f32;
        for i in 0..sides {
            let theta = consts::FRAC_PI_2 - i as f32 * step;
            let (sin, cos) = theta.sin_cos();

            verts.push(Vec3::new(cos * radius, sin * radius, 0.0));
        }

        Self::new(verts)
    }
}

impl From<Polygon> for LineStrip {
    fn from(p: Polygon) -> Self {
        LineStrip::new(p.verts)
    }
}

impl From<Polygon> for Mesh {
    fn from(p: Polygon) -> Self {
        // yay triangulate!
        todo!()
    }
}
