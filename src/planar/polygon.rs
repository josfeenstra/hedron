use crate::algos::signed_volume;
use crate::core::{Plane, Pose};
use crate::kernel::{fxx, Vec3, FRAC_PI_2, TAU, EPSILON, Vec2};

use crate::lines::Ray;
use crate::{
    core::PointBased,
    lines::{Line, LineStrip},
    pts::Vectors,
    solid::Mesh,
    util::{self, iter_pair_ids},
};
use crate::util::{iter_pairs, iter_triplet_ids, iter_triplets};

#[derive(Debug, Clone)]
pub struct Polygon {
    pub verts: Vec<Vec3>,
}

// TODO create support for Polylines (non-closed polygons)
impl Polygon {
    pub fn new(verts: Vec<Vec3>) -> Self {
        Self { verts }
    }

    pub fn new_regular(radius: fxx, sides: usize) -> Self {
        let mut verts: Vec<Vec3> = Vec::with_capacity(sides);

        let step = TAU / sides as fxx;
        for i in 0..sides {
            let theta = FRAC_PI_2 - i as fxx * step;
            let (sin, cos) = theta.sin_cos();

            verts.push(Vec3::new(cos * radius, sin * radius, 0.0));
        }

        Self::new(verts)
    }

    #[inline]
    pub fn center(&self) -> Vec3 {
        Vectors::average(&self.verts)
    }

    /// flip the polygon by reversing the order of the vertices
    #[inline]
    #[must_use]
    pub fn flip(mut self) -> Self {
        self.verts.reverse();
        self
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

    pub fn simple_shrink(mut self, distance: fxx) -> Self {
        let center = Vectors::average(&self.verts);
        for v in self.verts.iter_mut() {
            *v = *v - (*v - center).normalize() * distance;
        }
        self
    }

    /// offset the polygon by pretending its 2D, offsetting all line segments,
    /// and calculating the intersection points in an inefficient manner :)
    pub fn offset(mut self, normal: Vec3, distance: fxx) -> Self {
        let center = Vectors::average(&self.verts);

        let count = self.verts.len();
        // let mut line_offsets = vec![Vec3::ZERO; count];
        let mut offset_lines = vec![Line::NAN; count];

        // per line in the polyline, calculate the offset vector
        for (ia, ib) in iter_pair_ids(count) {
            let (a, b) = (self.verts[ia], self.verts[ib]);
            let offset_vec = (b - a).cross(normal).normalize() * -distance;
            // line_offsets[ia] = offset_vec;
            offset_lines[ia] = Line::new(a + offset_vec, b + offset_vec);
        }

        // vert I is intersection of line I and line I-1
        for (i_vert_min_one, i_vert) in iter_pair_ids(count) {
            let vert = self.verts[i_vert];

            let one = &offset_lines[i_vert];
            let two = &offset_lines[i_vert_min_one];

            // build two lines representing the lines + offset
            // TODO we dont need to store the line offsets, we can store the entire line!

            // let l_one = Line::new(vert + one, vert + one + one.cross(normal));
            // let l_two = Line::new(vert + two, vert + two + two.cross(normal));

            // TODO INTERSECT IN THE NORMAL PLANE BY PLANARIZING THE 3D LINES THEN DOING THIS PROPERLY
            let intersection =
                if (one.to - one.from).angle_between(two.to - two.from) < EPSILON {
                    one.from
                } else {
                    one.intersect_2d(&two)
                };

            self.verts[i_vert].x = intersection.x;
            self.verts[i_vert].y = intersection.y;
            // keep z the same
        }
        self
    }
    
    // pretend the polygon is 2d
    pub fn signed_area_2d(&self) -> fxx {
        let sum = iter_pairs(&self.verts).fold(0.0, |sum, (a, b)| sum + (b.x - a.x) * (b.y + a.y));
        sum / 2.0
    }

    pub fn estimate_pose(&self) -> Pose {
        if self.verts.len() < 3 {
            panic!("polygon invalid");
        } 
        let center = self.center();
        let normal = self.average_normal();
        let first = self.verts.first().unwrap();
        let axis = *first - center;
        Pose::from_pos(center).looking_at(normal, axis)
    }

    /// calculate the normalized vertices using the estimated pose
    pub fn planarized_verts(&self) -> Vec<Vec2> {
        let pose = self.estimate_pose();
        self.verts.iter().map(|vert| pose.transform_point_inv(*vert).truncate()).collect()
    }

    pub fn signed_area_planar(&self) -> fxx {
        let plan_verts = self.planarized_verts();
        let sum = iter_pairs(&plan_verts).fold(0.0, |sum, (a, b)| sum + (b.x - a.x) * (b.y + a.y));
        // dbg!(plan_verts);
        sum / 2.0
    }

    /// Intersect ray
    /// Only works for convex polygons
    pub fn intersect_ray_where(&self, ray: &Ray) -> Option<fxx> {
        if self.verts.len() < 3 {
            return None;
        }
        if self.intersect_ray(ray) {
            return None;
        }
        let t = ray.x_plane(&Plane::from_pts(self.verts[0], self.verts[1], self.verts[2]));
        Some(t)
    }

    pub fn intersect_ray(&self, ray: &Ray) -> bool {
        for (a, b) in iter_pairs(&self.verts) {
            if signed_volume(ray.origin, ray.origin + ray.normal, *a, *b) > 0.0 {
                return false;
            }
        }
        true
    }



    /// there are better ways of doing this, 
    /// TODO: principle component analysis ? overkill ?
    pub fn average_normal(&self) -> Vec3 {
        assert!(self.verts.len() >= 3);
        let center = self.center();
        let mut normals = Vec::new();
        for (a, b) in iter_pairs(&self.verts) {
            normals.push((*a - center).cross(*b - center));
        }

        Vectors::average(&normals).normalize()
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

#[cfg(test)]
mod tests {
    use crate::kernel::Vec3;

    use super::Polygon;

    #[test]
    pub fn test_offset() {
        let mut polygon = Polygon::new(vec![
            Vec3::new(0., 0., 0.),
            Vec3::new(1., 0., 0.),
            Vec3::new(1., 1., 0.),
            Vec3::new(0., 1., 0.),
        ]);
        polygon = polygon.offset(Vec3::Z, 0.25);
        println!("{polygon:?}");
    }
}
