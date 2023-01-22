use std::collections::HashMap;

use crate::{
    kernel::{fxx, Vec3, INFINITY},
    solid::{Mesh, Polyhedron, VertPtr, Octoid}, planar::Polygon, lines::Ray,
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

    /// order the positions first by vert, then by height
    pub fn is_higher_order_than(&self, rhs: &Self) -> bool {
        if self.vert > rhs.vert {
            true
        } else if self.vert < rhs.vert {
            false
        } else {
            if self.height > rhs.height {
                true
            } else  {
                false
            }   
        }
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

    pub fn cell_side_neighbors(&self, pos: CellPos) -> Vec<CellPos> {
        self.topo.get_vert_neighbors(pos.vert)
            .into_iter()
            .map(|vp| CellPos::new(vp, pos.height))
            .collect()
    }

    /// render the terrain using marching cubes
    /// TODO lets first do it on dry land, and create one joined mesh from everything.
    pub fn render_marching_cubes(&self) -> Mesh {
        let cuboids = self.create_cuboids();
        let source_meshes = [Mesh::new_icosahedron(1.0)];
        let mut target_meshes = Vec::new();
        
        for cuboid in cuboids {
            let verts = [cuboid[0].0, cuboid[1].0, cuboid[2].0, cuboid[3].0, cuboid[4].0, cuboid[5].0, cuboid[6].0, cuboid[7].0];
            let key = [cuboid[0].1, cuboid[1].1, cuboid[2].1, cuboid[3].1, cuboid[4].1, cuboid[5].1, cuboid[6].1, cuboid[7].1];
            // TODO: look up using the key
            let mut tile = source_meshes[0].clone();
            
            let oct = Octoid::new(verts);
            for vert in tile.verts.iter_mut() {
                *vert = oct.tri_lerp(*vert);
            }
            
            target_meshes.push(tile);
        }

        Mesh::from_join(target_meshes)
    }

    /// create all data needed to render the polygon using marching cubes
    /// TODO 1 : build a unit test for this
    /// TODO 2 : integrate with the marching cubes meshes you have! 
    /// TODO 3 : trilinear interpolation!
    pub fn create_cuboids(&self) -> Vec<[(Vec3, bool); 8]> {
        let mut cubes = Vec::new();
        
        for (base_pos, occupancy) in self.cells.iter() {
            let quads = self.topo.get_vert_faces(base_pos.vert);

            // twice per quad
            for quad in quads {
                let edges = self.topo.get_loop(self.topo.face(quad).edge);
                let vps = edges.iter().map(|ep| self.topo.edge(*ep).from).collect::<Vec<_>>();
                let points = vps.iter().map(|vp| self.topo.vert(*vp).pos).collect::<Vec<_>>();

                for i in 0..2 {                            
                    let lower_height = base_pos.height + i - 1 as i32;
                    let upper_height = base_pos.height + i as i32;

                    let lower = lower_height as fxx * self.delta_height;
                    let upper = upper_height as fxx * self.delta_height;

                    let [a, b, c, d] = points[..] else {
                        println!("WARN: not a quad!");
                        continue;
                    };
                    let [va, vb, vc, vd] = vps[..] else {
                        println!("WARN: not a quad!");
                        continue;
                    };
                    
                    let cell_positions = [
                        CellPos::new(va, lower_height),
                        CellPos::new(vb, lower_height),
                        CellPos::new(vd, lower_height),
                        CellPos::new(vc, lower_height),
                        CellPos::new(va, upper_height),
                        CellPos::new(vb, upper_height),
                        CellPos::new(vd, upper_height),
                        CellPos::new(vc, upper_height)
                    ];

                    fn get(t: &PolygonTerrain, pos: &CellPos) -> bool {
                        match t.cells.get(pos) {
                            Some(b) => *b,
                            None => false,
                        }
                    }

                    // prevent duplicates: 
                    // - of all cells occupied by something (so the ones we are iterating through using the base_pos)
                    // - only continue if base_pos is the highest order one.
                    // - this ordering is arbitrary, just some metric to make 1 stand out on top consistently
                    if cell_positions
                        .iter()
                        .filter(|pos| get(&self, pos)) 
                        .any(|pos| pos.is_higher_order_than(base_pos)) { 
                        continue;
                    }

                    let cube = [
                        (a + lower, get(&self, &cell_positions[0])),
                        (b + lower, get(&self, &cell_positions[1])),
                        (d + lower, get(&self, &cell_positions[2])),
                        (c + lower, get(&self, &cell_positions[3])),
                        (a + upper, get(&self, &cell_positions[4])),
                        (b + upper, get(&self, &cell_positions[5])),
                        (d + upper, get(&self, &cell_positions[6])),
                        (c + upper, get(&self, &cell_positions[7]))
                    ];

                    cubes.push(cube);
                }
            }
        }

        cubes
    }

    
}

// methods relating to the isosurface
impl PolygonTerrain {

    /// render the current isosurface as one mesh
    pub fn render_iso(&self) -> Mesh {
        Mesh::from_join(self.iso.iter().map(|(_, _, pg)| pg.triangulate_naive()).collect())
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

    /// create an isosurface of polygons, with metadata, so we know which polygon beongs to which cell
    /// TODO in the future, we might want to leverage the fact that this isosurface can be implicitly gathered from the cells + topology
    /// no clue what this will look like
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
        self.render_iso()
    }
}

#[cfg(feature = "bevy")]
impl Into<bevy::prelude::Mesh> for PolygonTerrain {
    fn into(self) -> bevy::prelude::Mesh {
        let mesh: Mesh = self.into();
        mesh.into()
    }
}
