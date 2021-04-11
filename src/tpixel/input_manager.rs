use std::collections::HashMap;
use glfw::{Action, Key};

struct KeyState {
    this_frame : bool,
    last_frame : bool,
}

pub struct InputManager {
    key_states : HashMap<Key, KeyState>,
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            key_states : HashMap::new(),
        }
    }
    pub fn update_input(&mut self, window : &glfw::Window) {
        for (_key, value) in self.key_states.iter_mut() {
            //value.this_frame = window.get_key(*key) == Action::Press;
            value.last_frame = value.this_frame;
        }
    }
    pub fn update_event(&mut self, key : Key, action : Action){
        if !self.key_states.contains_key(&key) {
            self.key_states.insert(key, KeyState {this_frame : false, last_frame : false});
        }
        self.key_states.get_mut(&key).unwrap().this_frame = action != Action::Release;
    }
    pub fn is_key_pressed(&self, key : Key) -> bool {
        if self.key_states.contains_key(&key) {
            let state = self.key_states.get(&key).unwrap();
            state.this_frame && (state.this_frame != state.last_frame)
        } else {
            false
        }
    }
    pub fn is_key_down(&self, key : Key) -> bool {
        if self.key_states.contains_key(&key) {
            let state = self.key_states.get(&key).unwrap();
            state.this_frame
        } else {
            false
        }
    }
    pub fn is_key_released(&self, key : Key) -> bool {
        if self.key_states.contains_key(&key) {
            let state = self.key_states.get(&key).unwrap();
            (!state.this_frame) && (state.this_frame != state.last_frame)
        } else {
            false
        }
    }
}
