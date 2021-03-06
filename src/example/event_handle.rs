extern mod arkoh;

use arkoh::window::Window;
use arkoh::event;

fn render_loop(window: &mut Window) {
    window.poll_events(event_handler);
}

fn main() {
    do Window::create_window|window| {
       window.render_loop(render_loop);
    };
}

fn event_handler(_: &mut Window, event: &event::Event) -> bool {
    match *event {
        event::KeyPressed(code) => {
            println!("You pressed the key with code: {:?}", code);
            println!("Do not try to press escape: the callback returns `false` (does not propagate events)!");
            false // override the default keyboard handler
        },
        event::KeyReleased(code) => {
            println!("You released the key with code: {:?}", code);
            println!("Do not try to press escape: the callback returns `false` (does not propagate events)!");
            false // override the default keyboard handler
        },
        event::ButtonPressed(button, mods) => {
            println!("You pressed the mouse button with code: {:?}", button);
            println!("You pressed the mouse button with modifiers: {:?}", mods);
            true // dont override the default mouse handler
        },
        event::ButtonReleased(button, mods) => {
            println!("You released the mouse button with code: {:?}", button);
            println!("You released the mouse button with modifiers: {:?}", mods);
            true // dont override the default mouse handler
        },
        event::CursorPos(x, y) => {
            println!("Cursor pos: ({} , {})", x, y);
            true // dont override the default mouse handler
        },
        event::Scroll(xshift, yshift) => {
            println!("Cursor pos: ({} , {})", xshift, yshift);
            true // dont override the default mouse handler
        },
        _ => true
    }
}

