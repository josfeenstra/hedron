////////////////////////////////////////////////////////////////////////////
// these are all shaper functions
// variations of a 0 to 1 interpolation, creating a nice effect
// from : https://www.youtube.com/watch?v=YJB1QnEmlTs
////////////////////////////////////////////////////////////////////////////

use crate::kernel::{fxx, FRAC_PI_2};

use super::lerp;

pub enum Shaper {
    Linear,
    Log,
    Smooth,
    QuadIn,
    QuadOut,
    QuadInOut,
    Parabola(i32),
    SkewedSmooth(fxx),
    CubicBezier(fxx, fxx),
}

impl Shaper {
    pub fn eval(&self, t: fxx) -> fxx {
        match self {
            Shaper::Linear => lerp(t, 0.0, 1.0),
            Shaper::Log => log_lerp(t, 0.0, 1.0),
            Shaper::Smooth => smooth(t),
            Shaper::QuadIn => quad_in(t),
            Shaper::QuadOut => quad_out(t),
            Shaper::QuadInOut => quad_in_out(t),
            Shaper::Parabola(k) => parabola(t, *k),
            Shaper::SkewedSmooth(s) => skewed_smooth_step(t, *s),
            Shaper::CubicBezier(w_start, w_end) => lerp_cubic_bezier(t, *w_start, *w_end),
        }
    }
}

/// logaritmic lerping
#[inline]
pub fn log_lerp(t: fxx, from: fxx, to: fxx) -> fxx {
    let zoom = lerp(t, fxx::ln(from), fxx::ln(to));
    fxx::exp(zoom)
}

/// Fade function as defined by Ken Perlin.
/// equals to 6t^5 - 15t^4 + 10t^3
#[inline]
pub fn smooth(t: fxx) -> fxx {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
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
pub fn quad_in_out(t: fxx) -> fxx {
    // this mixing is very interesting for what I want to do with the wobble
    lerp(t, quad_in(t), quad_out(t))
}

#[inline]
pub fn parabola(t: fxx, k: i32) -> fxx {
    (4.0 * t * (1.0 - t)).powi(k)
}


/// bezier-interpolate 
/// with w_start, w_end, and t in [0..1], create a 2D unit bezier curve as (0,0), (start, 0), (1 - end,1), (1,1). 
/// interpolate this bezier using t, then return the x of this bezier
/// TODO: currenty, this is the opposite of what we want: speed at the edges, smooth in the middle... 
#[inline]
pub fn lerp_cubic_bezier(t: fxx, w_start: fxx, w_end: fxx) -> fxx {
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
pub fn skewed_smooth_step(t: fxx, s: fxx) -> fxx {
    // this mixing is very interesting for what I want to do with the wobble
    lerp(t, quad_in(t) * (1.0 - s), quad_out(t) * s)
}

#[inline]
pub fn elastic_out(t: fxx) -> fxx {
    fxx::sin(-13.0 * (t + 1.0) * FRAC_PI_2) * fxx::powf(2.0, -10.0 * t) + 1.0
}

pub fn bounce_out(mut t: fxx, nl: fxx, dl: fxx) -> fxx {
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

pub fn bounce_out_default(t: fxx) -> fxx {
    let nl = 7.5625;
    let dl = 2.75;
    bounce_out(t, nl, dl)
}

#[cfg(test)]
mod test {
    use crate::kernel::fxx;

    use super::lerp_cubic_bezier;


    #[test]
    fn test_bezier_int() {
        for i in 0..101 {
            let f = i as fxx / 100.0;

            let t = lerp_cubic_bezier(1.0, 1.0, f);
            println!("f {f}, t {t}");
        }
    }
}