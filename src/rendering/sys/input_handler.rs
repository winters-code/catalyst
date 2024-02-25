
extern crate glfw;

use glfw::{Action, Context, Key};
use crate::rendering::sys::dstore::inputs::{Inputs, InputState};

pub struct InputHandler<'core> {
    window: &'core mut glfw::PWindow,
    events: &'core glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    inputs: Inputs
}

impl <'core> InputHandler<'core>  {
    pub fn new(window: &'core mut glfw::PWindow, events: &'core glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) -> InputHandler<'core> {
        let mut inputs: Inputs = Inputs::new();
        InputHandler {
            window: window,
            events: events,
            inputs: inputs
        }
    }

    fn handle_individual_event(&mut self, e: glfw::WindowEvent) {
        match e {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                self.window.set_should_close(true)
            },
            _ => {},
        }
    }

    pub fn handle_events(&self) {

    }
}