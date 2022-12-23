// TODO create a trait, allowing some struct to be modified by its vertices. 
// it just needs to expose a Iter<&mut Vec3>

use super::Geometry;
use glam::Vec3;

/// If some geometry is ultimately defined in terms of points, 
/// A whole set of common functionalities can be used to transform said geometry. 
/// it can also be used to 
pub trait PointBased {
    // smooth
    // scramble 
    // scale from

    // fn mutate_points<'a>(&'a mut self) -> &'a mut Vec<Vec3>;
    fn mutate_points<'a>(&'a mut self) -> Vec<&'a mut Vec3>;
}

impl<T: PointBased> Geometry for T {
     
    fn mv(mut self, mv: &Vec3) -> Self {
        for v in self.mutate_points() {
            *v = *v + *mv;
        }
        self
    }

    fn rot(mut self, rot: &glam::Quat) -> Self {
        for v in self.mutate_points() {
            *v = *rot * *v;
        }
        self
    }

    fn scale(mut self, scale: &Vec3) -> Self {
        for v in self.mutate_points() {
            *v = *scale * *v;
        }
        self
    }

    fn scale_u(mut self, scale: f32) -> Self {
        for v in self.mutate_points() {
            v.x *= scale;
            v.y *= scale;
            v.z *= scale;
        }
        self
    }
}

