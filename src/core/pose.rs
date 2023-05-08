use std::ops::Mul;

use crate::kernel::{fxx, Affine3, Mat3, Mat4, Quat, Vec3};

use super::Plane;

/// TODO: Merge Pose with Plane
/// Strictly speaking, a pose should not have a scale.
/// Its the difference between a 'transform' and a 'scale'
#[derive(Debug)]
pub struct Pose {
    pub pos: Vec3,
    pub rot: Quat,
    // pub scale: Vec3,
}

impl Pose {
    /// An identity [`Pose`] with no translation, rotation, and a scale of 1 on all axes.
    pub const IDENTITY: Self = Self {
        pos: Vec3::ZERO,
        rot: Quat::IDENTITY,
        // scale: Vec3::ONE,
    };

    pub const WORLD_XY: Self = Self::IDENTITY;

    pub const WORLD_YZ: Self = Self {
        pos: Vec3::ZERO,
        rot: Quat::IDENTITY, // TODO
                             // scale: Vec3::ONE,
    };

    pub const WORLD_ZX: Self = Self {
        pos: Vec3::ZERO,
        rot: Quat::IDENTITY, // TODO
                             // scale: Vec3::ONE,
    };

    /// Creates a new [`Pose`] at the position `(x, y, z)`. In 2d, the `z` component
    /// is used for z-ordering elements: higher `z`-value will be in front of lower
    /// `z`-value.
    #[inline]
    pub const fn from_xyz(x: fxx, y: fxx, z: fxx) -> Self {
        Self::from_pos(Vec3::new(x, y, z))
    }

    /// Extracts the translation, rotation, and scale from `matrix`. It must be a 3d affine
    /// transformation matrix.
    #[inline]
    pub fn from_matrix(matrix: Mat4) -> Self {
        let (_, rot, pos) = matrix.to_scale_rotation_translation();
        Self {
            pos,
            rot,
            // scale
        }
    }

    /// Creates a new [`Pose`], with `translation`. Rotation will be 0 and scale 1 on
    /// all axes.
    #[inline]
    pub const fn from_pos(pos: Vec3) -> Self {
        Self {
            pos,
            ..Self::IDENTITY
        }
    }

    /// Creates a new [`Pose`], with `rotation`. Translation will be 0 and scale 1 on
    /// all axes.
    #[inline]
    pub const fn from_rot(rot: Quat) -> Self {
        Self {
            rot,
            ..Self::IDENTITY
        }
    }

    // /// Creates a new [`Pose`], with `scale`. Translation will be 0 and rotation 0 on
    // /// all axes.
    // #[inline]
    // pub const fn from_scale(scale: Vec3) -> Self {
    //     Self {
    //         scale,
    //         ..Self::IDENTITY
    //     }
    // }

    /// Updates and returns this [`Pose`] by rotating it so that its unit
    /// vector in the local negative `Z` direction is toward `target` and its
    /// unit vector in the local `Y` direction is toward `up`.
    #[inline]
    #[must_use]
    pub fn looking_at(mut self, target: Vec3, up: Vec3) -> Self {
        self.look_at(target, up);
        self
    }

    /// Returns this [`Pose`] with a new translation.
    #[inline]
    #[must_use]
    pub const fn with_pos(mut self, translation: Vec3) -> Self {
        self.pos = translation;
        self
    }

    /// Returns this [`Pose`] with a new rotation.
    #[inline]
    #[must_use]
    pub const fn with_rot(mut self, rotation: Quat) -> Self {
        self.rot = rotation;
        self
    }

    // /// Returns this [`Pose`] with a new scale.
    // #[inline]
    // #[must_use]
    // pub const fn with_scale(mut self, scale: Vec3) -> Self {
    //     self.scale = scale;
    //     self
    // }

    /// Returns the 3d affine transformation matrix from this transforms translation,
    /// rotation, and scale.
    #[inline]
    pub fn compute_matrix(&self) -> Mat4 {
        // let scale = self.scale
        let scale = Vec3::ONE;
        Mat4::from_scale_rotation_translation(scale, self.rot, self.pos)
    }

    /// Returns the 3d affine transformation matrix from this transforms translation,
    /// rotation, and scale.
    #[inline]
    pub fn compute_affine(&self) -> Affine3 {
        // let scale = self.scale
        let _scale = Vec3::ONE;
        Affine3::from_scale_rotation_translation(Vec3::ONE, self.rot, self.pos)
    }

    /// Get the unit vector in the local `X` direction.
    #[inline]
    pub fn local_x(&self) -> Vec3 {
        self.rot * Vec3::X
    }

    /// Get the unit vector in the local `Y` direction.
    #[inline]
    pub fn local_y(&self) -> Vec3 {
        self.rot * Vec3::Y
    }

    /// Get the unit vector in the local `Z` direction.
    #[inline]
    pub fn local_z(&self) -> Vec3 {
        self.rot * Vec3::Z
    }

    /// Rotates this [`Pose`] by the given rotation.
    #[inline]
    pub fn rotate(&mut self, rotation: Quat) {
        self.rot = rotation * self.rot;
    }

