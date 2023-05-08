use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

use crate::kernel::fxx;

use super::{partial_clamp, SecondOrder};

#[derive(Debug, Default, Clone, Copy)]
pub struct Organic<T> {
    pub system: SecondOrder<T>,
    pub target: T,
}

impl<T> Organic<T>
where
    T: Default
        + Copy
        + PartialOrd
        + Sub<T, Output = T>
        + Div<fxx, Output = T>
        + Add<T, Output = T>
        + AddAssign<fxx>
        + Mul<fxx, Output = T>,
{
    pub fn new(initial: T, f: fxx, r: fxx, d: fxx) -> Organic<T> {
        Organic {
            system: SecondOrder::new_frequency_response(initial, f, r, d),
            target: initial,
        }
    }

    pub fn get(&self) -> T {
        self.system.output
    }

    /// shorthand for adding to target in a clamped manner.
    pub fn add_clamped(&mut self, delta: T, lower: T, upper: T) {
        self.target = partial_clamp(self.target + delta, lower, upper);
    }

    /// set the target
    pub fn set(&mut self, t: T) {
        self.target = t;
    }

    /// Force the system to `t`, by setting the target AND current position to t.
    /// reset velocity too, so the system is in rest
    pub fn set_force(&mut self, t: T) {
        self.target = t;
        self.system.input_previous = t;
        self.system.output = t;
        self.system.output_velocity = T::default();
    }

    pub fn update(&mut self, dt: fxx) {
        self.system.update(self.target, dt);
    }
}

impl<T> AddAssign<fxx> for Organic<T>
where
    T: AddAssign<fxx>,
{
    #[inline]
    fn add_assign(&mut self, rhs: fxx) {
        self.target.add_assign(rhs);
    }
}

impl<T> Add<fxx> for Organic<T>
where
    T: Add<fxx, Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(mut self, rhs: fxx) -> Self::Output {
        self.target = self.target + rhs;
        self
    }
}

impl<T> SubAssign<fxx> for Organic<T>
where
    T: SubAssign<fxx>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: fxx) {
        self.target.sub_assign(rhs);
    }
}

impl<T> Sub<fxx> for Organic<T>
where
    T: Sub<fxx, Output = T>,
{
    type Output = Self;

    #[inline]
    fn sub(mut self, rhs: fxx) -> Self::Output {
        self.target = self.target - rhs;
        self
    }
}
