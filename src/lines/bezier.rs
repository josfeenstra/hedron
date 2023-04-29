use crate::kernel::{fxx, Quat, Vec3};

use crate::{core::Geometry, math::bernstein};

pub struct Bezier {
    pub verts: Vec<Vec3>,
}

/// A bezier curve!
impl Bezier {
    pub fn new(verts: Vec<Vec3>) -> Bezier {
        Bezier { verts }
    }

    pub fn equalize_degrees(mut curves: Vec<Bezier>) -> Vec<Bezier> {
        let max_degree = curves.iter().map(|b| b.degree()).max().unwrap_or(0);

        for curve in curves.iter_mut() {
            while curve.degree() < max_degree {
                curve.increase_degree();
            }
        }

        curves
    }

    /// Get the degree
    pub fn degree(&self) -> usize {
        self.verts.len() - 1
    }

    pub fn hodograph(&self) -> Bezier {
        let count = self.verts.len() - 1;
        let mut hodo_verts = Vec::with_capacity(count);
        for i in 0..count {
            hodo_verts.push(self.verts[i + 1] - self.verts[i]);
        }
        Bezier::new(hodo_verts)
    }

    /// increase degree by one.
    /// copy first and last point,
    /// and interpolate in-betweens
    pub fn increase_degree(&self) -> Bezier {
        let n = self.degree();
        let mut verts = Vec::with_capacity(n + 2);

        verts.push(self.verts[0]);
        for i in 1..n + 1 {
            let pa = self.verts.get(i - 1).unwrap();
            let pb = self.verts.get(i).unwrap();
            let sa = i as fxx / (n + 1) as fxx;
            let sb = 1.0 - sa;
            let q = *pa * sa + *pb * sb;
            verts.push(q);
        }

        verts.push(self.verts[self.verts.len() - 1]);

        // create a new curve from it
        Bezier::new(verts)
    }

    pub fn split(&self, _t: fxx) -> Bezier {
        let _size = self.degree() + 1;
        // let tri =
        todo!();
    }

    pub fn extend(&self, _t: fxx) -> Bezier {
        todo!();
    }

    pub fn point_at(&self, t: fxx) -> Vec3 {
        let degree = self.degree();
        let mut p = Vec3::ZERO;
        for (i, vert) in self.verts.iter().enumerate() {
            p += *vert * bernstein(t, i, degree);
        }
        p
    }

    pub fn tangent_at(&self, t: fxx) -> Vec3 {
        // NOTE: not extremely efficient
        self.hodograph().point_at(t)
    }

    pub fn normal_at(&self, t: fxx, up: Vec3) -> Vec3 {
        self.tangent_at(t).cross(up)
    }

    pub fn to_polyline(&self, segments: usize) -> Vec<Vec3> {
        let count = segments + 1;
        let mut verts = Vec::with_capacity(count);
        for i in 0..count {
            let t = i as fxx / count as fxx;
            verts.push(self.point_at(t));
        }
        verts
    }
}

// impl Curve for Bezier {
//     fn eval(&self, t: fxx) -> Vec3 {
//         Vec3::ZERO
//     }
// }

impl Geometry for Bezier {
    fn mv(mut self, mv: Vec3) -> Self {
        for v in self.verts.iter_mut() {
            *v += mv;
        }
        self
    }

    fn rot(mut self, rot: &Quat) -> Self {
        for v in self.verts.iter_mut() {
            *v = *rot * *v;
        }
        self
    }

    fn scale(mut self, scale: Vec3) -> Self {
        for v in self.verts.iter_mut() {
            *v = scale * *v;
        }
        self
    }

    fn scale_u(mut self, scale: fxx) -> Self {
        for v in self.verts.iter_mut() {
            v.x *= scale;
            v.y *= scale;
            v.z *= scale;
        }
        self
    }
}
