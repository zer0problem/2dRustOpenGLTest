use std::collections::HashMap;
use std::any::TypeId;
use std::any::Any;
use crate::tpixel::sparse_map::SparseMap;

pub struct Registry {
    next_id : u32,
    component_maps : HashMap<TypeId, Box<dyn Any>>,
}

impl Registry {
    pub fn new() -> Registry {
        Registry{next_id : 0, component_maps : HashMap::new() }
    }
    pub fn create_entity(&mut self) -> u32 {
        // TODO manage their components in the component_maps so you can destroy an entity and remove all its components
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    pub fn init_map<T : Any>(&mut self) {
        let ti : TypeId = TypeId::of::<T>();
        let a : Box<SparseMap<T>> = Box::new(SparseMap::<T>::new());
        self.component_maps.insert(ti, a);
    }
    pub fn get_map<T : Any>(&self) -> &SparseMap<T> {
        let ti : TypeId = TypeId::of::<T>();
        let a : Option<&Box<dyn Any>> = self.component_maps.get(&ti);
        let b : &Box<dyn Any> = a.unwrap();
        let c : Option<&SparseMap<T>> = b.downcast_ref::<SparseMap<T>>();
        c.unwrap()
    }
    pub fn get_map_mut<T : Any>(&mut self) -> &mut SparseMap<T> {
        let ti : TypeId = TypeId::of::<T>();
        let a : Option<&mut Box<dyn Any>> = self.component_maps.get_mut(&ti);
        let b : &mut Box<dyn Any> = a.unwrap();
        let c : Option<&mut SparseMap<T>> = b.downcast_mut::<SparseMap<T>>();
        c.unwrap()
    }
}
