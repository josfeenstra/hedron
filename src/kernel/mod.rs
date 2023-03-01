// The Floating-point precision is globally configurable using the "f32" feature

#[cfg(feature = "f32")]
pub mod kernel {
    #[allow(non_camel_case_types)]
    pub type fxx = f32;
    pub use std::f32 as floats;

    pub type Vec2 = glam::Vec2;
    pub type Vec3 = glam::Vec3;
    pub type Vec4 = glam::Vec4;
    pub type Mat3 = glam::Mat3;
    pub type Mat4 = glam::Mat4;
    pub type Quat = glam::Quat;
    pub type Affine3 = glam::Affine3A;

    pub const TAU: f32 = std::f32::consts::TAU;
    pub const PI: f32 = std::f32::consts::PI;
    pub const FRAC_PI_2: f32 = std::f32::consts::FRAC_PI_2;
    pub const EPSILON: f32 = std::f32::EPSILON;

    pub const SQRT_OF_3: f32 = 1.7320508075688772;

    pub const INFINITY: f32 = std::f32::INFINITY;
    pub const NEG_INFINITY: f32 = std::f32::NEG_INFINITY;

    pub use glam::vec2;
    pub use glam::vec3;

    pub fn uvec3_to_vec3(some: glam::UVec3) -> Vec3 {
        some.as_vec3()
    }

    pub fn uvec2_to_vec2(some: glam::UVec2) -> Vec2 {
        some.as_vec2()
    }

    pub fn ivec3_to_vec3(some: glam::IVec3) -> Vec3 {
        some.as_vec3()
    }

    pub fn ivec2_to_vec2(some: glam::IVec2) -> Vec2 {
        some.as_vec2()
    }

    pub fn as_vec2<I: Into<Vec2>>(some: I) -> Vec2 {
        some.into()
    }

    pub fn as_vec3<I: Into<Vec3>>(some: I) -> Vec3 {
        some.into()
    }
}

#[cfg(not(feature = "f32"))]
pub mod kernel {
    #[allow(non_camel_case_types)]
    pub type fxx = f64;
    pub use std::f64 as floats;

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
    pub const EPSILON: f64 = std::f64::EPSILON;
    pub const SQRT_OF_3: f64 = 1.7320508075688772;

    pub const INFINITY: f64 = std::f64::INFINITY;
    pub const NEG_INFINITY: f64 = std::f64::NEG_INFINITY;

    pub use glam::dvec2 as vec2;
    pub use glam::dvec3 as vec3;


    pub fn uvec3_to_vec3(some: glam::UVec3) -> Vec3 {
        some.as_dvec3()
    }

    pub fn uvec2_to_vec2(some: glam::UVec2) -> Vec2 {
        some.as_dvec2()
    }

    pub fn ivec3_to_vec3(some: glam::IVec3) -> Vec3 {
        some.as_dvec3()
    }

    pub fn ivec2_to_vec2(some: glam::IVec2) -> Vec2 {
        some.as_dvec2()
    }
    
    pub fn as_mat4(mat: glam::Mat4) -> Mat4 {
        mat.as_dmat4()
    }

    pub fn as_vec2<I: Into<Vec2>>(some: I) -> Vec2 {
        some.into()
    }
    
    pub fn as_vec3<I: Into<Vec3>>(some: I) -> Vec3 {
        some.into()
    }
}

pub use kernel::*;
