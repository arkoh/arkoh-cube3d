pub static vertex_src: &'static str =
    "#version 120
    attribute vec2 position;
     
    void main(void) {
        gl_Position = vec4(position, 0.0, 1.0);
    }";


pub static fragment_src: &'static str =
    "#version 120
    uniform vec3 color;

    void main(void) {
        gl_FragColor = vec4(color, 1.0);
    }";

