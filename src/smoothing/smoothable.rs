// a lerp abstraction
use bevy_inspector_egui::Inspectable;

#[derive(Inspectable, Debug, Copy, Clone, PartialEq)]
pub enum State {
    Running,
    Finished,
}

pub trait Smoothable {
    fn tol_equals(&self, other: &Self, tolerance: f32) -> bool;
    fn lerp(&self, other: &Self, t: f32) -> Self;
    fn add(self, rhs: Self) -> Self;
    fn adds(self, rhs: f32) -> Self;
    fn mul(self, rhs: f32) -> Self;
    fn add_clamped(self, rhs: Self, min: Self, max: Self) -> Self;
    fn get(self) -> f32;
    fn is_negative(self) -> bool;
}

impl Smoothable for f32 {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        crate::math::lerp(*self, *other, t)
    }

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }

    fn adds(self, rhs: f32) -> Self {
        self + rhs
    }

    fn get(self) -> f32 {
        self
    }

    fn add_clamped(self, rhs: Self, min: Self, max: Self) -> Self {
        Self::clamp(self + rhs, min, max)
    }

    fn mul(self, rhs: f32) -> Self {
        self * rhs
    }

    fn tol_equals(&self, other: &Self, tolerance: f32) -> bool {
        if (self - other).abs() < tolerance {
            true
        } else {
            false
        }
    }

    fn is_negative(self) -> bool {
        if self < 0.0 {
            true
        } else {
            false
        }
    }
}

// impl Smoothable for Vec2 {
//     fn tol_equals(&self, other: &Self, tolerance: f32) -> bool {
//         if (self.x - other.x).abs() < tolerance &&
//            (self.y - other.y).abs() < tolerance {
//             true
//         } else {
//             false
//         }
//     }

//     fn lerp(&self, other: &Self, t: f32) -> Self {
//         Self::lerp(*self, *other, t)
//     }

//     fn add(self, rhs: Self) -> Self {
//         self + rhs
//     }

//     fn mul(self, rhs: f32) -> Self {
//         self * rhs
//     }
// }

// impl Smoothable for Vec3 {
//     fn tol_equals(&self, other: &Self, tolerance: f32) -> bool {
//         if (self.x - other.x).abs() < tolerance &&
//            (self.y - other.y).abs() < tolerance &&
//            (self.z - other.z).abs() < tolerance {
//             true
//         } else {
//             false
//         }
//     }

//     fn lerp(&self, other: &Self, t: f32) -> Self {
//         Self::lerp(*self, *other, t)
//     }

//     fn add(self, rhs: Self) -> Self {
//         self + rhs
//     }

//     fn mul(self, rhs: f32) -> Self {
//         self * rhs
//     }
// }
