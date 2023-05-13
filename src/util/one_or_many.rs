/// NOTE: one day, this will create a bug where One<T> turns into a list of Many, all copying One
#[derive(Debug, Clone)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<T: Clone> OneOrMany<T> {
    pub fn get_id(&self, i: usize) -> usize {
        match self {
            OneOrMany::One(_) => 0,
            OneOrMany::Many(_) => i,
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        match self {
            OneOrMany::One(one) => Some(&one),
            OneOrMany::Many(many) => many.get(i),
        }
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        match self {
            OneOrMany::One(one) => Some(one),
            OneOrMany::Many(many) => many.get_mut(i),
        }
    }

    // map the inner vec of many
    pub fn clone_map<F: FnMut(Vec<T>) -> Vec<T>>(self, mut callback: F) -> Self {
        match self {
            OneOrMany::One(one) => OneOrMany::One(one),
            OneOrMany::Many(many) => OneOrMany::Many(callback(many)),
        }
    }

    // map the inner vec of many
    pub fn map<F: FnMut(&Vec<T>) -> Vec<T>>(&self, mut callback: F) -> Self {
        match self {
            OneOrMany::One(one) => OneOrMany::One(one.clone()),
            OneOrMany::Many(many) => OneOrMany::Many(callback(many)),
        }
    }

    pub fn push(&mut self, item: T) {
        if let OneOrMany::Many(many) = self {
            many.push(item)
        }
    }
}
