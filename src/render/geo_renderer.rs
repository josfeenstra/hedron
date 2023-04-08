use std::{collections::HashMap};
use super::{
    extract_vertices, FaceMaterial, InstanceData, InstanceMaterialData,
    LineMaterial,
};
use crate::kernel::fxx;
use bevy::{prelude::*, render::view::NoFrustumCulling};

#[derive(Resource, Default)]
pub struct GeoRenderer<M:Material + Default> {
    // to_add: DynStack<(String, Mesh, dyn Material)>,
    to_add: Vec<(String, Mesh, Color, fxx, bool)>,
    to_remove: Vec<Entity>,
    rendered: HashMap<String, Entity>,
    pub custom_mat: Handle<M>,
}

// a simple (debug) renderer for geometry.
impl<M:Material + Default> GeoRenderer<M> {
    
    pub fn new() -> Self {
        Self { ..default() }
    }

    // TODO add with key, to make it replaceable
    pub fn set<Meshable: Into<Mesh>>(&mut self, key: &str, meshable: Meshable, color: Color, width: fxx, custom: bool) {
        if self.rendered.get(key).is_some() {
            self.delete(key);
        }

        // check if it hasnt been set twice
        if self.to_add.iter().any(|(existing_key, _, _, _, _)| existing_key == key) {
            // TODO this only goes wrong if the 'update_system' is too late.
            println!("WARN: we are re-adding things");
            return;
        }

        self.to_add.push((key.to_owned(), meshable.into(), color, width, custom));
    }

    pub fn set_mat(&mut self, mat: Handle<M>) {
        self.custom_mat = mat;
    }

    pub fn set_quick<Meshable: Into<Mesh>>(&mut self, key: &str, renderable: Meshable) {
        self.set(key, renderable, Color::WHITE, 1.0, false)
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

    pub fn update_system(
        mut c: Commands,
        mut gr: ResMut<GeoRenderer<M>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut _materials: ResMut<Assets<StandardMaterial>>,
        mut l_materials: ResMut<Assets<LineMaterial>>,
        mut f_materials: ResMut<Assets<FaceMaterial>>,
    ) {
        // REMOVE
        while let Some(entity) = gr.to_remove.pop() {
            c.entity(entity).despawn_recursive();
        }

        // ADD
        while let Some((id, mut mesh, color, width, use_custom_material)) = gr.to_add.pop() {
            // spawn something different based on the type of mesh
            let entity = match mesh.primitive_topology() {
                bevy::render::render_resource::PrimitiveTopology::PointList => {
                    // we don't have a good way of rendering points just yet...
                    // lets use instanced rendering
                    // This is slightly inefficient, because we now needlessly translate back from a mesh
                    let points = extract_vertices(&mut mesh).expect("points lists should verts");
                    // let mesh: Mesh = ico.into();
                    
                    let Result::Ok(mesh) = Mesh::try_from(shape::Icosphere {
                        radius: 0.1,
                        subdivisions: 1,
                    }) else {
                       continue; 
                    }; 

                    c.spawn((
                        meshes.add(mesh),
                        SpatialBundle::INHERITED_IDENTITY,
                        InstanceMaterialData(
                            points
                                .iter()
                                .map(|v| InstanceData {
                                    position: *v,
                                    scale: width,
                                    color: color.as_rgba_f32(),
                                })
                                .collect(),
                        ),
                        NoFrustumCulling,
                        Name::new(id.clone()),
                    )).id()
                }
                bevy::render::render_resource::PrimitiveTopology::LineList => c
                    .spawn((
                        MaterialMeshBundle {
                            mesh: meshes.add(mesh),
                            material: l_materials.add(LineMaterial { color }),
                            ..default()
                        },
                        Name::new(id.clone()),
                    ))
                    .id(),
                bevy::render::render_resource::PrimitiveTopology::LineStrip => c
                    .spawn((
                        MaterialMeshBundle {
                            mesh: meshes.add(mesh),
                            material: l_materials.add(LineMaterial { color }),
                            ..default()
                        },
                        Name::new(id.clone()),
                    ))
                    .id(),
                bevy::render::render_resource::PrimitiveTopology::TriangleList => 
                    match use_custom_material {
                        true => c.spawn((
                            MaterialMeshBundle {
                                mesh: meshes.add(mesh),
                                material: gr.custom_mat.clone(),
                                ..default()
                            },
                            Name::new(id.clone()),
                        )).id(),   
                        false => c
                        .spawn((
                            MaterialMeshBundle {
                                mesh: meshes.add(mesh),
                                material: f_materials.add(FaceMaterial { color }),
                                // material: materials.add(StandardMaterial {
                                //     base_color: color,
                                //     unlit: true,
                                //     alpha_mode: AlphaMode::Opaque,
                                //     depth_bias: -1.0,
                                //     ..default()
                                // }),
                                ..default()
                            },
                            Name::new(id.clone()),
                        )).id(),
                    },
                bevy::render::render_resource::PrimitiveTopology::TriangleStrip => c
                    .spawn((
                        MaterialMeshBundle {
                            mesh: meshes.add(mesh),
                            material: f_materials.add(FaceMaterial { color }),
                            ..default()
                        },
                        Name::new(id.clone()),
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

#[derive(Default)]
pub struct GeoRendererPlugin<M> {
    pub dummy: M // this is weird, dont look at it
}

impl<M: Material + Default> Plugin for GeoRendererPlugin<M> {
    fn build(&self, app: &mut App) {
        app.insert_resource(GeoRenderer::<M>::default());
        app.add_system(GeoRenderer::<M>::update_system);
    }
}
