// author: Jos Feenstra
/// purpose: polynomial math needed for curves & surfaces
/// note: uses a hardcoded pascal's triangle for performance reasons
/// notes:   based upon the excellent explanations from Prof. C.-K. Shene: https://pages.mtu.edu/~shene/COURSES/cs3621/NOTES/
///
/// TODO: precalculate pascals triangle
///  
use super::{fact};
use crate::{
    data::{Grid2},
};
use glam::Vec3;
use lazy_static::lazy_static;

const MAX_DEGREE: usize = 15;

lazy_static! {
    static ref LAZY_PASCAL: Vec<Vec<usize>> = pascal_triangle(MAX_DEGREE);
}

/// calculate weight using Bernstein Polynomials:
/// (n over i) t^i * (1-t)^(n - i).
/// precalculated Pascal's triangle for a bit more efficiency
#[inline]
pub fn bernstein(t: f32, i: usize, n: usize) -> f32 {
    return (get_bicoef(n, i) as f32) * t.powi(i as i32) * (1.0 - t).powi((n - i) as i32);
}

/// Binomial Coefficient
fn get_bicoef(n: usize, i: usize) -> usize {
    LAZY_PASCAL[n][i]
}

/// Binomial Coefficient
fn calc_bicoef(n: usize, i: usize) -> usize {
    // let f = crate::math::quick::fact;
    return fact(n) / (fact(i) * fact(n - i));
}

/// calculate pascals triangle from top to bottom
fn pascal_triangle(size: usize) -> Vec<Vec<usize>> {
    let mut pascal = Vec::with_capacity(size);
    for n in 0..size {
        let mut vec = Vec::with_capacity(n + 1);
        for i in 0..(n + 1) {
            vec.push(calc_bicoef(n, i))
        }
        pascal.push(vec);
    }
    pascal
}

///  This function returns the entire castejau piramid.
///  Hovever, this is slower than the PointAt() method.
///  useful for:
///  Subdividing bezier curves, debugging, and splines
fn decastejau(verts: Vec<Vec3>, t: f32) -> Grid2<Vec3> {
    let count = verts.len();
    let mut tri = Grid2::new(count, count);

    // copy paste the first row
    for x in 0..count {
        tri.set(x, 0, verts[x]);
    }

    // // iterate over this triangle, starting at the base + 1
    for x in 1..count {
        for y in 0..(count - x) {
            let p_a = tri.get(x - 1, y).unwrap();
            let p_b = tri.get(x - 1, y + 1).unwrap();
            let q = Vec3::lerp(p_a, p_b, t);
            tri.set(x, y, q);
        }
    }
    // result
    tri
}

// calculate the decastejau piramid based on extrapolation
fn decastejau_extrapolate_end(verts: Vec<Vec3>, t: f32) -> Grid2<Vec3> {
    let count = verts.len();

    let tri = Grid2::new(count, count);

    // copy paste the first row
    // for x in 0..count {
    //     tri.set(x, 0, verts[x]);
    // }

    // // // iterate over this triangle, starting at the base + 1
    // for x in 1..count {
    //     for y in 0..(count - x) {
    //         let p_a = tri.get(x - 1, y).unwrap();
    //         let p_b = tri.get(x - 1, y + 1).unwrap();
    //         let q = Vec3::lerp(p_a, p_b, t);
    //         tri.set(x, y, q);
    //     }
    // }
    // result
    tri
}

///////////////////////////////////////////////////////////////////////////////

/// TODO: do the coxdeboor things needed for spline curves
fn coxdeboor() -> f32 {
    1.0
}

///////////////////////////////////////////////////////////////////////////////

// cargo test polynomial -- --nocapture
#[cfg(test)]
mod tests {
    use crate::math::polynomial::pascal_triangle;

    #[test]
    fn test_pascals_triangle() {
        println!("testing pascals triangle generation");

        assert_eq!(pascal_triangle(3), vec![vec![1], vec![1, 1], vec![1, 2, 1]]);
        assert_eq!(
            pascal_triangle(4),
            vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]]
        );
        assert_eq!(
            pascal_triangle(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }

    #[test]
    fn test_something_else() {
        println!("testing something else");
    }
}
