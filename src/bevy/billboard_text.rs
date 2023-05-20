use bevy::{ecs::system::Command, prelude::*};

#[derive(Component)]
pub struct WorldPosition {
    pub pos: Vec3,
}

pub fn billboard_text_system(
    mut styles: Query<(&mut Style, &WorldPosition, &Node), With<Text>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    for (camera, tf) in cameras.iter() {
        for (mut style, wp, node) in styles.iter_mut() {
            let res = camera.world_to_viewport(tf, wp.pos);
            if res.is_none() {
                continue;
            }
            let vpc = res.unwrap();
            style.position = UiRect {
                bottom: Val::Px(vpc.y), // - node.size.y * 0.5
                left: Val::Px(vpc.x - node.size().x * 0.5),
                ..default()
            }
        }
    }
}

pub fn _create_text_system(mut _c: Commands, _a: Res<AssetServer>) {
    // spawn a billboard
    // Self::spawn_billboard_text(&mut c, &a, "(4.0, 0.0, 0.0)", Vec3::new(4.0, 0.0, 0.0));
    // Self::spawn_billboard_text(&mut c, &a, "(0.0, 4.0, 0.0)", Vec3::new(0.0, 4.0, 0.0));
    // Self::spawn_billboard_text(&mut c, &a, "(0.0, 0.0, 4.0)", Vec3::new(0.0, 0.0, 4.0));
}

#[derive(Component)]
pub struct BillboardText;

pub struct SpawnText {
    pos: Vec3,
    text: String,
}
impl SpawnText {
    pub fn new(pos: Vec3, text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            pos,
        }
    }
}
impl Command for SpawnText {
    fn write(self, world: &mut World) {
        let Self { text, pos } = self;
        let Some(asset_server) = world.get_resource::<AssetServer>() else {
            return;
        };
        world.spawn((
            TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                    font_size: 12.0,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect { ..default() },
                ..default()
            }),
            WorldPosition { pos },
            BillboardText,
        ));
    }
}

pub fn spawn_verts_as_text(c: &mut Commands, mesh: &crate::prelude::HedronMesh) {
    for (i, vert) in mesh.verts.iter().enumerate() {
        c.add(SpawnText::new(*vert, i.to_string()))
    }
}
pub fn spawn_naked_edges_as_text(c: &mut Commands, mesh: &crate::prelude::HedronMesh) {
    for edge in mesh.iter_naked_edges() {
        let (a, b) = (mesh.verts[edge.0], mesh.verts[edge.1]);
        let x = (a + b) / 2.0;
        c.add(SpawnText::new(x, format!("({} - {})", edge.0, edge.1)))
    }
}
