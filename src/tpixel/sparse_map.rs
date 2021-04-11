use std::collections::HashMap;

pub struct SparseMapItem<T> {
    key : u32, // read only
    pub value : T, // read & write
}

pub struct SparseMap<T> {
    data : Vec<SparseMapItem<T>>,
    map : HashMap<u32, usize>,
}

impl<T> SparseMapItem<T> {
    pub fn get_key(&self) -> u32 {
        self.key
    }
}

impl<T> SparseMap<T> {
    pub fn new() -> SparseMap<T> {
        SparseMap {data : Vec::new(), map : HashMap::new(), }
    }
    pub fn insert(&mut self, key : u32, value : T) {
        self.map.insert(key, self.data.len());
        self.data.push(SparseMapItem {key : key, value : value});
    }
    pub fn contains_key(&self, key : u32) -> bool {
        self.map.contains_key(&key)
    }
    pub fn get(&self, key : u32) -> &T {
        &self.data[self.map[&key]].value
    }
    pub fn get_mut(&mut self, key : u32) -> &mut T {
        &mut self.data[self.map[&key]].value
    }
    pub fn remove(&mut self, key : u32) {
        let index = self.map[&key];
        self.data.swap_remove(index);
        *self.map.get_mut(&self.data[index].key).unwrap() = index;
        self.map.remove(&key);
    }
    pub fn all_iter(&self) -> impl Iterator<Item = &SparseMapItem<T>> {
        self.data.iter()
    }
    pub fn all_iter_mut(&mut self) -> impl Iterator<Item = &mut SparseMapItem<T>> {
        self.data.iter_mut()
    }
}
