use std::collections::HashMap;

use super::{
    extract_vertices, InstanceData, InstanceMaterialData, InstanceMaterialPlugin, LineMaterial,
};
use bevy::{prelude::*, render::view::NoFrustumCulling};

#[derive(Resource, Default)]
pub struct GeoRenderer {
    to_add: Vec<(String, Mesh, Color, f32)>,
    // to_update: Vec<(String, Mesh, Color, f32)>,
    to_remove: Vec<Entity>,

    rendered: HashMap<String, Entity>,
}

// a simple (debug) renderer for geometry.
impl GeoRenderer {
    pub fn new() -> GeoRenderer {
        GeoRenderer { ..default() }
    }

    // TODO add with key, to make it replaceable
    pub fn set<M: Into<Mesh>>(&mut self, key: &str, renderable: M, color: Color, width: f32) {
        if let Some(_) = self.rendered.get(key) {
            self.delete(key);
        }
        self.to_add
            .push((key.to_owned(), renderable.into(), color, width));
    }

    pub fn set_quick<M: Into<Mesh>>(&mut self, key: &str, renderable: M) {
        self.set(key, renderable, Color::WHITE, 1.0)
    }

    pub fn delete(&mut self, id: &str) -> Option<()> {
        if self.rendered.contains_key(id) {
            let e = self.rendered.remove(id).unwrap();
            self.to_remove.push(e);
            Some(())
        } else {
            None
        }
    }

    fn update_system(
        mut c: Commands,
        mut gr: ResMut<GeoRenderer>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut l_materials: ResMut<Assets<LineMaterial>>,
    ) {
        // REMOVE
        while let Some(entity) = gr.to_remove.pop() {
            c.entity(entity).despawn_recursive();
        }

        // ADD
        while let Some((id, mut mesh, color, width)) = gr.to_add.pop() {
            // spawn something different based on the type of mesh
            let entity = match mesh.primitive_topology() {
                bevy::render::render_resource::PrimitiveTopology::PointList => {
                    // we don't have a good way of rendering points just yet...
                    // lets use instanced rendering
                    // This is slightly inefficient, because we now needlessly translate back from a mesh
                    let points = extract_vertices(&mut mesh).expect("points lists should verts");
                    c.spawn((
                        meshes.add(Mesh::from(shape::Icosphere {
                            radius: 0.1,
                            subdivisions: 1,
                        })),
                        SpatialBundle::VISIBLE_IDENTITY,
                        InstanceMaterialData(
                            points
                                .data
                                .iter()
                                .map(|v| InstanceData {
                                    position: *v,
                                    scale: width,
                                    color: color.as_rgba_f32(),
                                })
                                .collect(),
                        ),
                        NoFrustumCulling,
                        Name::new("instanced points"),
                    ))
                    .id()
                }
                bevy::render::render_resource::PrimitiveTopology::LineList => c
                    .spawn((
                        MaterialMeshBundle {
                            mesh: meshes.add(mesh),
                            material: l_materials.add(LineMaterial { color }),
                            ..default()
                        },
                        Name::new("Lines"),
                    ))
                    .id(),
                bevy::render::render_resource::PrimitiveTopology::LineStrip => c
                    .spawn((
                        MaterialMeshBundle {
                            mesh: meshes.add(mesh),
                            material: l_materials.add(LineMaterial { color }),
                            ..default()
                        },
                        Name::new("Line Strip"),
                    ))
                    .id(),
                bevy::render::render_resource::PrimitiveTopology::TriangleList => c
                    .spawn((
                        MaterialMeshBundle {
                            mesh: meshes.add(mesh),
                            material: l_materials.add(LineMaterial { color }),
                            ..default()
                        },
                        Name::new("Line Strip"),
                    ))
                    .id(),
                bevy::render::render_resource::PrimitiveTopology::TriangleStrip => c
                    .spawn((
                        MaterialMeshBundle {
                            mesh: meshes.add(mesh),
                            material: l_materials.add(LineMaterial { color }),
                            ..default()
                        },
                        Name::new("Line Strip"),
                    ))
                    .id(),
            };
            if gr.rendered.contains_key(&id) {
                println!("key already exists! update scheduling mistake occurred");
            } else {
                gr.rendered.insert(id, entity);
            }
        }

        // TODO: This makes updates more efficient
        // // UPDATE
        // while let Some((id, mesh, color, width)) = gr.to_update.pop() {
        //     // if we know it, update it
        //     if let Some(entity) = gr.rendered.get(&id) {
        //         if let Ok(handle) = mesh_handles.get(*entity) {
        //             if let Some(mesh) = meshes.get_mut(handle) {
        //                 println!("todo update mesh!");
        //                 // TODO: replace geodata
        //                 // mesh
        //             }
        //         }

        //         // update potential line material
        //         if let Ok(handle) = material_handles.get(*entity) {
        //             if let Some(mesh) = l_materials.get_mut(handle) {
        //                 println!("todo update material!");
        //                 // TODO: replace geodata
        //                 // mesh
        //             }
        //         }
        //     }
        // }
    }
}

pub struct GeoRendererPlugin;

impl Plugin for GeoRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InstanceMaterialPlugin);
        app.insert_resource(GeoRenderer::default());
        app.add_system(GeoRenderer::update_system);
    }
}
