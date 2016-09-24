extern crate rustbox;

use std::fs::File;
use std::io::Write;

use mode::Mode;
use point::{Direction,Point};

pub struct State {
    pub mode: Mode,
    pub cursor: Point,
    pub buffer: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
    pub filepath: Option<String>, // the path of the file we're editing
}

#[derive(PartialEq,Copy,Debug,Clone)]
pub enum Action {
    BackwardDelete,
    ChangeMode(Mode),
    Delete,
    Insert(char),
    NewLine,
    MoveCursor(Direction),
    Save,
    Quit,
}

impl State {
    pub fn new(width: usize, height: usize) -> State {
        State {
            mode: Mode::Command,
            cursor: Point::new(0, 0),
            buffer: vec!(Vec::with_capacity(120)),
            width: width,
            height: height,
            filepath: None,
        }
    }

    pub fn handle_key(&mut self, key: rustbox::Key) -> bool {
        if let Some(action) = self.mode.key_pressed(key) {
            match action {
                Action::NewLine => {
                    let mut buffer = self.buffer.clone();
                    let newline = buffer[self.cursor.y].split_off(self.cursor.x);
                    buffer.push(newline);
                    self.buffer = buffer;

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
                Action::Save => self.save_file(),
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

    pub fn save_file(&mut self) {
        if self.filepath.is_none() { return } // TODO: choose filepath

        let path = self.filepath.clone().unwrap();
        let mut f = File::create(path).unwrap();
        for line in self.buffer.iter() {
            for &c in line.iter() {
                let _ = f.write_all(&[c as u8]);
            }
            let _ = f.write_all(&['\n' as u8]);
        }
    }
}

