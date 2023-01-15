use std::io::Write;
use std::iter::Rev;
use std::ops::Range;

use crate::kernel::{fxx, vec2, vec3, Vec2, Vec3, kernel};

use crate::{core::PointBased, data::Grid2};

#[derive(Default, Debug)]
pub struct Mesh {
    pub verts: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
    pub tri: Vec<usize>,
}

impl Mesh {
    pub fn new(verts: Vec<Vec3>, tri: Vec<usize>, uvs: Vec<Vec2>) -> Self {
        Self { verts, tri, uvs }
    }

    pub fn new_weave(verts: Grid2<(Vec3, Vec2)>) -> Self {
        // TODO
        todo!();
    }

    // Get a grid mesh from weaving vertices of a grid.
    // The grid can deal with holes, use None vertices to indicate holes
    // We do this by finding 'valid cells' of 4 vertices, and lacing them like this:
    // Corners of 3 vertices are also created.
    // ```
    //  (d)----(c) .. ( )
    //   |  \   |      .
    //   |   \  |      .
    //  (b)----(a) .. ( )
    //   .      .      .
    //   .      .      .
    //  ( ) .. ( ) .. ( )
    // ```
    pub fn new_holed_weave(verts: Grid2<Option<(Vec3, Vec2)>>) -> Self {
        let mut mesh = Self::default();

        // add all verts, and dummy zero vectors at zero spots
        for (i, res) in verts.items.iter().enumerate() {
            let (x, y) = verts.to_xy(i);
            match res {
                Some((pos, uv)) => {
                    mesh.verts.push(pos.clone());
                    mesh.uvs.push(uv.clone());
                }
                None => {
                    mesh.verts.push(vec3(0., 0., 0.));
                    mesh.uvs.push(vec2(0., 0.));
                }
            }
        }

        // add triangles by lacing patches
        for (i, res) in verts.items.iter().enumerate() {
            let (x, y) = verts.to_xy(i);

            if x == 0 || y == 0 {
                continue;
            }

            let ia = i;
            let ib = verts.to_index(x - 1, y).unwrap();
            let ic = verts.to_index(x, y - 1).unwrap();
            let id = verts.to_index(x - 1, y - 1).unwrap();

            let a = verts.get_unsafe(ia);
            let b = verts.get_unsafe(ib);
            let c = verts.get_unsafe(ic);
            let d = verts.get_unsafe(id);

            // do some bitmask checking to get corner triangles
            // there is no elegant way to do this,
            // since we have to get the triangle counter-clockwise direction right
            if a.is_some() && b.is_some() && c.is_some() && d.is_some() {
                mesh.tri.append(&mut vec![ia, id, ib]);
                mesh.tri.append(&mut vec![ia, ic, id]);
            } else if a.is_some() && b.is_some() && c.is_some() {
                mesh.tri.append(&mut vec![ia, ic, ib]);
            } else if a.is_some() && b.is_some() && d.is_some() {
                mesh.tri.append(&mut vec![ia, id, ib]);
            } else if a.is_some() && c.is_some() && d.is_some() {
                mesh.tri.append(&mut vec![ia, ic, id]);
            } else if b.is_some() && c.is_some() && d.is_some() {
                mesh.tri.append(&mut vec![ib, ic, id]);
            }
        }

        // TODO clean away all unused vertices
        // return mesh.to_clean();
        mesh
    }

    //
    pub fn new_diamonds(points: Vec<Vec3>, size: fxx) -> Self {
        let mut meshes = Vec::new();
        for point in points {
            meshes.push(Self::new_diamond(point, size))
        }
        Self::from_join(meshes)
    }

    pub fn new_diamond(center: Vec3, size: fxx) -> Self {
        let mut mesh = Mesh::default();

        mesh.verts
            .push(Vec3::new(center.x + size, center.y, center.z));
        mesh.verts
            .push(Vec3::new(center.x - size, center.y, center.z));
        mesh.verts
            .push(Vec3::new(center.x, center.y + size, center.z));
        mesh.verts
            .push(Vec3::new(center.x, center.y - size, center.z));
        mesh.verts
            .push(Vec3::new(center.x, center.y, center.z + size));
        mesh.verts
            .push(Vec3::new(center.x, center.y, center.z - size));

        mesh.tri.append(&mut vec![
            4, 0, 2, 4, 2, 1, 4, 1, 3, 4, 3, 0, 5, 2, 0, 5, 1, 2, 5, 3, 1, 5, 0, 3,
        ]);

        mesh
    }

