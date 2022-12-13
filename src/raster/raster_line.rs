use glam::IVec2;

pub struct ILine {
    pub from: IVec2,
    pub to: IVec2
}

impl ILine {

    pub fn new(from: IVec2, to: IVec2) -> Self {
        Self {from, to}
    }

    /// turn a line into a bunch of points on a grid
    /// https://www.redblobgames.com/grids/line-drawing.html was very helpful!
    pub fn to_grid(&self) -> Vec<IVec2> {
        let mut cover: Vec<IVec2> = Vec::new();
        
        let delta = self.to - self.from;
        let x_step = if delta.x > 0 { 1 } else { -1 };
        let y_step = if delta.y > 0 { 1 } else { -1 };
        let x_count = delta.x.abs();
        let y_count = delta.y.abs();

        let mut cursor = self.from.clone();
        cover.push(cursor.clone());

        let mut xi = 0; 
        let mut yi = 0;

        while xi < x_count || yi < y_count {
            let compare = (1 + 2*xi) * y_count - (1 + 2*yi) * x_count;
            if compare >= 0 {
                // next step is vertical
                yi += 1;
                cursor.y += y_step;
            }
            if compare <= 0 {
                // next step is horizontal
                xi += 1;
                cursor.x += x_step;
            }
            cover.push(cursor.clone());
        };
        // util::print(&format!("length: {}", cover.len()));
        cover
    }


}