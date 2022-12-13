use glam::IVec2;

use crate::math::stack_sum;

/// A triangular grid.
/// Pattern utilized:
///  ```
///          column x
///       0  1  2  3  4
///      +--------------
///  r 0 |0  1  3  6  10
///  o 1 |   2  4  7  11
///  w 2 |      5  8  12
///    3 |         9  13
///  y 4 |            14
/// ```
pub struct GridTri<T> {
    pub total_size: usize, // total size
    pub edge_size: usize,
    pub items: Vec<T>,
}

impl<T: Clone + Copy + Default> GridTri<T> {
    pub fn new(edge_size: usize) -> Self {
        let total_size = stack_sum(edge_size);
        let items = vec![T::default(); total_size];
        Self {
            total_size,
            edge_size,
            items,
        }
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, cell: T) -> Option<usize> {
        let id = self.to_index(x, y)?;
        self.items[id] = cell;
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
    fn to_index(&self, x: usize, y: usize) -> Option<usize> {
        Some(stack_sum(x) + y)
    }

    // pub fn get_column(&self, x: usize) -> Vec<T> {
    //     let mut v = vec![T::default(); self.height];
    //     for y in 0..self.height {
    //         v[y] = self.get(x, y).unwrap();
    //     }
    //     v
    // }

    // pub fn get_row(&self, y: usize) -> Vec<T> {
    //     let mut v = vec![T::default(); self.width];
    //     for x in 0..self.width {
    //         v[x] = self.get(x, y).unwrap();
    //     }
    //     v
    // }

    pub fn iterate(&self) {
        // let size = 0;
        // for col in ((self.edge_size - 1)..-1).step_by(-1) {
        //     // reverse
        //     for row in 0..col {
        //         // ...
        //     }
        // }

        // for (let col = size - 1; col > -1; col -= 1) {
        //     for (let row = 0; row <= col; row++) {
        //         let idx = Util.iterateTriangle(col, row);
        //     }
        // }
    }
}

// /**
//  * ```
//  *         column
//  *      4  3  2  1  0
//  *   0 |>
//  * r 1 |>     >
//  * o 2 |>     >     >
//  * w 3 |>     >
//  *   4 |>
//  *

// * ```
// */
// pub fn iterateTriangle(column: usize, row: usize): usize {
//     return crate::math::stack_sum(column) + row;
// }

// pub fn getTriangleBase(triangle: MultiVector3, size: number) {
//     let base = MultiVector3.new(size);
//     let basecolumn = size - 1;
//     let i = 0;
//     for (let row = 0; row <= basecolumn; row++) {
//         let idx = Util.;
//         base.set(i, triangle.get(iterateTriangle(basecolumn, row)));
//         i++;
//     }
//     return base;
// }

// pub fn getTriangleLeft(triangle: MultiVector3, size: number) {
//     // prepare
//     let left = MultiVector3.new(size);

//     // the two edges of the triangle opposite to the base are the vertices we are interested in
//     let i = 0;
//     for (let col = size - 1; col > -1; col -= 1) {
//         left.set(i, triangle.get(iterateTriangle(basecolumn, row)));
//         i++;
//     }

//     return left;
// }

// pub fn getTriangleRight(triangle: MultiVector3, size: number) {
//     // prepare
//     let right = MultiVector3.new(size);

//     // the two edges of the triangle opposite to the base are the vertices we are interested in
//     let i = 0;
//     for (let col = size - 1; col > -1; col -= 1) {
//         right.set(i, triangle.get(iterateTriangle(basecolumn, row)));
//         i++;
//     }

//     return right;
// }
// }
