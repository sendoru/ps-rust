use std::{cmp, io, str::FromStr, sync::Arc};
pub struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>
}

impl Dsu {
    pub fn new(size: usize) -> Self {
        let mut parent = vec![0 as usize; size];
        for i in 0 .. size {
            parent[i] = i;
        }
        Self {parent: parent, size: vec![1 as usize; size]}
    }

    pub fn get_parent(&mut self, idx: usize) -> usize {
        if self.parent[idx] == idx {
            return idx;
        }
        
        self.parent[idx] = self.get_parent(self.parent[idx]);
        return self.parent[idx];
    }

    pub fn get_size(&mut self, idx: usize) -> usize {
        let idx_new = self.get_parent(idx);
        return self.size[idx_new];
    }
    
    /* Merge elem ```a``` and elem ```b``` 
     */
    pub fn unite (&mut self, a: usize, b: usize) {
        let a_par = self.get_parent(a);
        let b_par = self.get_parent(b);
        if a_par == b_par {
            return;
        }

        if a_par < b_par {
            self.size[a_par] += self.size[b_par];
            self.parent[b_par] = self.parent[a_par];
        }
        else {
            self.size[b_par] += self.size[a_par];
            self.parent[a_par] = self.parent[b_par];
        }
    }
}