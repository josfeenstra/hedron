use bevy_inspector_egui::{InspectorOptions};

use super::{Smoothable, State};
use crate::kernel::fxx;

// this is a smooth dropoff
#[derive(InspectorOptions, Debug)]
pub struct Dropoff<T> {
    // from: T,
    t: T,
    min: T,
    max: T,
    delta: T,
    delta_zero: T,
    tol: fxx,
    state: State,
}

// TODO make this copy less
impl<T: Smoothable + Copy + Default + std::fmt::Debug> Dropoff<T> {
    /**
     * acc: acceleration (actually, decelerations, use something in between 0 and 0.99)
     */
    pub fn new(v: T, min: T, max: T, tol: fxx) -> Self {
        Self {
            // from: v,
            t: v,
            delta: T::default(),
            delta_zero: T::default(),
            min,
            max,
            tol,
            state: State::Finished,
        }
    }

    pub fn tick(&mut self, drop: fxx) {
        if self.state == State::Finished {
            return;
        }
        self.t = self.t.add(self.delta);
        self._tick(drop);
    }

    pub fn tick_clamped(&mut self, drop: fxx) {
        if self.state == State::Finished {
            return;
        }
        self.t = self.t.add_clamped(self.delta, self.min, self.max);
        self._tick(drop);
    }

    fn _tick(&mut self, drop: fxx) {
        // println!("{}, {:?}, {:?}", dt, &self.delta_zero, &self.t);
        // dropoff-distance;

        // we want to know what the deceleration (dropoff) must be,
        // given delta time and the dropoff distance.
        // galileo will help us!:
        // s = ut + ½at²
        // and if I remember my high school maths, that means:
        // a = 2(s - ut) / t²
        // s: total distance traveled
        // u: speed
        //

        // let distance: fxx = (1.0_fxx).ln()

        self.delta = self.delta.mul(drop);
        self.state = if self.delta.tol_equals(&self.delta_zero, self.tol) {
            State::Finished
        } else {
            State::Running
        };
    }

    #[inline]
    pub fn get(&self) -> &T {
        &self.t
    }

    pub fn set_delta(&mut self, delta: T) {
        self.delta = delta;
        self.state = State::Running;
    }

    pub fn set(&mut self, delta: T) {
        self.t = delta;
        // TODO calculate where we need to set delta to, to get at 'destination'
    }
}
