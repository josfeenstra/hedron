use crate::{kernel::fxx, util};
use bevy_math::IVec2;

/**
A grid data structure
*/
pub struct Grid2<T> {
    pub width: usize,
    pub height: usize,
    pub items: Vec<T>,
}

pub type Matrix = Grid2<fxx>;

// for some reason, nalgebras print thing fails to print the full matrix correctly...
#[cfg(feature = "nalgebra")]
pub fn print_mat<T: std::fmt::Display>(mat: &nalgebra::DMatrix<T>) {
    println!("┌       ┐");
    for row in 0..mat.nrows() {
        print!("| ");
        for col in 0..mat.ncols() {
            print!(" {} ", mat[(row, col)]);
        }
        println!(" |");
    }
    println!("└       ┘");
}

// basic data methods
impl<T: Clone + Copy + Default> Grid2<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let items = vec![T::default(); width * height];
        Self {
            width,
            height,
            items,
        }
    }

    // ASSUMES a uniformly stacked Vec!!!
    pub fn from_stacked_columns(stacked: Vec<Vec<T>>) -> Grid2<T> {
        assert!(!stacked.is_empty());
        Self {
            width: stacked.len(),
            height: stacked[0].len(),
            items: stacked.into_iter().flatten().collect(),
        }
    }

    pub fn from_stacked_rows(stacked: &Vec<Vec<T>>) -> Grid2<T> {
        assert!(!stacked.is_empty());
        let width = stacked[0].len();
        let height = stacked.len();
        let mut grid = Grid2::new(width, height);

        #[allow(clippy::needless_range_loop)]
        for x in 0..width {
            for y in 0..height {
                grid.set(x, y, stacked[y][x]);
            }
        }

        grid
    }

    pub fn from_stacked_rows_flipped(stacked: &Vec<Vec<T>>) -> Grid2<T> {
        assert!(!stacked.is_empty());
        let width = stacked.len();
        let height = stacked[0].len();
        let mut grid = Grid2::new(width, height);

        #[allow(clippy::needless_range_loop)]
        for x in 0..width {
            for y in 0..height {
                grid.set(x, y, stacked[x][y]);
            }
        }

        grid
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
    pub fn get_unsafe(&self, id: usize) -> T {
        self.items[id]
    }

    #[inline]
    pub fn get_i32(&self, x: i32, y: i32) -> Option<T> {
        if x < 0 || y < 0 {
            return None;
        };
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
    pub fn to_index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some((y * self.width) + x)
        }
    }

    pub fn to_xy(&self, i: usize) -> (usize, usize) {
        let x = i % self.width;
        let y = i / self.width;
        (x, y)
    }

    pub fn get_column(&self, x: usize) -> Vec<T> {
        let mut v = vec![T::default(); self.height];
        for (y, item) in v.iter_mut().enumerate() {
            *item = self.get(x, y).unwrap();
        }
        v
    }

    pub fn get_row(&self, y: usize) -> Vec<T> {
        let mut v = vec![T::default(); self.width];
        for (x, item) in v.iter_mut().enumerate() {
            *item = self.get(x, y).unwrap();
        }
        v
    }

    pub fn iter_wh(&self) -> impl Iterator<Item = (usize, usize)> {
        util::iter_xy((self.width, self.height))
    }

    // fn to_coord(&self, i: usize) -> (i32, i32) {
    //     ((i % self.width) as i32, (i / self.width) as i32)
    // }
}

#[cfg(test)]
mod tests {
    use crate::kernel::fxx;

    use super::Grid2;

    #[test]
    fn test_grid() {
        let grid = Grid2::<fxx>::new(4, 5);

        assert_eq!(grid.to_xy(0), (0, 0));
        assert_eq!(grid.to_xy(1), (1, 0));
        assert_eq!(grid.to_xy(4), (0, 1));
        assert_eq!(grid.to_xy(5), (1, 1));

        assert_eq!(grid.to_index(0, 0), Some(0));
        assert_eq!(grid.to_index(1, 0), Some(1));
        assert_eq!(grid.to_index(0, 1), Some(4));
        assert_eq!(grid.to_index(1, 1), Some(5));

        // for (x, y) in grid.iter_xy() {
        //     grid.set(x, y, grid.get(x, y).unwrap() + 2.0);
        // }
    }
}
