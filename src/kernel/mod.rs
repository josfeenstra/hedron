// The Floating-point precision is globally configurable using the "f32" feature

#[cfg(feature = "f32")]
pub mod kernel {
    #[allow(non_camel_case_types)]
    pub type fxx = f32;
    pub use std::f32 as floats;

    pub type Vec2 = glam::Vec2;
    pub type Vec3 = glam::Vec3;
    pub type Mat3 = glam::Mat3;
    pub type Mat4 = glam::Mat4;
    pub type Quat = glam::Quat;

    pub const TAU: f32 = std::f32::consts::TAU;
    pub const PI: f32 = std::f32::consts::PI;
    pub const FRAC_PI_2: f32 = std::f32::consts::FRAC_PI_2;

    pub fn as_vec2(vec: glam::IVec2) -> Vec2 {
        vec.as_vec2()
    }
}

#[cfg(not(feature = "f32"))]
pub mod kernel {
    #[allow(non_camel_case_types)]
    pub type fxx = f64;
    pub use std::f64 as floats;
    pub type List = Vec<fxx>;

    pub type Vec2 = glam::DVec2;
    pub type Vec3 = glam::DVec3;
    pub type Vec4 = glam::DVec4;
    pub type Mat3 = glam::DMat3;
    pub type Mat4 = glam::DMat4;
    pub type Quat = glam::DQuat;
    pub type Affine3 = glam::DAffine3;

    pub const TAU: f64 = std::f64::consts::TAU;
    pub const PI: f64 = std::f64::consts::PI;
    pub const FRAC_PI_2: f64 = std::f64::consts::FRAC_PI_2;

    pub use glam::dvec2 as vec2;
    pub use glam::dvec3 as vec3;

    pub fn as_vec2(vec: glam::IVec2) -> Vec2 {
        vec.as_dvec2()
    }
}

pub use kernel::*;
