use glfw;
use gl;
use event;
use callback::{KeyCallback, ScrollCallback, CursorPosCallback, MouseButtonCallback, FramebufferSizeCallback, ErrorContext};
use extra::arc::RWArc;

pub struct Window<'a> {
    window: glfw::Window,
    priv events: RWArc<~[event::Event]>,
}

impl<'a> Window<'a> {

    pub fn create_window(user_callback:proc(&mut Window)) {
        glfw::set_error_callback(~ErrorContext);

        do glfw::start {
            let window = glfw::Window::create(300, 300, "Hello this is window", glfw::Windowed)
                         .expect("Failed to create GLFW window.");
            window.make_context_current();

            gl::load_with(glfw::get_proc_address);

            gl::FrontFace(gl::CCW);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::SCISSOR_TEST);
            gl::DepthFunc(gl::LEQUAL);


            let mut user_window = Window { window: window,  events: RWArc::new(~[]) };

            // Add framebuffer callback
            let collector = user_window.events.clone();
            user_window.window.set_framebuffer_size_callback(~FramebufferSizeCallback::new(collector)); 
           
            // Add keyboard callback
            let collector = user_window.events.clone();
            user_window.window.set_key_callback(~KeyCallback::new(collector));

            // Add mouse botton callback
            let collector = user_window.events.clone();
            user_window.window.set_mouse_button_callback(~MouseButtonCallback::new(collector));

            // Add cursor pos callback
            let collector = user_window.events.clone();
            user_window.window.set_cursor_pos_callback(~CursorPosCallback::new(collector));

            // Add scroll callback
            let collector = user_window.events.clone();
            user_window.window.set_scroll_callback(~ScrollCallback::new(collector));

 
            user_callback(&mut user_window);
        }
    }

    pub fn poll_events(&mut self, events_handler: |&mut Window, &event::Event| -> bool) {
        // redispatch them
        let events = self.events.clone();
        events.read(|es| {
            for e in es.iter() {
                if events_handler(self, e) {
                    match *e {
                        event::KeyReleased(key) => {
                            if key == glfw::KeyEscape {
                                self.close();
                                continue
                            }
                        },
                        _ => { }
                    }

                }
            }
        });

        // clear the events collector
        self.events.write(|c| c.clear());
    }

    pub fn render_loop(&mut self, callback: fn(&mut Window) ) {
        while !self.window.should_close() {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::ClearColor(0.0,1.0,1.0,1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
           
            gl::Scissor(0 as i32, 0 as i32, 300 as i32, 300 as i32);

            glfw::poll_events();
            callback(self);
            self.window.swap_buffers();
        }
    }

    /// Closes the window.
    pub fn close(&mut self) {
        self.window.set_should_close(true)
    }

}

/*
struct ErrorContext;
impl glfw::ErrorCallback for ErrorContext {
    fn call(&self, _: glfw::Error, description: ~str) {
        println!("GLFW Error: {:s}", description);
    }
}

//
// Key callback
//
struct KeyCallback {
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
*/
