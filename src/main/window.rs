use glfw;
use gl;
use gl::types::*;
use event;
use callback::{KeyCallback, ScrollCallback, CursorPosCallback, MouseButtonCallback, FramebufferSizeCallback, ErrorContext};
use extra::arc::RWArc;
use object::Object;
use shader::shaders;
use vertices::*;
use std::mem;
use std::cast;
use std::ptr;

pub struct Window<'a> {
    window: glfw::Window,
    priv events: RWArc<~[event::Event]>,
    objects: ~[~Object],
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


            let mut user_window = Window { window: window,  events: RWArc::new(~[]), objects: ~[] };

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
        // Create Vertex Array Object
        let mut vao: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1,&mut vao);
            gl::BindVertexArray(vao);
        }

        // Create a Vertex Buffer Object and copy the vertex data to it
        let mut vbo: GLuint = 0;
        unsafe {
            gl::GenBuffers(1,&mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                          (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                          cast::transmute(&vertices[0]),
                          gl::STATIC_DRAW);
         } 

        // Create and compile the vertex shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        unsafe {
            gl::ShaderSource(vertex_shader, 1, &shaders::vertex_src.to_c_str().unwrap(), ptr::null());
            gl::CompileShader(vertex_shader);
        }
        
        // Create and compile the fragment shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        unsafe {
            gl::ShaderSource(fragment_shader, 1, &shaders::fragment_src.to_c_str().unwrap(), ptr::null());
            gl::CompileShader(fragment_shader);
        }


        // Link the vertex and fragment shader into a shader program
        let shader_program = gl::CreateProgram();
       
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
            //"outColor".with_c_str(|ptr| gl::BindFragDataLocation(shader_program, 0, ptr));
        gl::LinkProgram(shader_program);
        gl::UseProgram(shader_program);
        

        // Specify the layout of the vertex data
        let pos_attrib = unsafe{ "position".with_c_str(|ptr| gl::GetAttribLocation(shader_program, ptr)) as GLuint };
        unsafe {
            gl::EnableVertexAttribArray(pos_attrib);
            gl::VertexAttribPointer(pos_attrib, 3, gl::FLOAT, gl::FALSE,
                                    3 * mem::size_of::<GLfloat>() as GLsizei,
                                    ptr::null());
        }

        let color_location = unsafe {
            "color".with_c_str(|ptr| gl::GetUniformLocation(shader_program, ptr))
        };

        while !self.window.should_close() {
            glfw::poll_events();
            callback(self);
            self.poll_events(|_, _| true);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::ClearColor(0.0,1.0,1.0,1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Clear(gl::DEPTH_BUFFER_BIT);
            

            for obj in self.objects.iter() {
                obj.draw(color_location);
            }
            
            //gl::Scissor(0 as i32, 0 as i32, 300 as i32, 300 as i32);

            self.window.swap_buffers();
        }

        unsafe {
            gl::DeleteProgram(shader_program);
            gl::DeleteShader(fragment_shader);
            gl::DeleteShader(vertex_shader);

            gl::DeleteBuffers(1, &vbo);

            gl::DeleteVertexArrays(1, &vao);
        }
    }

    pub fn add_cube(&mut self) {
        let res = ~Object::new(cube_begin, cube_end, 1.0, 1.0, 1.0);
        self.objects.push(res);
    }


    /// Closes the window.
    pub fn close(&mut self) {
        self.window.set_should_close(true)
    }

}
