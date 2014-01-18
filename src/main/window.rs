use glfw;
use std::libc;


pub struct Window<'a> {
    window: glfw::Window
}

impl<'a> Window<'a> {

    pub fn create_window(user_callback:proc(&mut Window)) {
        glfw::set_error_callback(~ErrorContext);

        do glfw::start {
            let window = glfw::Window::create(300, 300, "Hello this is window", glfw::Windowed)
                         .expect("Failed to create GLFW window.");

            window.set_key_callback(~KeyContext);
            window.make_context_current();

            let mut user_window = Window { window: window };
            user_callback(&mut user_window);
        }
    }

    pub fn render_loop(&mut self, callback: fn(&mut Window) ) {
        while !self.window.should_close() {
            glfw::poll_events();
            callback(self);
        }
    }
}

struct ErrorContext;
impl glfw::ErrorCallback for ErrorContext {
    fn call(&self, _: glfw::Error, description: ~str) {
        println!("GLFW Error: {:s}", description);
    }
}

struct KeyContext;
impl glfw::KeyCallback for KeyContext {
    fn call(&self, window: &glfw::Window, key: glfw::Key, _: libc::c_int, action: glfw::Action, _: glfw::Modifiers) {
        if action == glfw::Press && key == glfw::KeyEscape {
            window.set_should_close(true);
        }
    }
}
