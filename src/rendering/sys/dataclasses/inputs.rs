
// Load hashmaps
use std::collections::{HashMap};

// The state of an input
pub enum InputState {

    // General, commonplace inputs
    Pressed,
    Held,
    Released,
    Off,

    // Specifically for scrolling
    Up,
    Down
}

// Create the inputs struct
pub struct Inputs {
    keys: HashMap<String, InputState>
}

// Load all the functions for input
impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            keys: HashMap::new()
        }
    }

    pub fn set(&self, key: String, state: InputState) {

    }
}