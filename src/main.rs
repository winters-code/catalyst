
// Prepare the modules
mod rendering;

// Import all of the requirements
use rendering::prelude::*;

// Initialize the engine
fn main() {

    // Create a new WindowHandler
    let mut w = WindowHandler::new();

    // Initialize the window
    w.start();
}
