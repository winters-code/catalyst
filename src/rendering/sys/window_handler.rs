
// Import all of the requirements
extern crate glfw;

use glfw::{Action, Context, Key, fail_on_errors};

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
    }

    // Start up the window & enable the event loop
    pub fn start(&mut self) {

        self.window.make_current();
        self.window.set_key_polling(true);

        while !self.window.should_close() {
            // Swap front and back buffers
            self.window.swap_buffers();

            // Poll for and process events
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

        println!("Started the window");
    }
}
