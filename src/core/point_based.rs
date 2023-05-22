// TODO create a trait, allowing some struct to be modified by its vertices.
// it just needs to expose a Iter<&mut Vec3>

use super::Geometry;
use crate::kernel::{fxx, Quat, Vec3};

/// If some geometry is ultimately defined in terms of points,
/// A whole set of common functionalities can be used to transform said geometry.
/// it can also be used to
pub trait PointBased: Sized + Geometry {
    // smooth
    // scramble
    // scale from

    // fn mutate_points<'a>(&'a mut self) -> &'a mut Vec<Vec3>;
    fn mutate_points(&mut self) -> Vec<&mut Vec3>;

    /// scale from a certain position.
    fn scale_from(mut self, pos: Vec3, factor: fxx) -> Self {
        for v in self.mutate_points() {
            *v = Vec3::lerp(pos, *v, factor);
        }
        self
    }

    // /// scale from a Pose, by applying only the scale element from the pose
    // fn scale_nu(mut self, pose: &Pose) -> Self {
    //     let pos_inv = pose.pos * -1.0;
    //     let rot_inv = pose.rot.inverse();
    //     for v in self.mutate_points() {
    //         let v_norm = rot_inv * (*v + pos_inv); // move and rotate thte point to pose space
    //         let v_scaled = v_norm * pose.scale; // then scale it
    //         *v = (pose.rot * v_scaled) + pose.pos; // and translate it back
    //     }
    //     self
    // }
}

impl<T: PointBased> Geometry for T {
    fn mv(mut self, mv: Vec3) -> Self {
        for v in self.mutate_points() {
            *v += mv;
        }
        self
    }

    fn rot(mut self, rot: &Quat) -> Self {
        for v in self.mutate_points() {
            *v = *rot * *v;
        }
        self
    }

    fn scale(mut self, scale: Vec3) -> Self {
        for v in self.mutate_points() {
            *v = scale * *v;
        }
        self
    }

    fn scale_u(mut self, scale: fxx) -> Self {
        for v in self.mutate_points() {
            v.x *= scale;
            v.y *= scale;
            v.z *= scale;
        }
        self
    }
}
