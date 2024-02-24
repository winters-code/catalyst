
extern crate glfw;

use glfw::{Action, Context, Key};

pub struct InputHandler {
    window: &glfw::Window,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>
}

impl InputHandler {
    pub fn new(window: &glfw::Window, events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) -> InputHandler {
        InputHandler {
            window: window,
            events: events
        }
    }

    fn handle_individual_event(&self, e: glfw::WindowEvent) {
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