use std::{
    f32::consts::PI,
    ops::{Add, Div, Mul, Sub},
};

use crate::kernel::fxx;

/// A second order system for producing an organic, realisticly delayed response to any input.
/// Written by Jabuwu, credits to T3ssel8r
#[derive(Debug, Default, Clone, Copy)]
pub struct SecondOrder<T> {
    pub input_previous: T,
    pub output: T,
    pub output_velocity: T,

    pub k1: fxx,
    pub k2: fxx,
    pub k3: fxx,
}

impl<T> SecondOrder<T>
where
    T: Default
        + Copy
        + Sub<T, Output = T>
        + Div<fxx, Output = T>
        + Add<T, Output = T>
        + Mul<fxx, Output = T>,
{
    pub fn new(initial: T, k1: fxx, k2: fxx, k3: fxx) -> SecondOrder<T> {
        SecondOrder {
            input_previous: initial,
            output: initial,
            output_velocity: T::default(),
            k1,
            k2,
            k3,
        }
    }

    /// frequency, response and dampning are nicer parameters to play with
    pub fn new_frequency_response(
        initial: T,
        frequency: f32,
        response: f32,
        damping: f32,
    ) -> SecondOrder<T> {
        let mut second_order = SecondOrder::new(initial, 0., 0., 0.);
        second_order.set_frequency_response(frequency, response, damping);
        second_order
    }

    pub fn set_frequency_response(&mut self, frequency: fxx, response: fxx, damping: fxx) {
        self.k1 = damping / (PI * frequency);
        self.k2 = 1. / (2. * PI * frequency).powf(2.);
        self.k3 = response * damping / (2. * PI * frequency);
    }

    pub fn update(&mut self, input: T, delta_seconds: fxx) -> T {
        if delta_seconds == 0. {
            return self.output;
        }
        let k2_stable = self
            .k2
            .max(delta_seconds * delta_seconds / 2. + delta_seconds * self.k1 / 2.)
            .max(delta_seconds * self.k1);
        let vec_velocity = (input - self.input_previous) / delta_seconds;
        self.input_previous = input;
        self.output = self.output + self.output_velocity * delta_seconds;
        self.output_velocity = self.output_velocity
            + (input + vec_velocity * self.k3 - self.output - self.output_velocity * self.k1)
                / k2_stable
                * delta_seconds;
        self.output
    }
}
