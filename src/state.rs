extern crate rustbox;

use mode::Mode;
use point::{Direction,Point};

pub struct State {
    pub mode: Mode,
    pub cursor: Point,
    pub buffer: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize
}

#[derive(PartialEq,Copy,Debug,Clone)]
pub enum Action {
    Insert(char),
    BackwardDelete,
    Delete,
    NewLine,
    MoveCursor(Direction),
    ChangeMode(Mode),
    Quit,
}

impl State {
    pub fn new(width: usize, height: usize) -> State {
        State {
            mode: Mode::Command,
            cursor: Point::new(0, 0),
            buffer: vec!(Vec::with_capacity(10)),
            width: width,
            height: height,
        }
    }

    pub fn handle_key(&mut self, key: rustbox::Key) -> bool {
        if let Some(action) = self.mode.key_pressed(key) {
            match action {
                Action::NewLine => {
                    self.buffer.push(Vec::new());
                    self.cursor.x = 0;
                    self.cursor.y += 1;
                },
                Action::Insert(c) => {
                    let ref mut line = self.buffer[self.cursor.y];
                    self.cursor.x += 1;

                    if self.cursor.x > line.len() {
                        line.push(c);
                    } else {
                        line.insert(self.cursor.x, c);
                    }
                }
                Action::Delete => {
                    self.buffer[self.cursor.y].remove(self.cursor.x);
                }
                Action::BackwardDelete => {
                    self.cursor.x -= 1;
                    self.buffer[self.cursor.y].remove(self.cursor.x);
                }
                Action::MoveCursor(direction) => {
                    self.move_cursor(direction);
                }
                Action::ChangeMode(mode) => self.mode = mode,
                Action::Quit => { return true }
            }
        }

        false
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        let p = self.cursor.with_direction(direction);

        if p.x < self.width && p.y < self.height {
            self.cursor = p;
        }
    }
}

