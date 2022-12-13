use glam::Vec3;

pub trait Curve {
    fn eval(&self, t: f32) -> Vec3;

    fn point_at(&self, t: f32) -> Vec3 {
        self.eval(t)
    }
}
