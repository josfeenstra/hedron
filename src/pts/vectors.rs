// we need a different struct, since a bunch of vectors will need to be rendered as a bunch of arrows

use crate::{
    core::PointBased,
    kernel::{fxx, Vec3, TAU, Quat},
};

// abstraction around a list of vectors.
// allows us to easily access function operating on lists of points, and to render them
// TODO, maybe add a more performant internal data structure, like a flat buffer? Would that even be more performant?
// TODO: specify points?
// TODO create a cool macro wrapping Vectors::new(vec![Vec3::new(0,0,0)]) as vectors![(0,0,0)]
pub struct Vectors {
    pub data: Vec<Vec3>,
}

// Anytime a function askes for a normal, you MUST provide an actually normalized vector!
impl Vectors {
    pub fn new(data: Vec<Vec3>) -> Self {
        Self { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn from_vec_of_arrays(vec: Vec<[fxx; 3]>) -> Vec<Vec3> {
        vec.iter().map(|v| Vec3::from_array(*v)).collect()
    }

    pub fn to_vec_of_arrays(self) -> Vec<[fxx; 3]> {
        self.into()
    }

    /// A simple but awesome operation to cheaply 'flatten' a point onto a plane.
    pub fn project_point_to_plane(center: Vec3, normal: Vec3, point: Vec3) -> Vec3 {
        let center_to_point = point - center;
        let distance = center_to_point.dot(normal);
        point - distance * normal
    }

    /// Given a plane defined by a `center` and `normal`, and `points` surrounding the origin of this plane,
    /// Return the indices of the point "before" and "after" a certain `sample` point.
    pub fn get_between(
        center: Vec3,
        normal: Vec3,
        points: Vec<Vec3>,
        sample_point: Vec3,
    ) -> Option<(usize, usize)> {
        match points.len() {
            0 => return None,
            1 => return Some((0, 0)),
            _ => {}
        };

        let sample = Self::project_point_to_plane(center, normal, sample_point);
        let to_sample = sample - center;

        let mut left_way_hit = TAU;
        let mut left_id = 0;
        let mut right_way_hit = TAU;
        let mut right_id = 0;

        // this only makes sense using an image
        for (i, point) in points.iter().enumerate() {
            let point = Self::project_point_to_plane(center, normal, *point);
            let to_point = point - center;

            // one angle takes the short route, the other the long route.
            let mut right_angle = to_sample.angle_between(to_point);
            let mut left_angle = TAU - right_angle;

            // TODO check which sign is which
            // based on this choice, (1,4) or (4,1) will be returned
            // - FOR THE EDGE CASE angle == PI, this does not matter
            //
            let sign = to_sample.cross(normal).dot(to_point) > 0.0;
            if sign {
                // println!("FLIPPERINO!");
                (left_angle, right_angle) = (right_angle, left_angle);
            }

            // println!("{i}, right: {right_angle}, left: {left_angle}");

            if left_angle < left_way_hit {
                left_way_hit = left_angle;
                left_id = i;
            }
            if right_angle < right_way_hit {
                right_way_hit = right_angle;
                right_id = i;
            }
        }
        // println!("best right, {right_id}, best left {left_id}");
        Some((right_id, left_id))
    }

    pub fn average(verts: &Vec<Vec3>) -> Vec3 {
        verts.iter().fold(Vec3::ZERO, |sum, item| sum + *item) / verts.len() as fxx
    }

    pub fn rotate_axis_angle(axis: Vec3, angle: fxx, vector: Vec3) -> Vec3 {
        Quat::from_axis_angle(axis, angle).mul_vec3(vector)
    }
    
    /// rotate a point using an axis and an angle
    pub fn rotate_pt_axis_angle(center: Vec3, axis: Vec3, angle: fxx, pt: Vec3) -> Vec3 {
        let norm = -center + pt;
        // println!("{norm}");
        let rot = Quat::from_axis_angle(axis, angle).mul_vec3(norm);
        // println!("{rot}");
        center + rot
    }
}

impl PointBased for Vectors {
    fn mutate_points<'a>(&'a mut self) -> Vec<&'a mut Vec3> {
        self.data.iter_mut().collect()
    }
}

// impl From<Points> for Vectors {
//     fn from(points: Points) -> Self {
//         Self { data: points.data }
//     }
// }

impl From<Vectors> for Vec<Vec3> {
    fn from(points: Vectors) -> Self {
        points.data
    }
}

impl From<Vectors> for Vec<[fxx; 3]> {
    fn from(points: Vectors) -> Self {
        points.data.iter().map(|v| v.to_array()).collect()
    }
}

#[cfg(feature = "nalgebra")]
use nalgebra::DMatrix;

#[cfg(feature = "nalgebra")]
impl From<Vectors> for DMatrix<fxx> {
    fn from(val: Vectors) -> Self {
        // println!("{}", val.data.len());
        let mut matrix = DMatrix::zeros(val.data.len(), 3);
        for (i, v) in val.data.iter().enumerate() {
            // println!("{}: {}", i, v);
            matrix[(i, 0)] = v.x;
            matrix[(i, 1)] = v.y;
            matrix[(i, 2)] = v.z;
        }
        // println!("{}, {}", matrix.nrows(), matrix.ncols());
        matrix
    }
}
