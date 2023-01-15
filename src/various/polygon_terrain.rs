use std::collections::HashMap;

use crate::{
    kernel::{fxx, Vec3},
    solid::{Mesh, Polyhedron, VertPtr}, planar::Polygon,
};

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub struct CellPos {
    pub vert: VertPtr, // pointer to the topo
    pub height: i32, // height on that topo
}

impl CellPos {
    pub fn new(vert: VertPtr, height: i32) -> Self {
        Self {vert, height}
    }
    
    /// raise or lower this position
    pub fn add_height(&self, height: i32) -> Self {
        Self::new(self.vert, self.height + height)
    }
}

/// a terrain data structure constisting of irregular polygons
#[derive(Default)]
pub struct PolygonTerrain {
    pub topo: Polyhedron,                       // the topology of the terrain. Cells exist at the vertices. TODO rename Polyhedron to graph.
    pub cells: HashMap<CellPos, bool>, // the occupancy cells of the terrain. bool is temporary.
    pub delta_height: fxx,
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
        hedron.cap();
        Self::new(hedron, delta_height)
    }
}

impl PolygonTerrain {

    // create or update a cell
    pub fn set_cell(&mut self, pos: CellPos, occupancy: bool) {
        if self.cells.contains_key(&pos) {
            let cell = self.cells.get_mut(&pos).unwrap();
            *cell = occupancy;
        }
    }

    // remove a cell
    pub fn delete_cell(&mut self, pos: CellPos) {
        self.cells.remove(&pos);
    }

    pub fn cell_side_neighbors(&self, pos: CellPos) -> Vec<CellPos> {
        self.topo.get_vert_neighbors(pos.vert)
            .into_iter()
            .map(|vp| CellPos::new(vp, pos.height))
            .collect()
    }

    pub fn create_iso_polygons(&self, base_plane: bool) -> Vec<(CellPos, FaceDirection, Polygon)> {
        let mut polygons = Vec::new();

        if base_plane {
            for (vp, vert) in self.topo.verts.iter_enum() {
                let y = -1;
                let upper = (y as fxx) + 0.5 * self.delta_height;
                let pg = self.topo.dual_face(vp, upper);
                let pos = CellPos::new(vp, y);
                polygons.push((pos, FaceDirection::Top, pg));
            }
        }

        for (pos, occ) in self.cells.iter() {
            assert!(self.topo.verts.get(pos.vert).is_some());
            
            let lower = (pos.height as fxx) - 0.5 * self.delta_height;
            let upper = (pos.height as fxx) + 0.5 * self.delta_height;
            
            let top_nb = &pos.add_height(1);
            let bot_nb = &pos.add_height(-1);
            let side_nbs = self.cell_side_neighbors(*pos);

            if self.cells.get(bot_nb).is_none() {
                let pg = self.topo.dual_face(pos.vert, lower);
                polygons.push((*pos, FaceDirection::Bottom, pg));
            }

            if self.cells.get(top_nb).is_none() {
                let pg = self.topo.dual_face(pos.vert, upper);
                polygons.push((*pos, FaceDirection::Top, pg));
            }

            for nb in side_nbs {
                if self.cells.get(&nb).is_none() {
                    let edge = self.topo.get_edge_between(pos.vert, nb.vert).expect("should exist");
                    let (a, b) = self.topo.dual_edge(edge, lower).expect("edge should be between two faces");
                    let (c, d) = self.topo.dual_edge(edge, upper).expect("edge should be between two faces");
                    let pg = Polygon::new(vec![a, b, d, c]);
                    polygons.push((*pos, FaceDirection::Side(nb.vert), pg));
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