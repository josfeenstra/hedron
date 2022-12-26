use glam::Vec3;

use crate::algos::line_line_2d;

use super::Curve;

#[derive(Clone)]
pub struct Line {
    pub from: Vec3,
    pub to: Vec3,
}

impl Line {
    pub const NAN: Line = Line::new(Vec3::NAN, Vec3::NAN);

    pub const fn new(from: Vec3, to: Vec3) -> Self {
        Self { from, to }
    }

    pub fn intersect_2D(&self, other: &Self) -> Vec3 {
        let (x, y) = line_line_2d(
            self.from.x,
            self.from.y,
            self.to.x,
            self.to.y,
            other.from.x,
            other.from.y,
            other.to.x,
            other.to.y,
        );
        Vec3::new(x, y, 0.0)
    }
}

impl Curve for Line {
    fn eval(&self, t: f32) -> Vec3 {
        self.from.lerp(self.to, t)
    }
}

#[cfg(test)]
mod test {
    use crate::algos::line_line_2d;

    #[test]
    fn test_line_x() {
        assert_eq!(
            (0.0, 0.0),
            line_line_2d(0.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 0.0)
        );
        assert_eq!(
            (2.0, 2.0),
            line_line_2d(0.0, 0.0, 2.0, 2.0, 2.0, 2.0, 2.0, 3.0)
        );
    }
}
