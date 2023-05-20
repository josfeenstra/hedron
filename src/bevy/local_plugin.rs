use bevy::prelude::*;

use super::ecs_helpers::despawn_flagged;

/// a non-global plugin, which only runs during a given 'gamestate'.
/// this is not a real bevy plugin, the syntax just looks simiar for the sake of clarity
pub trait LocalPlugin {
    /// T: the state associated with this scene
    fn build<T: States + Copy>(app: &mut App, state: T) {
        app.add_system(on_enter.in_schedule(OnEnter(state)));
        app.add_system(despawn_flagged::<Local>.in_schedule(OnExit(state)));
        app.add_systems((on_update_1, on_update_2).in_set(OnUpdate(state)));
    }
}

#[derive(Component)]
struct Local;

pub trait AppScenePluginExtention {
    fn add_local_plugin<T: LocalPlugin, S: States + Copy>(
        &mut self,
        associated_state: S,
        state_plugin: T,
    );
}

impl AppScenePluginExtention for App {
    fn add_local_plugin<T: LocalPlugin, S: States + Copy>(
        &mut self,
        associated_state: S,
        _scene_plugin: T,
    ) {
        T::build(self, associated_state);
    }
}

// runs on entering this state
fn on_enter() {
    todo!("enter!");
}

// runs every update tick in this state
fn on_update_1() {
    todo!("update 1!");
}
// runs every update tick in this state
fn on_update_2() {
    todo!("update 2!");
}
