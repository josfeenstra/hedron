use crate::kernel::Vec3;

pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Triangle {
    pub fn new<V: Into<Vec3>>(a: V, b: V, c: V) -> Self {
        Triangle {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        }
    }

    pub fn normal(&self) -> Vec3 {
        (self.b - self.a).cross(self.c - self.a).normalize()
    }
}
