
// Prepare the modules
mod rendering;

// Import all of the requirements
use rendering::prelude::*;
use pollster;

// Initialize the engine
fn main() {

    // Create a new WindowHandler
    let mut w = WindowHandler::new();

    // Initialize the window
    pollster::block_on(w.start());
}
