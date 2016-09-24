extern crate rustbox;
extern crate miv;

use std::env;
use rustbox::{Key,RustBox};

use miv::state::State;
use miv::view::View;

fn main() {
    let rustbox = RustBox::init(Default::default()).unwrap();
    let view = View::new(&rustbox);

    let mut state = State::new(rustbox.width(), rustbox.height());

    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        state.buffer.load_file(path.clone());
        state.buffer.filepath = Some(path.clone());
    }

    rustbox.clear();
    rustbox.set_cursor(0, 0);
    view.render(&state);

    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                if key == Key::Ctrl('c') {
                    break;
                }
                let exit = state.handle_key(key);

                if exit {
                    break;
                }
            },
            Ok(rustbox::Event::ResizeEvent(w, h)) => {
                state.width = w as usize;
                state.height = h as usize;
            }
            Err(e) => panic!("{}", e),
            _ => {}
        }

        view.render(&state);
        state.status = None;
    }
}

