/// A rig to be used with the camera,
/// Using two angles to deal with latitude (up/down) and longiture (left/right) rotations
use bevy::{
    input::mouse::{MouseMotion, MouseWheel}, prelude::{Component, Input, EventReader, MouseButton, KeyCode, Res, Query, Transform, Camera, With, default}, time::Time, window::CursorMoved,
};
use bevy_inspector_egui::Inspectable;
use crate::{math::spherical_to_cartesian, smoothing::Dropoff, kernel::{fxx, PI, FRAC_PI_2, Vec3, Vec2}};

const SPEED: fxx = 10.0;
const EPSILON: fxx = 0.0001;

const LERP_TOLERANCE: fxx = 0.001;
// const ROT_DELTA_DROPOFF:  fxx = 0.89;

const MOUSE_ROTATE_POWER: fxx = 0.0015;
const MOUSE_SCROLL_POWER: fxx = 0.04;

#[derive(Component, Inspectable, Debug)]
pub struct Rig {
    pub pos: Vec3,
    pub dis: Dropoff<fxx>,
    pub rot_x: Dropoff<fxx>, // incl, azi
    pub rot_y: Dropoff<fxx>, // incl, azi
    pub has_updated: bool,
    pub controls_active: bool,

    mouse_x: fxx,
    mouse_y: fxx,
}

impl Default for Rig {
    fn default() -> Self {
        Self {
            pos: Vec3::ZERO,
            dis: Dropoff::new(50.0, 1.0, 100.0, LERP_TOLERANCE),
            rot_x: Dropoff::new(PI * 0.85, 0.0, 0.0, LERP_TOLERANCE),
            rot_y: Dropoff::new(PI * 0.40, EPSILON, PI - EPSILON, LERP_TOLERANCE),
            has_updated: false,
            controls_active: true,

            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }
}

// private
impl Rig {
    pub fn get_rot_x() {}

    pub fn set_rot_delta(&mut self, delta_x: fxx, delta_y: fxx) {
        self.rot_x.set_delta(delta_x);
        self.rot_y.set_delta(delta_y);
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

        // rotation
        let rot_speed = 1.;
        if key_input.pressed(KeyCode::Down) {
            self.rot_y.set_delta(-rot_speed * dt);
            changed = true;
        }
        if key_input.pressed(KeyCode::Up) {
            self.rot_y.set_delta(rot_speed * dt);
            changed = true;
        }
        if key_input.pressed(KeyCode::Left) {
            self.rot_x.set_delta(-rot_speed * dt);
            changed = true;
        }
        if key_input.pressed(KeyCode::Right) {
            self.rot_x.set_delta(rot_speed * dt);
            changed = true;
        }
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
        
        if control && mouse_input.pressed(MouseButton::Right) || mouse_input.pressed(MouseButton::Middle) {
            // rotate
            let mut delta = Vec2::ZERO;
            for event in mouse_motion_events.iter() {
                delta += event.delta;
            }
            self.set_rot_delta(delta.x * MOUSE_ROTATE_POWER, -delta.y * MOUSE_ROTATE_POWER);
        } else if control && mouse_input.pressed(MouseButton::Left) {
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
        for event in mouse_wheel_events.iter() {
            let y_normalized = if event.y > EPSILON { 1.0 } else { -1.0 };
            self.dis
                .set_delta(self.dis.get() * y_normalized * -MOUSE_SCROLL_POWER);
        }

        false
    }
}

// public
impl Rig {
    pub fn new() -> Rig {
        Rig { ..default() }
    }

    /// for smoothening the movement,, we delay certain values.
    fn update_smooths(&mut self, dt: fxx) {
        let fps = 1.0 / dt; // = fps
        let smooth_frames = fps * 0.5; // represent the number of frames of a smoothening
        let tolerance_state = 0.0001; // represents the 'more or less zero' state of a smoothening
        let factor = fxx::powf(tolerance_state, 1.0 / smooth_frames);

        self.rot_x.tick(factor);
        self.rot_y.tick_clamped(factor);
        self.dis.tick_clamped(factor);
    }

    pub fn update(
        key_input: Res<Input<KeyCode>>,
        mouse_input: Res<Input<MouseButton>>,
        mut _cursor_moved_events: EventReader<CursorMoved>,
        mut mouse_motion_events: EventReader<MouseMotion>,
        mut mouse_wheel_events: EventReader<MouseWheel>,
        time: Res<Time>,
        mut query: Query<(&mut Transform, &mut Rig), With<Camera>>,
    ) {
        let dt = time.delta_seconds().into();

        for (mut tf, mut rig) in &mut query {
            // first, update x and y
            // for e in cursor_moved_events.iter() {

            // }

            // detect changes based on input
            if rig.controls_active {
                let _change_keys = rig.update_key_controls(&key_input, dt);
                let _change_mouse = rig.update_mouse_controls(
                    &key_input,
                    dt,
                    &mouse_input,
                    &mut mouse_motion_events,
                    &mut mouse_wheel_events,
                );
            }

            // if changes have occurred, recalculate the transform of the camera
            let mut vec = spherical_to_cartesian(*rig.rot_y.get(), *rig.rot_x.get());
            vec = rig.pos + (vec * *rig.dis.get());
            tf.translation = vec.into();
            tf.look_at(rig.pos, Vec3::Z);
            rig.update_smooths(dt);
        }
    }
}
