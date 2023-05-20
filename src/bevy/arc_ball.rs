use std::ops::Range;

use crate::{
    kernel::{fxx, Vec2, Vec3, FRAC_PI_2, PI},
    math::{spherical_to_cartesian, Organic},
};
/// A rig to be used with the camera,
/// Using two angles to deal with latitude (up/down) and longiture (left/right) rotations
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use bevy_inspector_egui::InspectorOptions;

const SPEED: fxx = 10.0;
const EPSILON: fxx = 0.0001;

// const LERP_TOLERANCE: fxx = 0.001;
// const ROT_DELTA_DROPOFF:  fxx = 0.89;

const MOUSE_ROTATE_POWER: fxx = 0.0015;
const MOUSE_SCROLL_POWER: fxx = 0.01;

#[derive(Component, InspectorOptions, Debug)]
pub struct ArcBall {
    pub pos: Vec3,
    pub dis: Organic<fxx>,
    pub dis_range: Range<fxx>,
    pub rot_x: Organic<fxx>, // incl, azi
    pub rot_y: Organic<fxx>, // incl, azi
    pub rot_y_range: Range<fxx>,
    pub has_updated: bool,
    pub active_look_controls: bool, // allow 'looking around' from the position of the rig (rotation and zooming)
    pub active_move_controls: bool, // allow movement of the rig

    pub mouse_x: fxx,
    pub mouse_y: fxx,
}

impl Default for ArcBall {
    fn default() -> Self {
        Self {
            pos: Vec3::ZERO,
            dis: Organic::new(50.0, 1.0, 1.0, 1.0),
            dis_range: 1.0..100.0,
            rot_x: Organic::new(PI * 0.85, 1.0, 1.0, 1.0),
            rot_y: Organic::new(PI * 0.40, 1.0, 1.0, 1.0),
            rot_y_range: EPSILON..PI - EPSILON,
            has_updated: false,
            active_look_controls: true,
            active_move_controls: true,

            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }
}

// private
impl ArcBall {
    pub fn get_rot_x() {}

    pub fn set_rot_delta(&mut self, delta_x: fxx, delta_y: fxx) {
        self.rot_x += delta_x;
        self.rot_y
            .add_clamped(delta_y, self.rot_y_range.start, self.rot_y_range.end);
    }

    // fn set_rot_clamped(&mut self, azi: fxx, incl: fxx) {
    //     self.rot = Vec2::new(
    //         azi,
    //         incl.clamp(EPSILON, PI - EPSILON)
    //     );
    // }

    fn rel_x(&self) -> Vec3 {
        let (sin_a, cos_a) = (self.rot_x.get() + FRAC_PI_2).sin_cos();
        Vec3::new(sin_a, cos_a, 0.0)
    }

    fn rel_y(&self) -> Vec3 {
        let (sin_a, cos_a) = self.rot_x.get().sin_cos();
        Vec3::new(sin_a, cos_a, 0.0)
    }

    fn update_key_controls(&mut self, key_input: &Input<KeyCode>, dt: fxx) -> bool {
        // movement
        let mut changed = false;
        if self.active_move_controls {
            if key_input.pressed(KeyCode::A) {
                self.pos += self.rel_x() * SPEED * dt;
                changed = true;
            }
            if key_input.pressed(KeyCode::D) {
                self.pos -= self.rel_x() * SPEED * dt;
                changed = true;
            }
            if key_input.pressed(KeyCode::S) {
                self.pos += self.rel_y() * SPEED * dt;
                changed = true;
            }
            if key_input.pressed(KeyCode::W) {
                self.pos -= self.rel_y() * SPEED * dt;
                changed = true;
            }
            if key_input.pressed(KeyCode::Q) {
                self.pos.z -= SPEED * dt;
                changed = true;
            }
            if key_input.pressed(KeyCode::E) {
                self.pos.z += SPEED * dt;
                changed = true;
            }
        }

        let rot_speed = 1.;

        if self.active_look_controls {
            if key_input.pressed(KeyCode::Down) {
                self.rot_y += -rot_speed * dt;
                changed = true;
            }
            if key_input.pressed(KeyCode::Up) {
                self.rot_y += rot_speed * dt;
                changed = true;
            }
            if key_input.pressed(KeyCode::Left) {
                self.rot_x += -rot_speed * dt;
                changed = true;
            }
            if key_input.pressed(KeyCode::Right) {
                self.rot_x += rot_speed * dt;
                changed = true;
            }
        }

        // rotation

        changed
    }

