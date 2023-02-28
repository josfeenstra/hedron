use std::ops::Range;

use crate::kernel::fxx;

use super::*;

// add some of these functions to std range
pub trait Range1 {
    type Iter: Iterator<Item=fxx>;

    fn normalize(&self, t: fxx) -> fxx;
    fn norm_clamp(&self, t: fxx) -> fxx;
    fn lerp(&self, t: fxx) -> fxx;
    fn remap(&self, other: &Range<fxx>, n: fxx, clamped: bool) -> fxx;

    fn iter_n_times(&self, steps: usize) -> Self::Iter; 
    fn iter_by_delta(&self, delta: fxx) -> Self::Iter; 
    fn expand_to(&mut self, t: fxx);
}

impl Range1 for Range<fxx> {
    
    // type Iter = Iterator<fxx>;
    type Iter = Box<dyn Iterator<Item=fxx>>;

    fn normalize(&self, t: fxx) -> fxx {
        normalize(t, self.start, self.end)
    }

    fn norm_clamp(&self, t: fxx) -> fxx {
        normalize(t, self.start, self.end).clamp(self.start, self.end)
    }

    fn lerp(&self, t: fxx) -> fxx {
        lerp(t, self.start, self.end)
    }

    fn remap(&self, other: &Range<fxx>, t: fxx, clamped: bool) -> fxx {
        remap(t, self.start, self.end, other.start, other.end, clamped)
    }

    fn expand_to(&mut self, t: fxx) {
        self.start = self.start.min(t);
        self.end = self.end.max(t);
    }

    fn iter_n_times(&self, steps: usize) -> Self::Iter {
        Box::new(iter_n_times(self.start, self.end, steps))
    }

    fn iter_by_delta(&self, delta: fxx) -> Self::Iter {
        Box::new(iter_by_delta(self.start, self.end, delta))
    }
}

