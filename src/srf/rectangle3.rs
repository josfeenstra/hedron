use crate::{math::Range2, core::{Pose, PointBased, Geometry}};
use crate::kernel::{Vec3, vec3};

/// a rectangle in 3D space, defined by bounds and a range 2
pub struct Rectangle3 {
    pub pose: Pose,
    pub bounds: Range2,
}

impl Rectangle3 {

    pub fn new(pose: Pose, bounds: Range2) -> Self {
        Self { pose, bounds }
    }

    pub fn corners(&self) -> [Vec3; 4] {
        let xs = self.bounds.x.start;
        let xe = self.bounds.x.end;
        let ys = self.bounds.y.start;
        let ye = self.bounds.y.end;
        [
            self.pose.transform_point(vec3(xs, ys, 0.0)),
            self.pose.transform_point(vec3(xe, ys, 0.0)),
            self.pose.transform_point(vec3(xs, ye, 0.0)),
            self.pose.transform_point(vec3(xe, ye, 0.0)),
        ]
    }

}

impl Geometry for Rectangle3 {
    fn mv(self, mv: crate::kernel::Vec3) -> Self {
        self.pose.pos += mv;
        self
    }

    fn rot(self, rot: &crate::kernel::Quat) -> Self {
        self.pose.rot *= *rot;
        self
    }

    // this scales the bound, NOT the pose 
    fn scale(self, scale: crate::kernel::Vec3) -> Self {
        self.bounds.scale(scale.truncate());
        self
    }

    // this scales the bound, NOT the pose 
    fn scale_u(self, scale: crate::kernel::fxx) -> Self {
        self.bounds.scale_u(scale);
        self
    }
}