use crate::kernel::{fxx, Vec3};

pub trait Curve {
    fn eval(&self, t: fxx) -> Vec3;

    fn point_at(&self, t: fxx) -> Vec3 {
        self.eval(t)
    }
}
