use super::Pose;
use crate::kernel::{fxx, Quat, Vec3};

// the core of a 'transformable thing',
// impl Geometry to give an object access to quick methods of transformation

/// If some geometry is ultimately defined in terms of points,
/// A whole set of common functionalities can be used to transform said geometry:
/// TODO smooth
//  scramble
//  scale from

/// If a Geometry is NOT purely defined by points,
/// reimplement the mv, rot, scale, and scale_u operators.
pub trait Geometry: Sized {
    // fn mutate_points<'a>(&'a mut self) -> &'a mut Vec<Vec3>;

    // fn mv(mut self, mv: &Vec3) -> Self {
    //     for v in self.mutate_points() {
    //         *v = *v + *mv;
    //     }
    //     self
    // }

    // fn rot(mut self, rot: &crate::math::Quat) -> Self {
    //     for v in self.mutate_points() {
    //         *v = *rot * *v;
    //     }
    //     self
    // }

    // fn scale(mut self, scale: &Vec3) -> Self {
    //     for v in self.mutate_points() {
    //         *v = *scale * *v;
    //     }
    //     self
    // }

    // fn scale_u(mut self, scale: fxx) -> Self {
    //     for v in self.mutate_points() {
    //         v.x *= scale;
    //         v.y *= scale;
    //         v.z *= scale;
    //     }
    //     self
    // }

    fn mv(self, mv: Vec3) -> Self;

    fn rot(self, rot: &Quat) -> Self;

    fn scale(self, scale: Vec3) -> Self;

    fn scale_u(self, scale: fxx) -> Self;

    /// rotate around x axis
    #[inline]
    #[must_use]
    fn rot_x(self, angle: fxx) -> Self {
        self.rot(&Quat::from_rotation_x(angle))
    }

    /// rotate around y axis
    #[inline]
    #[must_use]
    fn rot_y(self, angle: fxx) -> Self {
        self.rot(&Quat::from_rotation_y(angle))
    }

    /// rotate around z axis
    #[inline]
    #[must_use]
    fn rot_z(self, angle: fxx) -> Self {
        self.rot(&Quat::from_rotation_z(angle))
    }

    #[inline]
    #[must_use]
    /// apply a transformation
    fn tf(self, tf: &Pose) -> Self {
        // scale(tf.scale)
        self.rot(&tf.rot).mv(tf.pos)
    }

    #[inline]
    #[must_use]
    // apply the inverse of a transformation
    fn tf_inv(self, tf: &Pose) -> Self {
        self.mv(-tf.pos).rot(&tf.rot.inverse())
        // .scale(1.0 / tf.scale)
    }

    #[must_use]
    fn reorient(self, from: &Pose, to: &Pose) -> Self {
        self.tf_inv(from).tf(to)
    }
}

pub fn transform_point(tf: &Pose, mut point: Vec3) -> Vec3 {
    // point = tf.scale * point;
    point = tf.rot * point;
    point += tf.pos;
    point
}

pub fn transform_point_inverse(tf: &Pose, mut point: Vec3) -> Vec3 {
    point -= tf.pos;
    point = tf.rot.inverse() * point;
    // point = point / tf.scale;
    point
}

// you can do this:
// #[inline]
// pub const fn transform(mut self, tf: Mat4) -> Self {
//     // self.translation = translation;
//     // self
// }

// move, rotate, scale
// trait Transformable {

//     fn m(verts: &mut Vec<Vec3>, mv: Vec3) -> () {
//         for vert in verts.iter_mut() {
//             *vert += mv;
//         }
//     }

//     fn r(verts: &mut Vec<Vec3>, rot: Mat3) -> () {
//         for vert in verts.iter_mut() {
//             *vert = rot.mul_vec3(*vert);
//         }
//     }

//     fn s(verts: &mut Vec<Vec3>, scale: Vec3) -> () {
//         for vert in verts.iter_mut() {
//             *vert *= scale;
//         }
//     }
// }

// all past-tense functions return a copied object
// not sure what to do with this...

// trait Geometry {

//     fn transform(&self, m: Mat4) -> bool;

//     fn clone(&self) -> Self;

//     fn transformed(&self, m: Mat4) -> Self;

//     ///////////////////////////////////////////////////////////

//     fn rotate_x(&self, radians: fxx) -> bool {
//         let rotater = Mat4::from_rotation_x(radians);
//         self.transform(rotater)
//     }

//     fn rotate_y(&self, radians: fxx) -> bool {
//         let rotater = Mat4::from_rotation_y(radians);
//         self.transform(rotater)
//     }

//     fn rotate_z(&self, radians: fxx) -> bool {
//         let rotater = Mat4::from_rotation_z(radians);
//         self.transform(rotater)
//     }

//     fn rotate(&self, radians: fxx, axis: Vec3) -> bool {
//         let rotater = Mat4::from_axis_angle(axis, radians);
//         self.transform(rotater)
//     }

//     fn translate(&self, mover: Vec3) -> bool {
//         let mover = Mat4::from_translation(mover);
//         self.transform(mover)
//     }

//     fn scale(&self, scale: Vec3) -> bool {
//         let scaler = Mat4::from_scale(scale);
//         self.transform(scaler)
//     }

//     ///////////////////////////////////////////////////////////

//     fn rotated_x(&self, radians: fxx) -> bool {
//         let rotater = Mat4::from_rotation_x(radians);
//         self.transform(rotater)
//     }

//     fn rotated_y(&self, radians: fxx) -> bool {
//         let rotater = Mat4::from_rotation_y(radians);
//         self.transform(rotater)
//     }

//     fn rotated_z(&self, radians: fxx) -> bool {
//         let rotater = Mat4::from_rotation_z(radians);
//         self.transform(rotater)
//     }

//     fn rotated(&self, radians: fxx, axis: Vec3) -> bool {
//         let rotater = Mat4::from_axis_angle(axis, radians);
//         self.transform(rotater)
//     }

//     fn translated(&self, mover: Vec3) -> bool {
//         let mover = Mat4::from_translation(mover);
//         self.transform(mover)
//     }

//     fn scaled(&self, scale: Vec3) -> bool {
//         let scaler = Mat4::from_scale(scale);
//         self.transform(scaler)
//     }
// }
