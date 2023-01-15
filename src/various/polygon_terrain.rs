use std::collections::HashMap;

use crate::{
    kernel::{fxx, Vec3, INFINITY},
    solid::{Mesh, Polyhedron, VertPtr}, planar::Polygon, lines::Ray,
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

#[derive(Clone)]
pub enum FaceDirection {
    Top,
    Bottom,
    Side(VertPtr),
}

/// a terrain data structure constisting of irregular polygons
#[derive(Default, Clone)]
pub struct PolygonTerrain {
    pub topo: Polyhedron,                       // the topology of the terrain. Cells exist at the vertices. TODO rename Polyhedron to graph.
    pub cells: HashMap<CellPos, bool>, // the occupancy cells of the terrain. bool is temporary.
    pub iso: Vec<(CellPos, FaceDirection, Polygon)>,
    pub looks: Vec<bool>,

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
    pub fn set(&mut self, pos: CellPos, occupancy: bool) -> bool {
        if self.cells.contains_key(&pos) {
            let cell = self.cells.get_mut(&pos).unwrap();
            *cell = occupancy;
            false
        } else {
            self.cells.insert(pos, occupancy);
            true
        }
    }

    // remove a cell
    pub fn delete(&mut self, pos: CellPos) {
        self.cells.remove(&pos);
    }

    /// run this after updating cells
    pub fn update_iso(&mut self, base_plane: bool) {
        self.iso = self.create_iso_polygons(base_plane);
    }

    /// test intersection using the iso faces.
    /// returns the edge of the dual of graph of this face, and the face itself
    pub fn intersect_iso(&self, ray: &Ray) -> Option<(CellPos, CellPos, Polygon)>{
        let mut best_dist = INFINITY;
        let mut best_hit: Option<usize> = None;
        for (i, (vp, dir, pg)) in self.iso.iter().enumerate() {
            if pg.intersect_ray(ray) {
                let dist = pg.center().distance(ray.origin);
                let dot = ray.normal.dot(pg.average_normal() * -1.0);
                if dot < 0.0 {
                    continue;
                }

                if dist < best_dist {
                    best_dist = dist;
                    best_hit = Some(i);
                }
            }
        }
        match best_hit {
            Some(i) => {
                let (vp, dir, pg) = &self.iso[i];
                let other = match dir {
                    FaceDirection::Top => CellPos::new(vp.vert, vp.height + 1),
                    FaceDirection::Bottom => CellPos::new(vp.vert, vp.height - 1),
                    FaceDirection::Side(side) => CellPos::new(*side, vp.height),
                };
                Some((*vp, other, pg.clone()))
            },
            None => None,
        }
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
                let pg = self.topo.dual_face(vp, upper).flip();
                if pg.verts.len() < 3 {
                    continue;
                }
                let pos = CellPos::new(vp, y);
                polygons.push((pos, FaceDirection::Top, pg));
            }
        }

        for (pos, occ) in self.cells.iter() {
            assert!(self.topo.verts.get(pos.vert).is_some());
            
            let lower = ((pos.height as fxx) - 0.5) * self.delta_height;
            let upper = ((pos.height as fxx) + 0.5) * self.delta_height;
            
            let top_nb = &pos.add_height(1);
            let bot_nb = &pos.add_height(-1);
            let side_nbs = self.cell_side_neighbors(*pos);

            if self.cells.get(bot_nb).is_none() {
                let pg = self.topo.dual_face(pos.vert, lower);
                polygons.push((*pos, FaceDirection::Bottom, pg));
            }

            if self.cells.get(top_nb).is_none() {
                let pg = self.topo.dual_face(pos.vert, upper).flip();
                polygons.push((*pos, FaceDirection::Top, pg));
            }

            for nb in side_nbs {
                if self.cells.get(&nb).is_none() {
                    let edge = self.topo.get_edge_between(pos.vert, nb.vert).expect("should exist");
                    
                    let (Some((a, b)), Some((c, d))) = (
                        self.topo.dual_edge(edge, lower), 
                        self.topo.dual_edge(edge, upper)
                    ) else {
                        continue;
                    };
                    
                    let pg = Polygon::new(vec![c, d, b, a]);
                    polygons.push((*pos, FaceDirection::Side(nb.vert), pg));
                }
            }
        }
        polygons
    }
}

impl Into<Mesh> for PolygonTerrain {
    fn into(self) -> Mesh {
        Mesh::from_join(self.iso.iter().map(|(_, _, pg)| pg.triangulate_naive()).collect())
    }
}

#[cfg(feature = "bevy")]
impl Into<bevy::prelude::Mesh> for PolygonTerrain {
    fn into(self) -> bevy::prelude::Mesh {
        let mesh: Mesh = self.into();
        mesh.into()
    }
}
