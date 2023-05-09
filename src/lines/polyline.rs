use crate::kernel::Vec3;

/// A list of points that will have a line drawn between each consecutive points
/// An enum is used, so we don't have to deal with a duplicate last point, closed boolean, is_closed, etc.
#[derive(Debug, Clone)]
pub enum Polyline {
    Open(Vec<Vec3>),
    Closed(Vec<Vec3>),
}

impl Polyline {
    pub fn new_closed(verts: Vec<Vec3>) -> Self {
        Polyline::Closed(verts)
    }

    pub fn new_open(verts: Vec<Vec3>) -> Self {
        Polyline::Open(verts)
    }

    pub fn get_verts(&self) -> &Vec<Vec3> {
        match self {
            Polyline::Open(verts) => verts,
            Polyline::Closed(verts) => verts,
        }
    }

    pub fn get_verts_mut(&mut self) -> &mut Vec<Vec3> {
        match self {
            Polyline::Open(verts) => verts,
            Polyline::Closed(verts) => verts,
        }
    }

    /// only here we duplicate the last vertex
    /// TODO: how to make this an iterator
    pub fn get_verts_for_rendering(&self) -> Vec<Vec3> {
        let mut new_verts = self.get_verts().clone();
        match self {
            Polyline::Open(_) => new_verts,
            Polyline::Closed(_) => {
                if let Some(first) = new_verts.first() {
                    new_verts.push(first.clone());
                }

                new_verts
            }
        }

        // match self {
        //     Polyline::Open(verts) => verts.into_iter(),
        //     Polyline::Closed(verts) => {
        //         // add the first vert as the last
        //         match verts.first() {
        //             None => verts.iter(),
        //             Some(first) => {
        //                 let last_vert = iter::once(first);
        //                 verts.iter().chain(last_vert)
        //                 // verts.iter()
        //             }
        //         }
        //     }
        // }
    }

    /// By using the enum, this should not be needed
    /// but just in case:
    pub fn is_closed(&self) -> bool {
        matches!(self, Polyline::Closed(_))
    }

    pub fn close(self) -> Self {
        match self {
            Polyline::Open(verts) => Polyline::Closed(verts),
            Polyline::Closed(_) => self,
        }
    }

    pub fn open(self) -> Self {
        match self {
            Polyline::Open(_) => self,
            Polyline::Closed(verts) => Polyline::Open(verts),
        }
    }
}
