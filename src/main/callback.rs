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
// Scroll Callback
//
pub struct ScrollCallback {
    collector: RWArc<~[event::Event]>
}

impl ScrollCallback {
    pub fn new(collector: RWArc<~[event::Event]>) -> ScrollCallback {
        ScrollCallback {
            collector: collector
        }
    }
}

impl glfw::ScrollCallback for ScrollCallback {
    fn call(&self, _: &glfw::Window, x: f64, y: f64) {
        self.collector.write(|c| c.push(event::Scroll(x as f32, y as f32)))
    }
}


//
// Cursor Pos Callback
//
pub struct CursorPosCallback {
    collector: RWArc<~[event::Event]>
}

impl CursorPosCallback {
    pub fn new(collector: RWArc<~[event::Event]>) -> CursorPosCallback {
        CursorPosCallback {
            collector: collector
        }
    }
}

impl glfw::CursorPosCallback for CursorPosCallback {
    fn call(&self, _: &glfw::Window, x: f64, y: f64) {
        self.collector.write(|c| c.push(event::CursorPos(x as f32, y as f32)))
    }
}

//
// Mouse Button Callback
//
pub struct MouseButtonCallback {
    collector: RWArc<~[event::Event]>
}

impl MouseButtonCallback {
    pub fn new(collector: RWArc<~[event::Event]>) -> MouseButtonCallback {
        MouseButtonCallback {
            collector: collector
        }
    }
}

impl glfw::MouseButtonCallback for MouseButtonCallback {
    fn call(&self,
            _:      &glfw::Window,
            button: glfw::MouseButton,
            action: glfw::Action,
            mods:   glfw::Modifiers) {
        if action == glfw::Press {
            self.collector.write(|c| c.push(event::ButtonPressed(button, mods)))
        }
        else {
            self.collector.write(|c| c.push(event::ButtonReleased(button, mods)))
        }
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

//
// Framebuffer callback
//
pub struct FramebufferSizeCallback {
    collector: RWArc<~[event::Event]>
}

impl FramebufferSizeCallback {
    pub fn new(collector: RWArc<~[event::Event]>) -> FramebufferSizeCallback {
        FramebufferSizeCallback {
            collector: collector
        }
    }
}

impl glfw::FramebufferSizeCallback for FramebufferSizeCallback {
    fn call(&self, _: &glfw::Window, w: i32, h: i32) {
        self.collector.write(|c| c.push(event::FramebufferSize(w as f32, h as f32)))
    }
}

