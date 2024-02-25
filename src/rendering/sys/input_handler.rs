
// Load the GLFW crate
extern crate glfw;

// Get all the requirements for the file
use glfw::{Action, Context, Key};
use crate::rendering::sys::dstore::inputs::{Inputs, InputState};

// Create the InputHandler struct with a core lifetime
pub struct InputHandler<'core> {
    glfw: &'core glfw::Glfw,
    window: &'core mut glfw::PWindow,
    events: &'core glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    inputs: Inputs
}

// Load all the functions for the InputHand;er
impl <'core> InputHandler<'core>  {

    // Create a new InputHandler
    pub fn new(window: &'core mut glfw::PWindow, 
               events: &'core glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
               glfw: &'core glfw::Glfw) -> InputHandler<'core> {

        // The struct to save all the inputs
        let mut inputs: Inputs = Inputs::new();

        // Create & pop the input handler
        InputHandler {
            glfw: glfw,
            window: window,
            events: events,
            inputs: inputs
        }
    }

    // Parse every individual input event
    fn handle_individual_event(&mut self, e: glfw::WindowEvent) {

        // Check every possibility
        match e {

            // Pressing the escape key
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                self.window.set_should_close(true)
            },
            _ => {},
        }
    }

    // Handle every single event
    pub fn handle_events(&self) {
        for (_, event) in self.glfw::flush_messages(&(self.events)) {

        }
    }
}