use super::GizmoTag;
use bevy::{
    input::common_conditions::input_toggle_active, prelude::*, reflect::GetTypeRegistration,
};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

/// Add LocalPlugin to App
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

//////////////////////////////////////////// Simple Systems which are needed often with these types of local plugins

/// T: flag-component to use for despawning
pub fn despawn_flagged<T: Component>(mut c: Commands, mut entities: Query<Entity, With<T>>) {
    for e in &mut entities {
        c.entity(e).despawn_recursive();
    }
}

pub fn init_resource_system<R: Resource + Default>(mut c: Commands) {
    c.init_resource::<R>()
}

pub fn remove_resource_system<R: Resource + Default>(mut c: Commands) {
    c.remove_resource::<R>()
}

////////////////////////////////////////////

/// the common elements of a demo state plugin
pub fn build_demo_scene_plugin<R, M, T>(
    app: &mut App,
    state: T,
    _: R,
    on_resource_changed_system: impl IntoSystemAppConfig<M>,
) where
    R: Resource + Reflect + Clone + Default,
    T: States + Copy,
{
    // spawn a gizmo
    // TODO extract common logic: spawning tagged components on enter, removing these components on exit
    // app.add_system(crate::bevy::helpers::spawn_gizmo_system.in_schedule(OnEnter(state)))
    //     .add_system(despawn_flagged::<GizmoTag>.in_schedule(OnExit(state)));

    // TODO: spawn a grid

    // add the resource as a egui setting
    app.add_system(init_resource_system::<R>.in_schedule(OnEnter(state)))
        .add_system(remove_resource_system::<R>.in_schedule(OnExit(state)))
        .add_system(
            on_resource_changed_system.into_app_config().run_if(
                state_exists_and_equals(state).and_then(resource_exists_and_changed::<R>()),
            ),
        )
        .add_plugin(ResourceInspectorPlugin::<R>::new().run_if(state_exists_and_equals(state)));
}

// TODO use egui example stuff to figure out how to truly configure
// this the right way : on_enter returning one or multiple systems, then hooking it up in here
pub fn add_inspector_resource<T>(app: &mut App)
where
    T: Default + Resource + Reflect + GetTypeRegistration,
{
    app.init_resource::<T>().register_type::<T>().add_plugin(
        ResourceInspectorPlugin::<T>::new().run_if(input_toggle_active(false, KeyCode::Grave)),
    );
}