    /// Rotates this [`Pose`] around the given `axis` by `angle` (in radians).
    #[inline]
    pub fn rotate_axis(&mut self, axis: Vec3, angle: fxx) {
        self.rotate(Quat::from_axis_angle(axis, angle));
    }

    /// Rotates this [`Pose`] around the `X` axis by `angle` (in radians).
    #[inline]
    pub fn rotate_x(&mut self, angle: fxx) {
        self.rotate(Quat::from_rotation_x(angle));
    }

    /// Rotates this [`Pose`] around the `Y` axis by `angle` (in radians).
    #[inline]
    pub fn rotate_y(&mut self, angle: fxx) {
        self.rotate(Quat::from_rotation_y(angle));
    }

    /// Rotates this [`Pose`] around the `Z` axis by `angle` (in radians).
    #[inline]
    pub fn rotate_z(&mut self, angle: fxx) {
        self.rotate(Quat::from_rotation_z(angle));
    }

    /// Rotates this [`Pose`] by the given `rotation`.
    #[inline]
    pub fn rotate_local(&mut self, rotation: Quat) {
        self.rot *= rotation;
    }

    /// Rotates this [`Pose`] around its local `axis` by `angle` (in radians).
    #[inline]
    pub fn rotate_local_axis(&mut self, axis: Vec3, angle: fxx) {
        self.rotate_local(Quat::from_axis_angle(axis, angle));
    }

    /// Rotates this [`Pose`] around its local `X` axis by `angle` (in radians).
    #[inline]
    pub fn rotate_local_x(&mut self, angle: fxx) {
        self.rotate_local(Quat::from_rotation_x(angle));
    }

    /// Rotates this [`Pose`] around its local `Y` axis by `angle` (in radians).
    #[inline]
    pub fn rotate_local_y(&mut self, angle: fxx) {
        self.rotate_local(Quat::from_rotation_y(angle));
    }

    /// Rotates this [`Pose`] around its local `Z` axis by `angle` (in radians).
    #[inline]
    pub fn rotate_local_z(&mut self, angle: fxx) {
        self.rotate_local(Quat::from_rotation_z(angle));
    }

    /// Translates this [`Pose`] around a `point` in space.
    ///
    /// If this [`Pose`] has a parent, the `point` is relative to the [`Pose`] of the parent.
    #[inline]
    pub fn translate_around(&mut self, point: Vec3, rotation: Quat) {
        self.pos = point + rotation * (self.pos - point);
    }

    /// Rotates this [`Pose`] around a `point` in space.
    ///
    /// If this [`Pose`] has a parent, the `point` is relative to the [`Pose`] of the parent.
    #[inline]
    pub fn rotate_around(&mut self, point: Vec3, rotation: Quat) {
        self.translate_around(point, rotation);
        self.rotate(rotation);
    }

    /// Rotates this [`Pose`] so that its local negative `Z` direction is toward
    /// `target` and its local `Y` direction is towards `up`.
    #[inline]
    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        let forward = Vec3::normalize(self.pos - target);
        let right = up.cross(forward).normalize();
        let up = forward.cross(right);
        self.rot = Quat::from_mat3(&Mat3::from_cols(right, up, forward));
    }

    /// Multiplies `self` with `Pose` component by component, returning the
    /// resulting [`Pose`]
    #[inline]
    #[must_use]
    pub fn mul_transform(&self, pose: Pose) -> Self {
        let pos = self.transform_point(pose.pos);
        let rot = self.rot * pose.rot;
        // let scale = self.scale * pose.scale;
        Pose {
            pos,
            rot,
            // scale
        }
    }

    /// Transforms the given `point`, applying scale, rotation and translation.
    #[inline]
    pub fn transform_point(&self, mut point: Vec3) -> Vec3 {
        // point = self.scale * point;
        point = self.rot * point;
        point += self.pos;
        point
    }

    // inversly transform a point to the 'space' of this pose
    // This does not work if pose.scale contains zeroes.
    // After all, if 5 * 0 = 0, 0 * ? = 5 is impossible
    #[inline]
    pub fn transform_point_inv(&self, mut point: Vec3) -> Vec3 {
        point -= self.pos;
        point = self.rot.inverse() * point;
        // point = point / self.scale;
        point
    }

    /// Use the local Z as the normal
    #[inline]
    pub fn normal(&self) -> Vec3 {
        self.local_z()
    }

    /// turn into a precalculated plane
    pub fn to_plane(self) -> Plane {
        Plane::new(self.compute_matrix())
    }
}

impl Default for Pose {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Into<Plane> for Pose {
    fn into(self) -> Plane {
        self.to_plane()
    }
}

impl Mul<Pose> for Pose {
    type Output = Pose;

    fn mul(self, transform: Pose) -> Self::Output {
        self.mul_transform(transform)
    }
}

impl Mul<Vec3> for Pose {
    type Output = Vec3;

    fn mul(self, value: Vec3) -> Self::Output {
        self.transform_point(value)
    }
}

// TODO: make something like a CachedPose , then an Into<CachedPose>
