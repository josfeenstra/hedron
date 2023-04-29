// a lerp abstraction
use crate::kernel::fxx;
use bevy_inspector_egui::InspectorOptions;

#[derive(InspectorOptions, Debug, Copy, Clone, PartialEq)]
pub enum State {
    Running,
    Finished,
}

pub trait Smoothable {
    fn tol_equals(&self, other: &Self, tolerance: fxx) -> bool;
    fn lerp(&self, other: &Self, t: fxx) -> Self;
    fn add(self, rhs: Self) -> Self;
    fn adds(self, rhs: fxx) -> Self;
    fn mul(self, rhs: fxx) -> Self;
    fn add_clamped(self, rhs: Self, min: Self, max: Self) -> Self;
    fn get(self) -> fxx;
    fn is_negative(&self) -> bool;
}

impl Smoothable for fxx {
    fn lerp(&self, other: &Self, t: fxx) -> Self {
        crate::math::lerp(t, *self, *other)
    }

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }

    fn adds(self, rhs: fxx) -> Self {
        self + rhs
    }

    fn get(self) -> fxx {
        self
    }

    fn add_clamped(self, rhs: Self, min: Self, max: Self) -> Self {
        Self::clamp(self + rhs, min, max)
    }

    fn mul(self, rhs: fxx) -> Self {
        self * rhs
    }

    fn tol_equals(&self, other: &Self, tolerance: fxx) -> bool {
        (self - other).abs() < tolerance
    }

    fn is_negative(&self) -> bool {
        *self < 0.0
    }
}

// impl Smoothable for Vec2 {
//     fn tol_equals(&self, other: &Self, tolerance: fxx) -> bool {
//         if (self.x - other.x).abs() < tolerance &&
//            (self.y - other.y).abs() < tolerance {
//             true
//         } else {
//             false
//         }
//     }

//     fn lerp(&self, other: &Self, t: fxx) -> Self {
//         Self::lerp(*self, *other, t)
//     }

//     fn add(self, rhs: Self) -> Self {
//         self + rhs
//     }

//     fn mul(self, rhs: fxx) -> Self {
//         self * rhs
//     }
// }

// impl Smoothable for Vec3 {
//     fn tol_equals(&self, other: &Self, tolerance: fxx) -> bool {
//         if (self.x - other.x).abs() < tolerance &&
//            (self.y - other.y).abs() < tolerance &&
//            (self.z - other.z).abs() < tolerance {
//             true
//         } else {
//             false
//         }
//     }

//     fn lerp(&self, other: &Self, t: fxx) -> Self {
//         Self::lerp(*self, *other, t)
//     }

//     fn add(self, rhs: Self) -> Self {
//         self + rhs
//     }

//     fn mul(self, rhs: fxx) -> Self {
//         self * rhs
//     }
// }
