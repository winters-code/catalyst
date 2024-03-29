
// I am hiding warnings because I just want to see errors currently.
#![allow(warnings)]

// Prepare the modules
mod rendering;

// Import all of the requirements
use rendering::prelude::*;
use rendering::utils::*;
use pollster;

// Initialize the engine
fn main() {

    // Create a new WindowHandler
    let mut wh = WindowHandler::new();

    // Initialize the window
    pollster::block_on(wh.start());
}
