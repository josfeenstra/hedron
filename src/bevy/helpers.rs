use bevy::prelude::*;
use bevy_math::vec3;

use crate::{
    core::Geometry,
    pts::Vectors,
    solid::{Polyhedron, VertPtr},
};

use super::{FaceMaterial, GeoRenderer, SpawnText};

// app.add_plugin(GeoRendererPlugin::<CellMat>::default());

// the georenderer uses the following plugins
// these are
// NOTE: thats now lifted to the shaderplugin
// app.add_plugin(MaterialPlugin::<LineMaterial>::default());
// app.add_plugin(MaterialPlugin::<FaceMaterial>::default());

// #[cfg(not(target_arch = "wasm32"))] // wasm doesn't like lines
// app.add_startup_system(ModellingPlugin::spawn_line_grid);

// app.add_startup_system(Self::create);
// app.add_startup_system(Self::create_polyhedron);

// the terrain

// debuggy things

pub fn _render_debug_polyhedron(
    c: &mut Commands,
    renderer: &mut GeoRenderer<FaceMaterial>,
    hedron: &mut Polyhedron,
) {
    // render all sorts of things about it
    hedron.print_structure();
    for (i, vert) in hedron.verts.iter_enum() {
        c.add(SpawnText::new(vert.pos, format!("v{}", i)));
    }

    for (i, edge) in hedron.edges.iter_enum() {
        let (start, end) = hedron.edge_verts(i);

        let point = start.lerp(end, 0.33);

        //  edge.face.unwrap_or(0), edge.twin

        c.add(SpawnText::new(point, format!("[{}] -> {} ", i, edge.next)));
    }

    renderer.set(
        "my_polyhedron_points",
        Vectors::new(hedron.all_verts()),
        Color::MIDNIGHT_BLUE,
        0.8,
        false,
    );
}

fn _create_polyhedron(renderer: &mut GeoRenderer<FaceMaterial>) {
    let mut hedron = Polyhedron::new();

    let p0: VertPtr = hedron.add_vert(vec3(0., 0., 0.));
    let p1: VertPtr = hedron.add_vert(vec3(1., 0., 0.));
    let p2: VertPtr = hedron.add_vert(vec3(1., 1., 0.));
    let p3: VertPtr = hedron.add_vert(vec3(0., 1., 0.));
    hedron.add_planar_edge(p0, p1);
    hedron.add_planar_edge(p1, p2);
    hedron.add_planar_edge(p2, p3);
    hedron.add_planar_edge(p3, p0);

    let p4: VertPtr = hedron.add_vert(vec3(3., 0., 0.));
    let p5: VertPtr = hedron.add_vert(vec3(2., 2., 0.));
    hedron.add_planar_edge(p1, p4);
    hedron.add_planar_edge(p4, p5);
    hedron.add_planar_edge(p5, p2);
    // hedron.add_planar_edge(p5, p2);

    let p6: VertPtr = hedron.add_vert(vec3(0., 4., 0.));
    hedron.add_planar_edge(p6, p5);
    hedron.add_planar_edge(p6, p2);
    hedron.add_planar_edge(p6, p3);

    let p7: VertPtr = hedron.add_vert(vec3(3., 4., 0.));
    hedron.add_planar_edge(p7, p4);
    hedron.add_planar_edge(p7, p5);
    hedron.add_planar_edge(p7, p6);

    hedron = hedron.mv(vec3(-2.0, -2.0, 0.0)).scale_u(4.0);

    // render all sorts of things about it
    // hedron.print_structure();
    // for (i, vert) in hedron.verts.iter_enum() {
    //     Self::spawn_billboard_text(&mut c, &ass, format!("v{}", i), vert.pos);
    // }

    // for (i, edge) in hedron.edges.iter_enum() {
    //     let (start, end) = hedron.edge_verts(i);

    //     let point = start.lerp(end, 0.33);

    //     //  edge.face.unwrap_or(0), edge.twin
    //     Self::spawn_billboard_text(&mut c, &ass, format!("[{}] -> {} ", i, edge.next), point);
    // }

    renderer.set(
        "my_polyhedron_points",
        Vectors::new(hedron.all_verts()),
        Color::DARK_GREEN,
        0.1,
        false,
    );
    renderer.set("my_polyhedron", hedron, Color::DARK_GREEN, 0.3, false);
}

#[derive(Component)]
pub struct GizmoTag;

pub fn spawn_gizmo_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<crate::prelude::BevyMesh>>,
    mut materials: ResMut<Assets<FaceMaterial>>,
) {
    let extend = 1.0;
    let width = 0.1;
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::default())),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_xyz(extend * 0.5, 0.0, 0.0)
                .with_scale(Vec3::new(extend, width, width)),
            ..default()
        },
        GizmoTag,
    ));
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::default())),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, extend * 0.5, 0.0)
                .with_scale(Vec3::new(width, extend, width)),
            ..default()
        },
        GizmoTag,
    ));
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::default())),
            material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, extend * 0.5)
                .with_scale(Vec3::new(width, width, extend)),
            ..default()
        },
        GizmoTag,
    ));
}
