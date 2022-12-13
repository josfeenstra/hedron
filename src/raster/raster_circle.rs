use glam::{IVec2, Vec2};
use std::f32::consts;

/// represents a circle on the grid
pub struct ICircle {
    pub center: IVec2,
    pub radius: f32,
}

impl ICircle {

    pub fn new(center: IVec2, radius: f32) -> ICircle {
        ICircle {center, radius}
    }

    pub fn to_grid_fill(&self) -> Vec<IVec2> {
        let mut fill = Vec::new();

        let radius = self.radius;
        let center = &self.center;

        let size_y = ( radius * (0.5_f32).sqrt() ).floor() as i32;

        // TODO finish this!
        for dy in 0..=size_y {
            let fdy = dy as f32;
            let dx  = ((radius*radius - fdy * fdy) as f32).sqrt();
            let left  = (center.x as f32 - dx).ceil() as i32;
            let right = (center.x as f32 + dx).floor() as i32;

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

        let size_y = ( radius * (0.5_f32).sqrt() ).floor() as i32;

        for dy in 0..=size_y {
            let fdy = dy as f32;
            let dx  = ((radius*radius - fdy * fdy) as f32).sqrt().floor() as i32;
 
            if dy != 0 { // eliminate ortagonal duplicates
                border.push(IVec2::new(center.x - dx, center.y - dy));
                border.push(IVec2::new(center.x + dx, center.y + dy));
            }
            border.push(IVec2::new(center.x - dx, center.y + dy));
            border.push(IVec2::new(center.x + dx, center.y - dy));

            if dx == dy { continue } // eliminate diagonal duplicates

            if dy != 0 { // eliminate ortagonal duplicates
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
    pub fn to_grid_arc(&self, from: f32, to: f32) -> Vec<IVec2> {
        let circle = self.to_grid_edge();
        let arc = circle.into_iter().filter(|p| {
            let angle = (*p - self.center).as_vec2().angle_between(Vec2::X);
            from < angle && angle <= to || 
            from < angle+consts::TAU && angle+consts::TAU <= to ||
            from < angle-consts::TAU && angle-consts::TAU <= to
        }).collect();
        arc
    } 
}
