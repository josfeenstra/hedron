// this plugin instanciates all custom materials used by hedron

use bevy::prelude::*;

use super::{FaceMaterial, InstanceMaterialPlugin, LineMaterial};

pub struct HedronMaterialPlugin;

impl Plugin for HedronMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InstanceMaterialPlugin);
        app.add_plugin(MaterialPlugin::<LineMaterial>::default());
        app.add_plugin(MaterialPlugin::<FaceMaterial>::default());
    }
}
