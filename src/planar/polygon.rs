use glam::Vec3;
use std::f32::consts;

use crate::{core::PointBased, lines::LineStrip, math, pts::Vectors, solid::Mesh, util};

pub struct Polygon {
    verts: Vec<Vec3>,
}

impl Polygon {
    pub fn new(verts: Vec<Vec3>) -> Self {
        Self { verts }
    }

    pub fn new_regular(radius: f32, sides: usize) -> Self {
        let mut verts: Vec<Vec3> = Vec::with_capacity(sides);

        let step = consts::TAU / sides as f32;
        for i in 0..sides {
            let theta = consts::FRAC_PI_2 - i as f32 * step;
            let (sin, cos) = theta.sin_cos();

            verts.push(Vec3::new(cos * radius, sin * radius, 0.0));
        }

        Self::new(verts)
    }

    /// Simple triangulate using a fan of triangles, and the center of the vertex
    /// This will work for convex polygons. concave polygons may become weird
    pub fn triangulate_naive(&self) -> Mesh {
        let mut mesh = Mesh::default();

        let count = self.verts.len(); // the center will end up at this vert id
        for (a, b) in util::iter_pair_ids(count) {
            mesh.verts.push(self.verts[a]);
            mesh.tri.append(&mut vec![a, b, count]);
        }
        let center = Vectors::average(&self.verts);
        mesh.verts.push(center);

        mesh
    }
}

impl PointBased for Polygon {
    fn mutate_points<'a>(&'a mut self) -> Vec<&'a mut Vec3> {
        self.verts.iter_mut().collect()
    }
}

impl From<Polygon> for LineStrip {
    fn from(p: Polygon) -> Self {
        LineStrip::new(p.verts)
    }
}

impl From<Polygon> for Mesh {
    fn from(p: Polygon) -> Self {
        p.triangulate_naive()
    }
}
