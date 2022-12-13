use glam::IVec2;

/**
 A grid data structure
 */
pub struct Grid2<T> {
    pub width: usize,
    pub height: usize,
    pub items: Vec<T>,
}

pub type Matrix = Grid2<f32>;

// basic data methods
impl<T: Clone + Copy + Default> Grid2<T> {

    pub fn new(width: usize, height: usize) -> Self {
        let items = vec![T::default(); width * height];
        Self {width, height, items}
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.width * self.height
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, tile: T) -> Option<usize> {
        let id = self.to_index(x, y)?;
        self.items[id] = tile;
        Some(id)
    }

    #[inline]
    pub fn set_at(&mut self, pos: IVec2, tile: T) -> Option<usize> {
        self.set(pos.x as usize, pos.y as usize, tile)
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        let id = self.to_index(x, y)?;
        Some(self.items[id])
    }

    #[inline]
    pub fn get_i32(&self, x: i32, y: i32) -> Option<T> {
        if x < 0 || y < 0 { return None };
        let id = self.to_index(x as usize, y as usize)?;
        Some(self.items[id])
    }

    #[inline]
    pub fn get_at(&self, pos: IVec2) -> Option<T> {
        self.get(pos.x as usize, pos.y as usize)
    }

    // pub fn get_at(&self, point: Point) -> Option<Tile> {
    //     self.get_tile(point.x, point.y)
    // }

    #[inline]
    fn to_index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some((y as usize * self.width) + x as usize)
        }
    }

    pub fn get_column(&self, x: usize) -> Vec<T> {
        let mut v = vec![T::default(); self.height];
        for y in 0..self.height {
            v[y] = self.get(x, y).unwrap();
        }   
        v
    }

    pub fn get_row(&self, y: usize) -> Vec<T> {
        let mut v = vec![T::default(); self.width];
        for x in 0..self.width {
            v[x] = self.get(x, y).unwrap();
        }
        v      
    }

    // fn to_coord(&self, i: usize) -> (i32, i32) {
    //     ((i % self.width) as i32, (i / self.width) as i32)
    // }
}