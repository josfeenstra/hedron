use crate::kernel::{fxx, Mat4, Vec2, Vec3};
use crate::lines::Ray;
use bevy::prelude::{Camera, GlobalTransform};

// taken from https://github.com/aevyrie/bevy_mod_raycast/blob/main/src/primitives.rs
pub fn ray_from_screen(
    cursor_pos_screen: Vec2,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Ray> {
    // adjust cursor position
    let (viewport_min, viewport_max) = camera.logical_viewport_rect()?;
    let screen_size = camera.logical_target_size()?;
    let adj_cursor_pos = cursor_pos_screen
        - Vec2::new(
            viewport_min.x as fxx,
            (screen_size.y - viewport_max.y) as fxx,
        );

    // get a whole bunch of camera properties
    let view = camera_transform.compute_matrix();
    let projection = camera.projection_matrix();
    let ndc_to_world: Mat4 = view * projection.inverse();

    // these could be directly extracted
    let far_ndc = projection.project_point3(Vec3::NEG_Z).z;
    let near_ndc = projection.project_point3(Vec3::Z).z;

    let viewport_size = viewport_max - viewport_min;
    let cursor_ndc = (adj_cursor_pos / viewport_size) * 2.0 - Vec2::ONE;

    let near = ndc_to_world.project_point3(cursor_ndc.extend(near_ndc));
    let far = ndc_to_world.project_point3(cursor_ndc.extend(far_ndc));
    let ray_direction = far - near;

    Some(Ray::new(near, ray_direction.normalize()))
}
