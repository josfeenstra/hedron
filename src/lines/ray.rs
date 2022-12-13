use crate::core::Plane;
use bevy::prelude::{Camera, GlobalTransform};
use glam::*;

pub struct Ray {
    pub origin: Vec3,
    pub normal: Vec3,
}

impl Ray {
    /// NOTE: this expects a normalized normal!!!
    pub fn new(origin: Vec3, normal: Vec3) -> Self {
        Self { origin, normal }
    }

    /// shorthand for getting a ray from a point to another point
    pub fn new_from_points(origin: Vec3, to: Vec3) -> Self {
        Self {
            origin,
            normal: (to - origin).normalize(),
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.normal * t
    }

    /// intersect with a plane
    /// returns the t parameter on this ray
    pub fn x_plane(&self, plane: &Plane) -> f32 {
        // let t = plane.
        let pn = plane.normal().truncate();
        return -(self.origin.dot(pn) + plane.d()) / self.normal.dot(pn);
    }
}

impl Ray {
    // taken from https://github.com/aevyrie/bevy_mod_raycast/blob/main/src/primitives.rs
    pub fn from_screen(
        cursor_pos_screen: Vec2,
        camera: &Camera,
        camera_transform: &GlobalTransform,
    ) -> Option<Self> {
        // adjust cursor position
        let (viewport_min, viewport_max) = camera.logical_viewport_rect()?;
        let screen_size = camera.logical_target_size()?;
        let adj_cursor_pos =
            cursor_pos_screen - Vec2::new(viewport_min.x, screen_size.y - viewport_max.y);

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
}
