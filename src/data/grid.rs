use glam::IVec3;

/**
 A discrete, protected 3D data structure
 Protected in the sense that it always checks bounds, even in production.

 IDEAS: sparce matrix? chunked matrix? 
 */
pub struct Grid<T> {
    pub x_size: usize,
    pub y_size: usize,
    pub z_size: usize,
    pub items: Vec<T>,
}

// basic data methods
impl<T: Clone + Copy> Grid<T> {

    pub fn new(x: usize, y: usize, z: usize, default_value: T) -> Self {
        let items = vec![default_value; x * y * z];
        Self {x_size: x, y_size: y, z_size: z, items}
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.x_size * self.y_size * self.z_size
    }

    #[inline]
    pub fn size_vec(&self) -> IVec3 {
        IVec3::new(self.x_size as i32, self.y_size as i32, self.z_size as i32)
    }

    #[inline]
    pub fn set(&mut self, x: i32, y: i32, z: i32, tile: T) -> Option<usize> {
        let id = self.to_index(x, y, z)?;
        self.items[id] = tile;
        Some(id)
    }

    #[inline]
    pub fn set_at(&mut self, pos: IVec3, tile: T) -> Option<usize> {
        self.set(pos.x, pos.y, pos.z, tile)
    }

    #[inline]
    pub fn get(&self, x: i32, y: i32, z: i32) -> Option<T> {
        let id = self.to_index(x, y, z)?;
        Some(self.items[id])
    }

    #[inline]
    pub fn get_at(&self, pos: IVec3) -> Option<T> {
        self.get(pos.x, pos.y, pos.z)
    }

    pub fn is_on_grid(&self, x: i32, y: i32, z: i32) -> bool {
        x < 0 || x >= self.x_size as i32 || 
        y < 0 || y >= self.y_size as i32 ||
        z < 0 || z >= self.z_size as i32 
    }

    #[inline]
    pub fn to_index(&self, x: i32, y: i32, z: i32) -> Option<usize> {
        if self.is_on_grid(x, y, z) {
            None
        } else {
            Some((z as usize * self.x_size * self.y_size) + (y as usize * self.x_size) + x as usize)
        }
    }
}