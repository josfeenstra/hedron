/// math::quick
/// -----------
/// shorthands for a bunch of simple operations
use crate::kernel::fxx;
use crate::kernel::Vec3;
use crate::kernel::FRAC_PI_2;
use crate::kernel::PI;
use crate::kernel::vec2;
use num_traits::float::Float;
use std::ops::Range;

/// TODO add this to a vector / points area when we get it
/// convert a spherical coordinate to a cartesian one
#[inline]
pub fn spherical_to_cartesian(inclination: fxx, azimuthal: fxx) -> Vec3 {
    let (sin_i, cos_i) = inclination.sin_cos(); // theta
    let (sin_a, cos_a) = azimuthal.sin_cos();
    Vec3::new(sin_a * sin_i, cos_a * sin_i, cos_i)
}

/// roughly equals, for when dealing with floating point equality
#[inline]
pub fn epsilon_equals<T: Float>(lhs: T, rhs: T, e: T) -> bool {
    (lhs - rhs).abs() < e
}

/// get `t` as a fraction between `min` and `max`.
#[inline]
pub fn frac(t: fxx, min: fxx, max: fxx) -> fxx {
    (t - min) / (max - min)
}

#[inline]
pub fn to_deg(rad: fxx) -> fxx {
    rad * 180. / PI
}

#[inline]
pub fn to_rad(deg: fxx) -> fxx {
    (deg * PI) / 180.
}

/// Factorial, calculated naively
/// ex: 3! = 1 * 2 * 3 = 6
#[inline]
pub fn fact(n: usize) -> usize {
    let mut product = 1;
    for i in 1..(n + 1) {
        product *= i;
    }
    product
}

/// Factorial but addition, don't know what it is called
/// ex: 3 = 1 + 2 + 3 = 6
#[inline]
pub fn stack_sum(n: usize) -> usize {
    let mut sum = 0;
    for i in 1..(n + 1) {
        sum += i;
    }
    sum
}

///////////////////////////////////////////////////////////////////////////////

/// when interpolating t between multiple data points,
/// 'select' the two data points it falls between, if t in 0..data.len().
pub fn select_sample(t: fxx, data: Vec<fxx>) -> (fxx, fxx, fxx) {
    let count = data.len() - 1;
    let p = t * count as fxx;
    let a = p.floor();
    let b = p.ceil();
    (p - a, a, b)
}

/**
 * binary search to figure out between which two values this sample is
 * assumes data is sorted!!
 */
pub fn between(t: fxx, data: Vec<fxx>) -> (usize, usize) {
    let mut start: usize = 0;
    let mut end = data.len() - 1;

    for _ in 0..data.len() {
        if start > end {
            std::mem::swap(&mut start, &mut end);
            break;
        }

        let mid = ((end - start) as fxx / 2.0).round() as usize;
        if t < data[mid] {
            // lower | on the left
            start = mid;
        } else if t > data[mid] {
            // higher | on the right
            end = mid;
        } else {
            // same!
            start = mid;
            end = mid;
            break;
        }
    }
    return (start, end);
}

////////////////////////////////////////////////////////////////////////////
// these are all shaper functions
// variations of a 0 to 1 interpolation, creating a nice effect
// from : https://www.youtube.com/watch?v=YJB1QnEmlTs
////////////////////////////////////////////////////////////////////////////

/// logaritmic lerping
#[inline]
pub fn log_lerp(from: fxx, to: fxx, t: fxx) -> fxx {
    let zoom = lerp(t, fxx::ln(from), fxx::ln(to));
    fxx::exp(zoom)
}

#[inline]
pub fn lerp(t: fxx, a: fxx, b: fxx) -> fxx {
    a + t * (b - a)
}

