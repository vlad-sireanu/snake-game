use rand::{thread_rng, Rng};

use std::collections::HashMap;
use std::hash::Hash;

pub struct Bijection<T: Eq + Copy + Hash> {
    vec: Vec<T>,
    map: HashMap<T, usize>,
}

impl<T: Eq + Copy + Hash> Bijection<T> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            map: HashMap::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            vec: Vec::with_capacity(capacity),
            map: HashMap::with_capacity(capacity),
        }
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
    pub fn contains(&self, elem: T) -> bool {
        self.map.contains_key(&elem)
    }
    pub fn insert(&mut self, elem: T) {
        self.map.insert(elem, self.len());
        self.vec.push(elem);
    }
    pub fn remove_elem(&mut self, elem: &T) {
        let i = *self.map.get(elem).unwrap();
        let last_i = self.len() - 1;

        self.vec.swap(i, last_i);
        self.map.insert(self.vec[i], i);

        self.map.remove(elem);
        self.vec.pop();
    }
    pub fn pop_random(&mut self) -> T {
        let i = thread_rng().gen_range(0..self.len());
        let elem = self.vec[i];
        self.remove_elem(&elem);
        elem
    }
}
