use std::collections::HashMap;

use glam::{IVec2, ivec2};

use crate::{
    kernel::{fxx, Vec3},
    solid::{Mesh, Polyhedron, VertPtr}, planar::Polygon,
};

// #[derive(Eq, Hash, PartialEq, Copy, Clone)]
// pub struct IVec2 {
//     pub vert: VertPtr,
//     pub height: i32,
// }

// impl IVec2 {
//     pub fn new(vert: VertPtr, height: i32) -> Self {
//         Self {vert, height}
//     }
    
//     pub fn add() {
//         todo!()
//     }
// }

/// a terrain data structure constisting of irregular polygons
#[derive(Default)]
pub struct PolygonTerrain {
    topo: Polyhedron,                       // the topology of the terrain. Cells exist at the vertices. TODO rename Polyhedron to graph.
    cells: HashMap<IVec2, bool>, // the occupancy cells of the terrain. bool is temporary.
    delta_height: fxx,
}

impl PolygonTerrain {
    pub fn new(polyhedron: Polyhedron, delta_height: fxx) -> Self {
        Self {
            topo: polyhedron,
            delta_height,
            ..Default::default()
        }
    }

    /// Oskar Stalberg's method of generating smooth & funky quad grids
    /// TODO: smoothing procedure needs to be improved 
    pub fn new_loose_quad_field(
        radius: fxx,
        grid_div: usize,
        quad_div: usize,
        smoothings: usize,
        smooth_length: fxx,
        delta_height: fxx,
    ) -> Self {
        let mesh = Mesh::new_hexagrid(radius, grid_div);
        let mut hedron = Polyhedron::from_mesh(&mesh);
        hedron.make_random_quads();
        for _ in 0..quad_div {
            hedron.quad_divide();
        }
        for _ in 0..smoothings {
            hedron.quad_smooth_planar_partition(Vec3::Z, smooth_length);
        }
        Self::new(hedron, delta_height)
    }
}

impl PolygonTerrain {

    // create or update a cell
    pub fn set_cell(&mut self, pos: IVec2, occupancy: bool) {
        if self.cells.contains_key(&pos) {
            let cell = self.cells.get_mut(&pos).unwrap();
            *cell = occupancy;
        }
    }

    // remove a cell
    pub fn delete_cell(&mut self, pos: IVec2) {
        self.cells.remove(&pos);
    }

    pub fn cell_side_neighbors(&self, pos: IVec2) -> Vec<IVec2> {
        self.topo.get_vert_neighbors(pos.x as VertPtr)
            .into_iter()
            .map(|vp| IVec2::new(vp as i32, pos.y))
            .collect()
    }

    pub fn create_iso_polygons(&self, base_plane: bool) -> Vec<(IVec2, FaceDirection, Polygon)> {
        let polygons = Vec::new();

        if base_plane {
            for vert in self.topo.verts.iter() {
                let height = -0.5 * self.delta_height; 
                let pg = self.topo.dual_face(vert, height);
                polygons.add((pos, FaceDirection::Top, pg));
            }
        }

        for (pos, occ) in self.cells.iter() {
            let vert = pos.x as usize;
            assert!(self.topo.verts.get(vert).is_some());
            
            let lower = (pos.y as fxx) - 0.5 * self.delta_height;
            let upper = (pos.y as fxx) + 0.5 * self.delta_height;
            
            let top_nb = &(*pos + ivec2(0,  1));
            let bot_nb = &(*pos + ivec2(0, -1));
            let side_nbs = self.cell_side_neighbors(*pos);

            if self.cells.get(bot_nb).is_none() {
                let pg = self.topo.dual_face(vert, lower);
                polygons.add((pos, FaceDirection::Bottom, pg));
            }

            if self.cells.get(top_nb).is_none() {
                let pg = self.topo.dual_face(vert, upper);
                polygons.add((pos, FaceDirection::Top, pg));
            }

            for nb in side_nbs {
                if self.cells.get(&ivec2(nb, pos.y)).is_none() {
                    let edge = self.topo.get_edge_between(vert, nb);
                    let (a, b) = self.topo.dual_edge(edge, lower);
                    let (c, b) = self.topo.dual_edge(edge, upper);
                    let pg = Polygon::new(vec![a, b, d, c]);
                    polygons.add((pos, FaceDirection::Top, pg));
                }
            }
        }
        polygons
    }
}

pub enum FaceDirection {
    Top,
    Bottom,
    Side(VertPtr),
}