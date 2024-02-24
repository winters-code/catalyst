
use std::collections::{HashMap};

pub enum InputState {
    Pressed,
    Held,
    Released
}

pub struct Inputs {
    keys: HashMap<String, u32>,
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            HashMap::new()
        }
    }

    pub fn set(&self, key, state) {

    }
}