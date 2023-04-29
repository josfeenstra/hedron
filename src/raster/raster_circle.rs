use bevy_math::IVec2;

use crate::kernel::{fxx, ivec2_to_vec2, Vec2, TAU};

/// represents a circle on the grid
pub struct ICircle {
    pub center: IVec2,
    pub radius: fxx,
}

impl ICircle {
    pub fn new(center: IVec2, radius: fxx) -> ICircle {
        ICircle { center, radius }
    }

    pub fn to_grid_fill(&self) -> Vec<IVec2> {
        let mut fill = Vec::new();

        let radius = self.radius;
        let center = &self.center;

        let size_y = (radius * (0.5 as fxx).sqrt()).floor() as i32;

        // TODO finish this!
        for dy in 0..=size_y {
            let fdy = dy as fxx;
            let dx = ((radius * radius - fdy * fdy) as fxx).sqrt();
            let left = (center.x as fxx - dx).ceil() as i32;
            let right = (center.x as fxx + dx).floor() as i32;

            for x in left..=right {
                fill.push(IVec2::new(x, center.y + dy));
            }
        }

        fill
    }

    pub fn to_grid_edge(&self) -> Vec<IVec2> {
        let mut border = Vec::new();

        let radius = self.radius;
        let center = &self.center;

        let size_y = (radius * (0.5 as fxx).sqrt()).floor() as i32;

        for dy in 0..=size_y {
            let fdy = dy as fxx;
            let dx = ((radius * radius - fdy * fdy) as fxx).sqrt().floor() as i32;

            if dy != 0 {
                // eliminate ortagonal duplicates
                border.push(IVec2::new(center.x - dx, center.y - dy));
                border.push(IVec2::new(center.x + dx, center.y + dy));
            }
            border.push(IVec2::new(center.x - dx, center.y + dy));
            border.push(IVec2::new(center.x + dx, center.y - dy));

            if dx == dy {
                continue;
            } // eliminate diagonal duplicates

            if dy != 0 {
                // eliminate ortagonal duplicates
                border.push(IVec2::new(center.x - dy, center.y + dx));
                border.push(IVec2::new(center.x + dy, center.y - dx));
            }
            border.push(IVec2::new(center.x - dy, center.y - dx));
            border.push(IVec2::new(center.x + dy, center.y + dx));
        }

        border
    }

    /// from: angle in radians,
    /// to: angle in radians,
    /// NOTE: this is not the cleanest approach
    /// - we first calculate a full circle
    /// - we do stupid things with the angles, not foolproof
    pub fn to_grid_arc(&self, from: fxx, to: fxx) -> Vec<IVec2> {
        let circle = self.to_grid_edge();
        circle
            .into_iter()
            .filter(|p| {
                let angle = ivec2_to_vec2(*p - self.center).angle_between(Vec2::X);
                from < angle && angle <= to
                    || from < angle + TAU && angle + TAU <= to
                    || from < angle - TAU && angle - TAU <= to
            })
            .collect()
    }
}
