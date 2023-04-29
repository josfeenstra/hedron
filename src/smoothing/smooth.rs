use super::{Smoothable, State};
use crate::kernel::fxx;
use bevy_inspector_egui::InspectorOptions;

#[derive(InspectorOptions, Debug)]
pub struct Smooth<T> {
    // from: T,
    b: T,
    t: T,
    delta: fxx,
    tol: fxx,
    state: State,
}

impl<T: Smoothable + Copy> Smooth<T> {
    pub fn new_finished(v: T, delta: fxx, tol: fxx) -> Self {
        Self {
            // from: v,
            t: v,
            b: v,
            delta,
            tol,
            state: State::Finished,
        }
    }

    pub fn new_running(from: T, to: T, delta: fxx, tol: fxx) -> Self {
        Self {
            // from,
            t: from,
            b: to,
            delta,
            tol,
            state: State::Running,
        }
    }

    pub fn tick(&mut self) {
        if self.state == State::Finished {
            return;
        }
        self.t = self.t.lerp(&self.b, self.delta);
        self.state = if self.t.tol_equals(&self.b, self.tol) {
            State::Finished
        } else {
            State::Running
        };
    }

    #[inline]
    pub fn get(&self) -> &T {
        &self.t
    }

    pub fn set(&mut self, v: T) {
        // self.from = v;
        self.b = v;
        self.state = State::Running;
    }
}
