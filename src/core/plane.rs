use std::cmp::Ordering;

use bevy_math::Vec4Swizzles;

use crate::kernel::{fxx, Mat4, Quat, Vec3, Vec4};

use crate::math::TOLERANCE;
use crate::prelude::{half_plane_test_tol, line_x_plane, Vectors};

use super::geometry::Geometry;

// a 'transform', but from a modelling perspective
// TODO use a transform instead of a matrix!!!
#[derive(Clone)]
pub struct Plane {
    mat: Mat4,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            mat: Mat4::IDENTITY,
        }
    }
}

impl Plane {
    pub const WORLD_XY: Self = Self::new(Mat4::from_cols(Vec4::X, Vec4::Y, Vec4::Z, Vec4::W));
    pub const WORLD_YZ: Self = Self::new(Mat4::from_cols(
        Vec4::Y,
        Vec4::Z,
        Vec4::X, /* * -1? */
        Vec4::W,
    ));
    pub const WORLD_XZ: Self = Self::new(Mat4::from_cols(
        Vec4::X,
        Vec4::Z,
        Vec4::Y, /* * -1? */
        Vec4::W,
    ));

    pub const fn new(mat: Mat4) -> Self {
        Self { mat }
    }

    // pub const fn from_mat(mat: Mat4) -> Self {
    //     let tf = mat.to_scale_rotation_translation();
    //     Self { tf }
    // }

    /// TODO: use look at?
    pub fn from_pos_normal(point: Vec3, normal: Vec3) -> Self {
        let mut ihat = normal.any_orthonormal_vector();
        let jhat = ihat.cross(normal).normalize();
        let khat = ihat.cross(jhat);
        if khat.dot(normal) < 0.0 {
            ihat *= -1.0;
        }

        Self::from_pvv_exact(point, ihat, jhat)
    }

    /// Create a plane from a center point and two axis.
    /// These axis do not need to be orthogonal or normalized
    pub fn from_pvv_guess(p: Vec3, vi: Vec3, vj: Vec3) -> Option<Self> {
        // we do a trick so this works with non-orthogonal vectors
        // i is always i, j is adjusted to fit the model
        let k = vi.cross(vj).normalize();
        if vi.dot(vj) < TOLERANCE {
            return None;
        }
        let i = vi.normalize();
        let j = k.cross(i).normalize(); // is a cross product between normalized vectors always normalized ????

        Some(Self {
            mat: Mat4::from_cols(i.extend(0.0), j.extend(0.0), k.extend(0.0), p.extend(1.0)),
        })
    }

    /// Create a plane from a center point and two normalized, orthogonal axis
    pub fn from_pvv_exact(p: Vec3, i: Vec3, j: Vec3) -> Self {
        let k = i.cross(j);
        debug_assert!(k.length() > TOLERANCE);

        Self {
            mat: Mat4::from_cols(i.extend(0.0), j.extend(0.0), k.extend(0.0), p.extend(1.0)),
        }
    }

    /// TODO: improve this by using least squares or minimum volume algorithms
    pub fn from_points(points: &[Vec3], tolerance: fxx) -> Option<Self> {
        if points.len() < 3 {
            return None;
        }

        let center = Vectors::average(points);
        let axis_1 = points[0] - center;
        for vert in points.iter().skip(1) {
            let axis_2 = *vert - center;
            dbg!(&axis_2);
            if axis_2.dot(axis_1).abs() > tolerance {
                dbg!("found one!");
                return Self::from_pvv_guess(center, axis_1, axis_2);
            }
        }
        dbg!("found none!");
        None
    }

    pub fn from_pts(a: Vec3, b: Vec3, c: Vec3) -> Option<Self> {
        Self::from_pvv_guess(a, b - a, c - a)
    }

    /// get the center of the plane
    #[inline]
    pub fn origin_col(&self) -> Vec4 {
        self.mat.w_axis
    }

    #[inline]
    pub fn origin(&self) -> Vec3 {
        self.mat.w_axis.xyz()
    }

    /// get a clone of the normal of the plane
    #[inline]
    pub fn normal_col(&self) -> Vec4 {
        self.mat.z_axis
    }

    #[inline]
    pub fn normal(&self) -> Vec3 {
        self.mat.z_axis.xyz()
    }

    /// take a point defined in the world, and translate it to 'plane space'
    pub fn point_to_plane(&self, point: Vec3) -> Vec3 {
        // there must be more effective ways of doing this...
        let p = point.extend(0.0);
        let v = p - self.origin_col();
        let dist_x = v.dot(self.mat.x_axis);
        let dist_y = v.dot(self.mat.y_axis);
        let dist_z = v.dot(self.mat.z_axis);
        Vec3::new(dist_x, dist_y, dist_z)
    }

    /// take a point defined in the space of this plane, and transform it to world space
    pub fn point_to_world(&self, point: Vec3) -> Vec3 {
        self.mat.transform_point3(point)
    }

    pub fn distance_to_point(&self, point: Vec3) -> fxx {
        self.point_to_plane(point).z
    }

    #[inline]
    fn a(&self) -> fxx {
        self.mat.z_axis.x
    }

    #[inline]
    fn b(&self) -> fxx {
        self.mat.z_axis.y
    }

    #[inline]
    fn c(&self) -> fxx {
        self.mat.z_axis.z
    }

    #[inline]
    pub fn d(&self) -> fxx {
        self.normal_col().dot(self.origin_col()) * -1.0
    }

    /// voor de liefhebber
    pub fn abcd(&self) -> (fxx, fxx, fxx, fxx) {
        (self.a(), self.b(), self.c(), self.d())
    }

    /// Test on which side a point is, relative to the plane this
    /// pose represents
    /// NOT using robust predicates. Uses the crate-defined tolerance value to deal with fuzzyness
    ///
    /// Response:
    /// greater -> point is on the side the normal is pointing to
    /// less -> point is on the other side
    /// Equal -> point lies exactly on the plane
    pub fn half_plane_test(&self, point: Vec3) -> Ordering {
        half_plane_test_tol(self.origin(), self.normal(), point, TOLERANCE)
    }

    pub fn half_plane_test_tol(&self, point: Vec3, tolerance: fxx) -> Ordering {
        half_plane_test_tol(self.origin(), self.normal(), point, tolerance)
    }

    pub fn x_line(&self, a: Vec3, b: Vec3) -> Option<Vec3> {
        line_x_plane(a, b, self.origin(), self.normal()).map(|t| a.lerp(b, t))
    }
}

impl Geometry for Plane {
    fn mv(mut self, mv: Vec3) -> Self {
        self.mat.w_axis += mv.extend(0.0);
        self
    }

    fn rot(mut self, rot: &Quat) -> Self {
        self.mat = self.mat.mul_mat4(&Mat4::from_quat(*rot));
        self
    }

    fn scale(mut self, scale: Vec3) -> Self {
        self.mat.x_axis.x *= scale.x;
        self.mat.y_axis.y *= scale.y;
        self.mat.z_axis.z *= scale.z;
        self
    }

    fn scale_u(mut self, scalar: fxx) -> Self {
        self.mat.x_axis.x *= scalar;
        self.mat.y_axis.y *= scalar;
        self.mat.z_axis.z *= scalar;
        self
    }
}