/// bezier-interpolate 
/// with w_start, w_end, and t in [0..1], create a 2D unit bezier curve as (0,0), (start, 0), (1 - end,1), (1,1). 
/// interpolate this bezier using t, then return the x of this bezier
/// TODO: currenty, this is the opposite of what we want: speed at the edges, smooth in the middle... 
#[inline]
pub fn lerp_quad_bezier(w_start: fxx, w_end: fxx, t: fxx) -> fxx {
    let [p0, p1, p2, p3] = [0.0, w_start, 1.0 - w_end, 1.0];
    
    // TODO: rewrite: so that we get a regular curve in the shape of y = ... polynomial
    let px = 
        0.0 *       (1.0 - t).powi(3) +
        0.0 * 3.0 * (1.0 - t).powi(2) * t + 
        1.0 * 3.0 * (1.0 - t)         * t.powi(2) + 
        1.0                           * t.powi(3);

    // let py = 
    //     p0 *       (1.0 - px).powi(3) +
    //     p1 * 3.0 * (1.0 - px).powi(2) * px + 
    //     p2 * 3.0 * (1.0 - px)         * px.powi(2) + 
    //     p3                            * px.powi(3);

    px
}


#[inline]
pub fn parabola(t: fxx, k: i32) -> fxx {
    (4.0 * t * (1.0 - t)).powi(k)
}

#[inline]
pub fn quad_in(t: fxx) -> fxx {
    t * t
}

#[inline]
pub fn quad_out(t: fxx) -> fxx {
    1.0 * (1.0 - t) * (1.0 - t)
}

#[inline]
pub fn smooth_step(t: fxx) -> fxx {
    // this mixing is very interesting for what I want to do with the wobble
    lerp(t, quad_in(t), quad_out(t))
}

pub fn skewed_smooth_step(t: fxx, s: fxx) -> fxx {
    // this mixing is very interesting for what I want to do with the wobble
    lerp(t, quad_in(t) * (1.0 - s), quad_out(t) * s)
}

#[inline]
pub fn elastic_out(t: fxx) -> fxx {
    fxx::sin(-13.0 * (t + 1.0) * FRAC_PI_2) * fxx::powf(2.0, -10.0 * t) + 1.0
}

pub fn bounce_out(mut t: fxx) -> fxx {
    let nl = 7.5625;
    let dl = 2.75;

    if t < 1.0 / dl {
        nl * t * t
    } else if t < 2.0 / dl {
        t -= 1.5 / dl;
        nl * t * t + 0.75
    } else if t < 2.5 / dl {
        t -= 2.25 / dl;
        nl * t * t + 0.9375
    } else {
        t -= 2.625 / dl;
        nl * t * t + 0.984375
    }
}

/// Fade function as defined by Ken Perlin.  This eases coordinate values
/// so that they will ease towards integral values.  This ends up smoothing
/// the final output.
/// equals to 6t^5 - 15t^4 + 10t^3
#[inline]
pub fn smooth(t: fxx) -> fxx {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

#[inline]
pub fn normalize(t: fxx, start: fxx, end: fxx) -> fxx {
    (t - start) / (end - start)
}

#[inline]
pub fn remap(t: fxx, from_start: fxx, from_end: fxx, to_start: fxx, to_end:fxx, clamped: bool) -> fxx {
    let mut norm = normalize(t, from_start, from_end);
    if clamped {
        norm = fxx::clamp(norm, 0.0, 1.0);
    }
    lerp(norm, to_start, to_end)
}

#[inline]
pub fn iter_n_times(start: fxx, end: fxx, steps: usize) -> impl Iterator<Item = fxx> {
    let delta = (end - start) / (steps - 1) as fxx;
    let thing = (0..steps).map(move |i| start + delta * i as fxx);
    thing
}

/// 
#[inline]
pub fn iter_by_delta(start: fxx, end: fxx, delta: fxx) -> impl Iterator<Item = fxx> {
    let steps = ((end - start) / delta).floor() as usize + 1;
    (0..steps).map(move |i| start + delta * i as fxx)
}

#[cfg(test)]
mod test {
    use crate::kernel::fxx;

    use super::lerp_quad_bezier;


    #[test]
    fn test_bezier_int() {
        for i in 0..101 {
            let f = i as fxx / 100.0;

            let t = lerp_quad_bezier(1.0, 1.0, f);
            println!("f {f}, t {t}");
        }
    }
}