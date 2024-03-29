use crate::kernel::*;
use num_traits::float::Float;

///! math::quick
///! -----------
///! shorthands for a bunch of simple operations

/// TODO add this to a vector / points area when we get it
/// convert a spherical coordinate to a cartesian one
#[inline]
pub fn spherical_to_cartesian(inclination: fxx, azimuthal: fxx) -> Vec3 {
    let (sin_i, cos_i) = inclination.sin_cos(); // theta
    let (sin_a, cos_a) = azimuthal.sin_cos();
    Vec3::new(sin_a * sin_i, cos_a * sin_i, cos_i)
}

/// computes the angle in radians with respect to the positive x-axis
pub fn vector_to_angle(vec: Vec2) -> fxx {
    fxx::atan2(-vec.y, -vec.x) + PI
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

/// -2 turns into max-2
/// also negatively wraps around:
/// The range is assumed to be `[0..max)`
pub fn wrap_around(i: i32, max: usize) -> usize {
    if i < 0 {
        max - (((i.abs() - 1) as usize) % max) - 1
    } else {
        (i as usize) % max
    }
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

/// uses wrap_around
/// displace all items in a list / vec by offset, and by wrapping around any out of bounds index.
/// `[1,2,3,4,5], offset=2` -> ` [3,4,5,1,2]`  
pub fn cycle_offset<T: Clone>(list: Vec<T>, offset: i32) -> Vec<T> {
    let len = list.len();
    let mut result = Vec::with_capacity(len);
    for i in 0..len as i32 {
        let item = list[wrap_around(i + offset, len)].clone();
        result.push(item);
    }
    result
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
    (start, end)
}

/// I know this is weird, I just want to go on with my life without worrying about NaN or whatever
pub fn partial_clamp<T: PartialOrd>(a: T, lower: T, upper: T) -> T {
    let lowered = match a.partial_cmp(&lower) {
        Some(res) => match res {
            std::cmp::Ordering::Less => lower,
            std::cmp::Ordering::Equal => lower,
            std::cmp::Ordering::Greater => a,
        },
        None => lower,
    };
    match lowered.partial_cmp(&upper) {
        Some(res) => match res {
            std::cmp::Ordering::Less => lowered,
            std::cmp::Ordering::Equal => upper,
            std::cmp::Ordering::Greater => upper,
        },
        None => lowered,
    }
}

///////////////////////////////////////////////////////////////////////////////

#[inline]
pub fn lerp(t: fxx, a: fxx, b: fxx) -> fxx {
    a + t * (b - a)
}

#[inline]
pub fn normalize(t: fxx, start: fxx, end: fxx) -> fxx {
    (t - start) / (end - start)
}

#[inline]
pub fn remap(
    t: fxx,
    from_start: fxx,
    from_end: fxx,
    to_start: fxx,
    to_end: fxx,
    clamped: bool,
) -> fxx {
    let mut norm = normalize(t, from_start, from_end);
    if clamped {
        norm = fxx::clamp(norm, 0.0, 1.0);
    }
    lerp(norm, to_start, to_end)
}

#[inline]
pub fn iter_n_times(start: fxx, end: fxx, steps: usize) -> impl Iterator<Item = fxx> {
    let delta = (end - start) / (steps - 1) as fxx;
    (0..steps).map(move |i| start + delta * i as fxx)
}

///
#[inline]
pub fn iter_by_delta(start: fxx, end: fxx, delta: fxx) -> impl Iterator<Item = fxx> {
    let steps = ((end - start) / delta).floor() as usize + 1;
    (0..steps).map(move |i| start + delta * i as fxx)
}
