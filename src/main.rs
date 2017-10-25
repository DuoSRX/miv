// extern crate rustbox;
extern crate termion;
extern crate miv;

use std::env;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};

use miv::state::State;
use miv::view::View;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::clear::All).unwrap();

    let mut view = View::new();

    let (width, height) = termion::terminal_size().unwrap();
    let mut state = State::new(width as usize, height as usize);

    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        let mut buffer = state.buffer.borrow_mut();
        buffer.load_file(path.clone());
        buffer.filepath = Some(path.clone());
    }

    view.render(&state);

    'running: for key in stdin.keys() {
        match key {
            //Key::Char('q') => break,
            Ok(Key::Ctrl('c')) => break,
            Ok(event) => {
                let exit = state.handle_key(event);
                if exit { break 'running }
            }
            Err(e) => panic!("{}", e),
        }

        view.render(&state);
    }

    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Show).unwrap();

    // 'running: loop {
    //     match rustbox.poll_event(false) {
    //         Ok(rustbox::Event::KeyEvent(Key::Ctrl('c'))) => {
    //             break 'running;
    //         }
    //         Ok(rustbox::Event::KeyEvent(key)) => {
    //             let exit = state.handle_key(key);
    //             if exit { break 'running }
    //         }
    //         Ok(rustbox::Event::ResizeEvent(w, h)) => {
    //             state.width = w as usize;
    //             state.height = h as usize;
    //         }
    //         Err(e) => panic!("{}", e),
    //         _ => {}
    //     }

    //     view.render(&state);
    // }
}
