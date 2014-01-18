extern mod arkoh;

use arkoh::window::Window;

fn render_loop(window: &mut Window) {
}

fn main() {
    do Window::create_window|window| {
       window.render_loop(render_loop);
    };
}