    fn update_mouse_controls(
        &mut self,
        key_input: &Input<KeyCode>,
        _dt: fxx,
        mouse_input: &Input<MouseButton>,
        mouse_motion_events: &mut EventReader<MouseMotion>,
        mouse_wheel_events: &mut EventReader<MouseWheel>,
    ) -> bool {
        // modifiers
        let control = key_input.any_pressed([KeyCode::LControl, KeyCode::RControl]);

        if self.active_look_controls && control && mouse_input.pressed(MouseButton::Right)
            || mouse_input.pressed(MouseButton::Middle)
        {
            // rotate
            let mut delta = Vec2::ZERO;
            for event in mouse_motion_events.iter() {
                delta += event.delta;
            }
            self.set_rot_delta(delta.x * MOUSE_ROTATE_POWER, -delta.y * MOUSE_ROTATE_POWER);
        } else if self.active_move_controls && control && mouse_input.pressed(MouseButton::Left) {
            // pan
            let mut delta = Vec2::ZERO;
            for event in mouse_motion_events.iter() {
                delta += event.delta;
            }
            self.pos -= self.rel_x() * delta.x * SPEED * 0.001;
            self.pos += self.rel_y() * delta.y * SPEED * 0.001;
        } else {
            // consume
            for _ in mouse_motion_events.iter() {}
        }

        // zooming
        if self.active_look_controls {
            for event in mouse_wheel_events.iter() {
                let y_normalized = if event.y > EPSILON { 1.0 } else { -1.0 };
                self.dis.add_clamped(
                    self.dis.get() * y_normalized * -MOUSE_SCROLL_POWER,
                    self.dis_range.start,
                    self.dis_range.end,
                );
            }
        }

        false
    }
}

// public
impl ArcBall {
    pub fn new() -> ArcBall {
        ArcBall { ..default() }
    }

    /// for smoothening the movement,, we delay certain values.
    fn update_smooths(&mut self, dt: fxx) {
        // let fps = 1.0 / dt; // = fps
        // let smooth_frames = fps * 0.5; // represent the number of frames of a smoothening
        // let tolerance_state = 0.0001; // represents the 'more or less zero' state of a smoothening
        // let factor = fxx::powf(tolerance_state, 1.0 / smooth_frames);

        self.rot_x.update(dt);
        self.rot_y.update(dt);
        self.dis.update(dt);
    }

    pub fn update(
        key_input: Res<Input<KeyCode>>,
        mouse_input: Res<Input<MouseButton>>,
        mut _cursor_moved_events: EventReader<CursorMoved>,
        mut mouse_motion_events: EventReader<MouseMotion>,
        mut mouse_wheel_events: EventReader<MouseWheel>,
        time: Res<Time>,
        mut query: Query<(&mut Transform, &mut ArcBall), With<Camera>>,
    ) {
        let dt = time.delta_seconds();

        for (mut tf, mut rig) in &mut query {
            // first, update x and y
            // for e in cursor_moved_events.iter() {

            // }

            // detect changes based on input

            let _change_keys = rig.update_key_controls(&key_input, dt);
            let _change_mouse = rig.update_mouse_controls(
                &key_input,
                dt,
                &mouse_input,
                &mut mouse_motion_events,
                &mut mouse_wheel_events,
            );

            // if changes have occurred, recalculate the transform of the camera
            let mut vec = spherical_to_cartesian(rig.rot_y.get(), rig.rot_x.get());
            vec = rig.pos + (vec * rig.dis.get());
            tf.translation = vec;
            tf.look_at(rig.pos, Vec3::Z);
            rig.update_smooths(dt);
        }
    }
}
