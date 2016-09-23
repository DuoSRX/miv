extern crate rustbox;

use rustbox::{Color, RustBox};
use rustbox::Key;


#[derive(PartialEq,Debug,Copy,Clone)]
struct Point { x: i16, y: i16 }

impl Point {
    pub fn new(x: i16, y: i16) -> Point {
        Point { x: x, y: y }
    }

    pub fn add(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(PartialEq,Debug,Copy,Clone)]
enum Mode {
    Insert,
    Command
}

struct State {
    mode: Mode,
    cursor: Point,
}

#[derive(PartialEq,Debug,Copy,Clone)]
enum Action {
    Insert(char),
    MoveCursor(Point),
    ChangeMode(Mode),
    Quit,
}

impl Mode {
    pub fn key_pressed(self, key: rustbox::Key) -> Option<Action> {
        match self {
            Mode::Insert => {
                match key {
                    Key::Esc  => Some(Action::ChangeMode(Mode::Command)),
                    Key::Char(c) => Some(Action::Insert(c)),
                    _ => None
                }
            },
            Mode::Command => {
                match key {
                    Key::Char('k') | Key::Up => Some(Action::MoveCursor(Point::new(0, -1))),
                    Key::Char('j') | Key::Down => Some(Action::MoveCursor(Point::new(0, 1))),
                    Key::Char('h') | Key::Left => Some(Action::MoveCursor(Point::new(-1, 0))),
                    Key::Char('l') | Key::Right => Some(Action::MoveCursor(Point::new(1, 0))),
                    Key::Char('i') => Some(Action::ChangeMode(Mode::Insert)),
                    Key::Char('q') => Some(Action::Quit),
                    _ => None
                }
            }
        }
    }
}

fn main() {
    let rustbox = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut state = State {
        mode: Mode::Command,
        cursor: Point::new(10, 10)
    };

    rustbox.clear();
    rustbox.set_cursor(state.cursor.x as isize, state.cursor.y as isize);
    print_mode(&rustbox, &state);
    rustbox.present();

    loop {
        match rustbox.poll_event(false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    key => {
                        if let Some(action) = state.mode.key_pressed(key) {
                            match action {
                                Action::Insert(c) => {
                                    print_at(&rustbox, state.cursor, c);
                                    state.cursor.x += 1; // TODO: carriage return
                                }
                                Action::MoveCursor(point) => state.cursor.add(point),
                                Action::ChangeMode(mode) => state.mode = mode,
                                Action::Quit => break,
                            }
                        }
                    }
                }
            },
            Err(e) => panic!("{}", e),
            _ => {}
        }

        rustbox.set_cursor(state.cursor.x as isize, state.cursor.y as isize);
        print_mode(&rustbox, &state);
        rustbox.present();
    }
}

fn print_at(rustbox: &rustbox::RustBox, point: Point, character: char) {
    rustbox.print_char(point.x as usize, point.y as usize, rustbox::RB_BOLD, Color::Yellow, Color::Black, character);
}

fn print_mode(rustbox: &rustbox::RustBox, state: &State) {
    let mode = match state.mode {
        Mode::Insert => "-- Insert Mode --",
        Mode::Command => "-- Command Mode --",
    };

    rustbox.print(0, rustbox.height() - 1, rustbox::RB_BOLD, Color::White, Color::Black, mode);
}
