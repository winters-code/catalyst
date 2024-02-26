
// Import all of the requirements
extern crate glfw;

use glfw::{Action, Context, Key, fail_on_errors};
use crate::gfx::sys::input_handler::{InputHandler};

// Window handler struct
// This includes all the required functions to handle the window.
pub struct WindowHandler {
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>
}

// Implement all of the functions for the window handler
impl WindowHandler {

    // Initialize a window and prepare it for drawing
    pub fn new() -> WindowHandler {
        
        // Create the instance of GLFW
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();

        // Create the window and event manager
        #[allow(unused_mut)]
        let (mut window, events) = glfw.create_window(680, 420, "Test", glfw::WindowMode::Windowed)
            .expect("Failed to make the GLFW window");

        // Pass the window handler back out
        WindowHandler {
            glfw: glfw,
            window: window,
            events: events
        }
        // InputHandler::new(&window, &events, &glfw)
    }

    // Start up the window & enable the event loop
    pub async fn start(&mut self) {

        // Make the window the currently active one
        self.window.make_current();

        // Start polling for key events
        self.window.set_key_polling(true);

        // While the window is open
        println!("Started the window");
        while !self.window.should_close() {

            // Swap the rendering buffers
            self.window.swap_buffers();

            // Get all of the events
            self.glfw.poll_events();

            for (_, event) in glfw::flush_messages(&(self.events)) {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true)
                    },
                    _ => {},
                }
            }
        }
    }

    pub fn window(&self) -> &glfw::PWindow {
        &self.window
    }
}
