/// math::quick
/// -----------
/// shorthands for a bunch of simple operations
use crate::kernel::fxx;
use crate::kernel::Vec3;
use crate::kernel::FRAC_PI_2;
use crate::kernel::PI;
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

/// logaritmic lerping
#[inline]
pub fn log_lerp(from: fxx, to: fxx, t: fxx) -> fxx {
    let zoom = lerp(fxx::ln(from), fxx::ln(to), t);
    fxx::exp(zoom)
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

#[inline]
pub fn lerp(a: fxx, b: fxx, t: fxx) -> fxx {
    a + t * (b - a)
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
    lerp(quad_in(t), quad_out(t), t)
}

pub fn skewed_smooth_step(t: fxx, s: fxx) -> fxx {
    // this mixing is very interesting for what I want to do with the wobble
    lerp(quad_in(t) * (1.0 - s), quad_out(t) * s, t)
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
pub fn normalize(t: fxx, range: &Range<fxx>) -> fxx {
    (t - range.start) / (range.end - range.start)
}

/// same as lerp, but using a Range
#[inline]
pub fn interpolate(t: fxx, range: &Range<fxx>) -> fxx {
    range.start + t * (range.end - range.start)
}

#[inline]
pub fn remap(t: fxx, from: &Range<fxx>, to: &Range<fxx>, clamped: bool) -> fxx {
    let mut norm = normalize(t, from);
    if clamped {
        norm = fxx::clamp(norm, 0.0, 1.0);
    }
    lerp(norm, to.start, to.end)
}
