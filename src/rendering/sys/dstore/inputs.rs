
use std::collections::{HashMap};

pub enum InputState {
    Pressed,
    Held,
    Released
}

pub struct Inputs {
    keys: HashMap<String, InputState>
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            keys: HashMap::new()
        }
    }

    pub fn set(&self, key: String, state: InputState) {

    }
}