    // simple join, not taking common verts into account
    pub fn from_join(meshes: Vec<Mesh>) -> Mesh {
        let mut mesh = Mesh::default();

        let mut vertcount = 0;
        for mut other in meshes {
            let length = other.verts.len();
            mesh.verts.append(&mut other.verts);
            mesh.tri
                .append(&mut other.tri.iter().map(|t| t + vertcount).collect());
            vertcount += length;
        }

        mesh
    }

    // create a mesh as a hexagonal 
    pub fn new_hexagrid(radius: fxx, divisions: usize) -> Self {
        let mut mesh = Self::default();        

        // get some ranges / iterations right
        let min_count = 2 + divisions;
        let max_count = min_count + 2 + divisions;

        let upper: Range<usize> = min_count..max_count;
        let lower: Rev<Range<usize>> = (min_count..max_count-1).rev();
        let range: Vec<usize> = upper.clone().chain(lower.clone()).collect();
        
        // get some counters right for spawning the right grid of points
        let y_count = 3 + 2 * divisions; 
        let y_offset = 1 + divisions;
        
        let dx = radius / (divisions as fxx + 1.0) * 2.0;
        let dy = dx * kernel::SQRT_OF_3;

        // verts
        for (i, steps) in range.iter().enumerate() {
            let x_start = (steps - 1) as fxx * dx;
            let y = (i as fxx - y_offset as fxx) * dy;
            for j in 0..*steps {
                let x = -x_start + (j as fxx * dx * 2.0);
                mesh.verts.push(Vec3::new(x, y, 0.0));
                // println!("({x}, {y})")
            }
        }

        // triangles
        let last = mesh.verts.len() - 1;
        let mut i = 0;
        let upper_steps: Vec<_> = upper.collect();
        for steps in min_count..max_count-1 {
            for step in 0..steps {

                let a = i;
                let b = i + 1;
                let c = i + steps;
                let d = i + steps + 1;
                
                // radial mirror on the opposite side
                let aa = last - a;
                let bb = last - b;
                let cc = last - c;
                let dd = last - d;
       
                // only one triangle at te last segment of the strip
                if step > (steps-2) {
                    mesh.tri.append(&mut vec![a, d, c]);
                    mesh.tri.append(&mut vec![aa, dd, cc]);
                } else {
                    mesh.tri.append(&mut vec![a, b, d]);
                    mesh.tri.append(&mut vec![aa, bb, dd]);
                    mesh.tri.append(&mut vec![a, d, c]);
                    mesh.tri.append(&mut vec![aa, dd, cc]);
                }
                
                i += 1;
            } 
        }

        mesh
    }
}

impl Mesh {
    
    pub fn get_triangles(&self) -> Vec<(usize, usize, usize)> {
        let mut data = Vec::new();
        assert!(self.tri.len() % 3 == 0);
        for i in (0..self.tri.len()).step_by(3) {
            data.push((self.tri[i], self.tri[i + 1], self.tri[i + 2]))
        }
        data
    }

    pub fn get_edges(&self) -> Vec<(usize, usize)> {
        let tri = self.get_triangles();
        let mut edges = Vec::new();
        for (a, b, c) in tri {
            edges.push((a, b));
            edges.push((b, c));
            edges.push((c, a));
        }
        edges
    }

    pub fn to_clean(&self) -> Mesh {
        // TODO: identify all vertices which are not references by any triangle,
        // exclude them
        // then update the triangle vertex pointers.
        todo!();
    }

    pub fn write_obj(&self, path: &str) -> Result<(), std::io::Error> {
        let obj = self.gen_obj_buffer("obj generated by Hedron", None, None)?;
        let mut obj_file = std::fs::File::create(&path)?;
        obj_file.write_all(&obj)?;
        Ok(())
    }

    pub fn write_obj_mtl(
        &self,
        path: &str,
        name_obj: &str,
        name_mtl: &str,
        name_texture: &str,
    ) -> Result<(), std::io::Error> {
        let mat_name = "Material";

        // both the texture and mtl should be in the same folder
        let mtl = Mesh::gen_mtl_buffer("mtl generated by Hedron", mat_name, Some(name_texture))?;
        let obj = self.gen_obj_buffer("obj generated by Hedron", Some(mat_name), Some(name_mtl))?;

        let texture_path = path.to_owned() + name_texture;
        let mtl_path = path.to_owned() + name_mtl;
        let obj_path = path.to_owned() + name_obj;

        let mut obj_file = std::fs::File::create(obj_path)?;
        obj_file.write_all(&obj)?;
        let mut mtl_path = std::fs::File::create(mtl_path)?;
        mtl_path.write_all(&mtl)?;

        Ok(())
    }

