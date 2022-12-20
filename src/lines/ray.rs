use crate::core::Plane;
use glam::*;

pub struct Ray {
    pub origin: Vec3,
    pub normal: Vec3,
}

impl Ray {
    /// NOTE: this expects a normalized normal!!!
    pub fn new(origin: Vec3, normal: Vec3) -> Self {
        Self { origin, normal }
    }

    /// shorthand for getting a ray from a point to another point
    pub fn new_from_points(origin: Vec3, to: Vec3) -> Self {
        Self {
            origin,
            normal: (to - origin).normalize(),
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.normal * t
    }

    /// intersect with a plane
    /// returns the t parameter on this ray
    pub fn x_plane(&self, plane: &Plane) -> f32 {
        // let t = plane.
        let pn = plane.normal().truncate();
        return -(self.origin.dot(pn) + plane.d()) / self.normal.dot(pn);
    }
}
