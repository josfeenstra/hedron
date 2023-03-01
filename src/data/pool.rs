use std::collections::HashMap;

/// A dumb pointer
pub type Ptr = usize;

/// A data pool. 
/// If you desire a Vec, and ID's pointing to that vec, 
/// PLUS delete functionality, this is the data type to use
#[derive(Default, Debug, Clone)]
pub struct Pool<T> {
    data: Vec<Option<T>>,
    freed_ids: Vec<Ptr>, // I prefer to work with a stack of freed spots, than iterating over all spots everytime we add a new one
}

impl<T: Clone> Pool<T> {
    
    /// Returns a list which maps the old indices to their new indices
    /// Get this before actual remapping.
    pub fn get_refactor_mapping(&self) -> HashMap<usize, usize> {
        let mut mapping = HashMap::new();
        let mut offset = 0;
        for i in 0..self.data.len() {
            if let Some(_) = self.get(i) {
                mapping.insert(i, i - offset);
            } else {
                offset += 1;
            }
        }
        mapping
    }

    /// clean up all empty spots within the vector
    /// WARNING: this invalidates all externally stored pointers!
    
    pub fn refactor(&mut self) {
        
        self.freed_ids.clear();
        let mut offset = 0;
        for i in 0..self.data.len() {
            if let Some(item) = self.get(i) {
                self.set(i - offset, item.clone())
            } else {
                offset += 1;
            }
        }

        // remove the last X items, which should be correctly copied and replaced
        for _i in 0..offset {
            self.data.pop();
        }
    }
}

impl<T> Pool<T> {

    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            freed_ids: Vec::new(),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
            freed_ids: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len() - self.freed_ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty() && self.freed_ids.is_empty()
    }

    pub fn push(&mut self, item: T) -> Ptr {
        // consume a freed spot if a freed spot is available
        if let Some(ptr) = self.freed_ids.pop() {
            assert!(self.get(ptr).is_none());
            self.data[ptr] = Some(item);
            ptr
        } else {
            self.data.push(Some(item));
            let ptr = self.data.len() - 1;
            ptr
        }
    }

    pub fn delete(&mut self, ptr: Ptr) {
        assert!(self.data.get(ptr).is_some());
        self.freed_ids.push(ptr);
        self.data[ptr] = None;
    }

    pub fn set(&mut self, ptr: Ptr, item: T) {
        self.data[ptr] = Some(item);
    }

    pub fn get(&self, ptr: Ptr) -> Option<&T> {
        let res = self.data.get(ptr)?;
        res.as_ref()
    }

    pub fn get_mut(&mut self, ptr: Ptr) -> Option<&mut T> {
        let res = self.data.get_mut(ptr)?;
        res.as_mut()
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item=&T> + 'a {
        self.data.iter()
            .filter(|i| i.is_some())
            .map(|i| i.as_ref().unwrap())
        
    }

    pub fn iter_enum<'a>(&'a self) -> impl Iterator<Item=(usize, &T)> + 'a {
        self.data.iter()
            .enumerate()
            .filter(|(_, item)| item.is_some())
            .map(|(i, item)| (i, item.as_ref().unwrap()))
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item=&mut T> + 'a {
        self.data.iter_mut()
            .filter(|i| i.is_some())
            .map(|i| i.as_mut().unwrap())
    }

    pub fn iter_enum_mut<'a>(&'a mut self) -> impl Iterator<Item=(usize, &T)> + 'a {
        self.data.iter_mut()
            .enumerate()
            .filter(|(_, item)| item.is_some())
            .map(|(i, item)| (i, item.as_ref().unwrap()))
    }

    pub fn all(&self) -> Vec<&T> {
        self.iter().collect()
    }

    pub fn all_ids(&self) -> Vec<Ptr> {
        self.iter_enum().map(|(ptr, _)| ptr).collect()
    }
}


#[cfg(test)]
mod tests {
    use crate::data::Pool;

    #[test]
    fn test_mutations() {
        println!("testtest");

        let mut pool = Pool::new();

        let henk_ptr = pool.push("henk");
        let blob_ptr = pool.push("blob");
        let kaas_ptr = pool.push("kaas");
        
        for item in pool.iter() {
            println!("{:?}", item);
        }

        assert_eq!(pool.all(), vec![&"henk", &"blob", &"kaas", &"piet"]);

        pool.delete(blob_ptr);
        pool.delete(kaas_ptr);

        assert_eq!(pool.all(), vec![&"henk", &"piet"]);
        
        pool.set(henk_ptr, "penk");

        assert_eq!(pool.all(), vec![&"penk", &"piet"]);

        println!("{:?}", pool);
        
        let _muis_ptr = pool.push("muis"); 
        let _puis_ptr = pool.push("puis"); 
        let _duis_ptr = pool.push("duis"); 
        
        assert_eq!(pool.all(), vec![&"penk", &"puis", &"muis", &"piet", &"duis"]);
        
        pool.delete(1);
        pool.delete(3);
        
        println!("{:?}", pool);
        pool.refactor();
        println!("{:?}", pool);
        
        assert_eq!(pool.all(), vec![&"penk", &"muis", &"duis"]);

        
    }
    
    
    #[test]
    fn test_iterations() {
        let mut pool = Pool::new();
        let _henk_ptr = pool.push("henk".to_owned());
        let _blob_ptr = pool.push("blob".to_owned());
        let _kaas_ptr = pool.push("kaas".to_owned());
        let _piet_ptr = pool.push("piet".to_owned());

        for item in pool.iter_mut() {
            item.make_ascii_uppercase();
        }
        
        assert_eq!(pool.iter().map(|s| s.as_str()).collect::<Vec<_>>(), vec!["HENK", "BLOB", "KAAS", "PIET"]);
    }
}