extern crate rustbox;
extern crate miv;

use std::env;
use rustbox::RustBox;
use rustbox::Key;
use miv::state::State;
use miv::view::View;

fn main() {
    let mut options = rustbox::InitOptions::default();
    options.output_mode = rustbox::OutputMode::EightBit;
    options.buffer_stderr = true;
    let rustbox = RustBox::init(options).unwrap();

    let mut view = View::new(&rustbox);
    let mut state = State::new(rustbox.width(), rustbox.height());

    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        state.buffer.load_file(path.clone());
        state.buffer.filepath = Some(path.clone());
    }

    rustbox.clear();
    rustbox.set_cursor(0, 0);
    view.render(&state);

    'running: loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(Key::Ctrl('c'))) => {
                break 'running;
            }
            Ok(rustbox::Event::KeyEvent(key)) => {
                let exit = state.handle_key(key);
                if exit { break 'running }
            }
            Ok(rustbox::Event::ResizeEvent(w, h)) => {
                state.width = w as usize;
                state.height = h as usize;
            }
            Err(e) => panic!("{}", e),
            _ => {}
        }

        view.render(&state);
    }
}