    pub fn gen_mtl_buffer(
        header: &str,
        mat_name: &str,
        texture_path: Option<&str>,
    ) -> Result<Vec<u8>, std::io::Error> {
        let mut mtl = Vec::new();
        writeln!(&mut mtl, "# {}", header)?;
        writeln!(&mut mtl, "newmtl {}", mat_name)?;
        writeln!(&mut mtl, "Ns 250.000000")?;
        writeln!(&mut mtl, "Ka 1.000000 1.000000 1.000000")?;
        writeln!(&mut mtl, "Kd 0.000000 0.000000 0.000000")?;
        writeln!(&mut mtl, "Ks 0.000000 0.000000 0.000000")?;
        writeln!(&mut mtl, "Ke 0.000000 0.000000 0.000000")?;
        writeln!(&mut mtl, "Ni 1.450000")?;
        writeln!(&mut mtl, "d 1.000000")?;
        writeln!(&mut mtl, "illum 2")?;
        if let Some(path) = texture_path {
            writeln!(&mut mtl, "{}", format!("map_Ka {}", texture_path.unwrap()))?;
            writeln!(&mut mtl, "{}", format!("map_Kd {}", texture_path.unwrap()))?;
            writeln!(&mut mtl, "{}", format!("map_Ks {}", texture_path.unwrap()))?;
        }
        Ok(mtl)
    }

    pub fn gen_obj_buffer(
        &self,
        header: &str,
        mat_name: Option<&str>,
        mtl_path: Option<&str>,
    ) -> Result<Vec<u8>, std::io::Error> {
        let mut obj = Vec::new();
        let o = &mut obj;

        writeln!(o, "# {}", header)?;

        if mtl_path.is_some() && mat_name.is_some() {
            writeln!(o, "mtllib {}", mtl_path.unwrap())?;
            writeln!(o, "usemtl {}", mat_name.unwrap())?;
        }
        for vert in self.verts.iter() {
            writeln!(o, "v {} {} {}", vert.x, vert.y, vert.z)?;
        }
        for uv in self.uvs.iter() {
            writeln!(o, "vt {} {}", uv.x, uv.y)?;
        }

        if self.uvs.len() == self.verts.len() {
            for (a, b, c) in self.get_triangles() {
                let (a, b, c) = (a + 1, b + 1, c + 1);
                writeln!(o, "f {a}/{a} {b}/{b} {c}/{c}")?;
            }
        } else {
            for (a, b, c) in self.get_triangles() {
                let (a, b, c) = (a + 1, b + 1, c + 1);
                writeln!(o, "f {a} {b} {c}")?;
            }
        }
        Ok(obj)
    }
}

///////////////////////////////////////////////////////////////////////////////

impl PointBased for Mesh {
    // TODO how to IntoIterator, so we don't have to iter / collect
    fn mutate_points<'a>(&'a mut self) -> Vec<&'a mut Vec3> {
        self.verts.iter_mut().collect() // its a bit sad we have to do this.
    }
}

#[cfg(test)]
mod test {
    use super::Mesh;
    use crate::core::Geometry;
    use crate::kernel::vec3;
    use std::io::Write;

    // #[test]
    // fn write_some_obj() {
    //     let mesh = Mesh::new_diamond(vec3(0.5, 0.5, 0.5), 1.333);

    //     mesh.write_obj_mtl("../data-results/", "some.obj", "some.mtl", "some.png")
    //         .expect("something went wrong!");
    // }

    
    #[test]
    fn test_hexagrid() {
        let mesh = Mesh::new_hexagrid(2.0, 1);
    }

    // #[test]
    // fn write_file() {
    //     let mut buffer = Vec::new();
    //     writeln!(&mut buffer, "test").unwrap();
    //     writeln!(&mut buffer, "formatted {}", "arguments").unwrap();

    //     let mut file = std::fs::File::create("data.txt").expect("create failed");
    //     file.write_all(&buffer).expect("write failed");
    // }

    #[test]
    fn transform_mesh() {
        let mut mesh = Mesh::new_diamond(vec3(0.5, 0.5, 0.5), 0.5);
        mesh = mesh.mv(-vec3(0.5, 0.5, 0.5));
        mesh = mesh.scale_u(2.0);
        assert_eq!(
            mesh.verts,
            vec![
                vec3(1.0, 0.0, 0.0),
                vec3(-1.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
                vec3(0.0, -1.0, 0.0),
                vec3(0.0, 0.0, 1.0),
                vec3(0.0, 0.0, -1.0)
            ]
        );
    }
}
