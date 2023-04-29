// file containing shape smoothing operators
use crate::{
    kernel::{fxx, Vec3, PI},
    pts::Vectors,
};

// take good care that the quad is in counter-clockwise orientation, with regard to the axis
pub fn smooth_quad_to_square(quad: &mut [Vec3; 4], axis: Vec3) {
    let smoothers = get_smoothers_quad_to_square(quad, axis);
    for (q, s) in quad.iter_mut().zip(smoothers) {
        *q += s;
    }
}

pub fn get_smoothers_quad_to_square(quad: &[Vec3; 4], axis: Vec3) -> [Vec3; 4] {
    let corners = quad.to_vec();
    let center = Vectors::average(&corners);

    let rotated = corners
        .iter()
        .enumerate()
        .map(|(i, corner)| {
            Vectors::rotate_axis_angle(axis, -PI * (i as fxx) * 0.5, center - *corner)
        })
        .collect::<Vec<_>>();
    let average_rot = Vectors::average(&rotated);
    let deltas = rotated
        .iter()
        .enumerate()
        .map(|(i, rot)| Vectors::rotate_axis_angle(axis, PI * (i as fxx) * 0.5, *rot - average_rot))
        .collect::<Vec<_>>();

    // println!("center: {center:?}");
    // println!("rotated: {rotated:?}");
    // println!("avg rot: {average_rot:?}");
    // println!("deltas: {deltas:?}");

    deltas.try_into().unwrap()
}

pub fn get_smoothers_quad_to_square_at_length(
    quad: &[Vec3; 4],
    axis: Vec3,
    length: fxx,
) -> [Vec3; 4] {
    let corners = quad.to_vec();
    let center = Vectors::average(&corners);

    let rotated = corners
        .iter()
        .enumerate()
        .map(|(i, corner)| {
            Vectors::rotate_axis_angle(axis, -PI * (i as fxx) * 0.5, center - *corner)
        })
        .collect::<Vec<_>>();
    let average_rot = Vectors::average(&rotated);

    let average_rot_at_right_length = average_rot.normalize() * length;
    let deltas = rotated
        .iter()
        .enumerate()
        .map(|(i, rot)| {
            Vectors::rotate_axis_angle(
                axis,
                PI * (i as fxx) * 0.5,
                *rot - average_rot_at_right_length,
            )
        })
        .collect::<Vec<_>>();

    // println!("center: {center:?}");
    // println!("rotated: {rotated:?}");
    // println!("avg rot: {average_rot:?}");
    // println!("deltas: {deltas:?}");

    deltas.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{
        kernel::{vec3, Vec3},
        pts::Vectors,
    };

    use super::get_smoothers_quad_to_square;

    #[test]
    fn test_quad_smoothing() {
        let mut some_quad = [
            vec3(0.0, 0.0, 0.0),
            vec3(1.0, 0.0, 0.0),
            vec3(1.0, 1.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        ];

        assert_eq!(
            Vectors::average(&some_quad.to_vec()),
            Vec3::new(0.5, 0.5, 0.0)
        );
        assert_eq!(
            Vectors::rotate_pt_axis_angle(vec3(0.5, 0.5, 0.0), Vec3::Z, 0.0, Vec3::ZERO),
            Vec3::ZERO
        );

        let smoothers = get_smoothers_quad_to_square(&some_quad, Vec3::Z);
        println!("should all be around zero: {smoothers:?}");

        // now mutate the quad
        some_quad[2] =
            Vectors::rotate_pt_axis_angle(vec3(0.5, 0.5, 0.0), Vec3::Z, 0.1, some_quad[2]);
        some_quad[3] =
            Vectors::rotate_pt_axis_angle(vec3(0.5, 0.5, 0.0), Vec3::Z, -0.1, some_quad[3]);
        println!();
        println!("QUADS: {some_quad:?}");
        let smoothers = get_smoothers_quad_to_square(&some_quad, Vec3::Z);
        println!("third corrector should be longest: {smoothers:?}");
        for (q, s) in some_quad.iter_mut().zip(smoothers) {
            *q += s;
        }

        // after smoothing, smoothers should be around zero
        println!();
        let smoothers = get_smoothers_quad_to_square(&some_quad, Vec3::Z);
        println!("should all be around zero: {smoothers:?}");
    }
}
