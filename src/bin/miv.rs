extern crate rustbox;
extern crate miv;

use std::env;
use std::io::Read;
use std::fs::File;
use rustbox::{Key,Color, RustBox};

use miv::mode::{Mode};
use miv::point::Point;
use miv::state::State;

fn buffer_from_file(path: String) -> Vec<Vec<char>> {
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    let _ = file.read_to_string(&mut s);
    let mut buf = Vec::new();

    for line in s.lines() {
        let mut l = Vec::with_capacity(line.len());
        for c in line.chars() {
            l.push(c);
        }
        buf.push(l)
    }

    buf
}

fn main() {
    let rustbox = RustBox::init(Default::default()).unwrap();

    let mut state = State::new(rustbox.width(), rustbox.height());

    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        state.buffer = buffer_from_file(path.clone());
        state.filepath = Some(path.clone());
    }

    rustbox.clear();
    rustbox.set_cursor(0, 0);
    render(&rustbox, &state);

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
            Err(e) => panic!("{}", e),
            _ => {}
        }

        render(&rustbox, &state);
    }
}

fn render(rustbox: &rustbox::RustBox, state: &State) {
    let mut x = 0;
    let mut y = 0;

    rustbox.clear();

    for line in state.buffer.iter() {
        for &c in line.iter() {
            if c == '\n' { continue };
            print_at(&rustbox, Point::new(x, y), c);
            x += 1;
        }
        y += 1;
        x = 0;
    }

    rustbox.set_cursor(state.cursor.x as isize, state.cursor.y as isize);
    print_mode(&rustbox, &state);
    rustbox.present();
}

fn print_at(rustbox: &rustbox::RustBox, point: Point, character: char) {
    rustbox.print_char(point.x, point.y, rustbox::RB_BOLD, Color::Yellow, Color::Black, character);
}

fn print_mode(rustbox: &rustbox::RustBox, state: &State) {
    let mode = match state.mode {
        Mode::Insert => "-- Insert Mode --",
        Mode::Command => "-- Command Mode --",
    };
    let coords = format!("{}:{}", state.cursor.y + 1, state.cursor.x);

    rustbox.print(0, rustbox.height() - 1, rustbox::RB_BOLD, Color::White, Color::Black, mode);
    rustbox.print(rustbox.width() - 2 - coords.len(), rustbox.height() - 1, rustbox::RB_BOLD, Color::White, Color::Black, coords.as_ref());
}
