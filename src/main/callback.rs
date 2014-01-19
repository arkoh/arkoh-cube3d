use glfw;
use std::libc;
use event;
use extra::arc::RWArc;

pub struct ErrorContext;
impl glfw::ErrorCallback for ErrorContext {
    fn call(&self, _: glfw::Error, description: ~str) {
        println!("GLFW Error: {:s}", description);
    }
}

//
// Key callback
//
pub struct KeyCallback {
    collector: RWArc<~[event::Event]>
}

impl KeyCallback {
    pub fn new(collector: RWArc<~[event::Event]>) -> KeyCallback {
        KeyCallback {
            collector: collector
        }
    }
}

impl glfw::KeyCallback for KeyCallback {
    fn call(&self,
            _:      &glfw::Window,
            key:    glfw::Key,
            _:      libc::c_int,
            action: glfw::Action,
            _:      glfw::Modifiers) {
        if action == glfw::Press {
            self.collector.write(|c| c.push(event::KeyPressed(key)))
        }
        else {
            self.collector.write(|c| c.push(event::KeyReleased(key)))
        }
    }
}
