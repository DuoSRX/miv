extern crate rustbox;

use std::io::Write;
use std::fs::{File,OpenOptions};

use mode::Mode;
use point::{Direction,Point};

pub struct State {
    pub mode: Mode,
    pub cursor: Point,
    pub buffer: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
    pub filepath: Option<String>, // the path of the file we're editing
    pub status: Option<String>, // text to be displayed in the bottom bar
}

#[derive(PartialEq,Debug,Clone)]
pub enum Action {
    BackwardDelete,
    ChangeMode(Mode),
    Delete,
    Insert(char),
    NewLine,
    NewLineAtPoint,
    MoveCursor(Direction),
    Save,
    Quit,
    Multi(Vec<Action>),
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
            status: None,
        }
    }

    pub fn handle_key(&mut self, key: rustbox::Key) -> bool {
        if let Some(action) = self.mode.key_pressed(key) {
            self.execute_action(action)
        } else {
            false
        }
    }

    fn execute_action(&mut self, action: Action) -> bool {
        match action {
            Action::NewLineAtPoint => {
                let mut buffer = self.buffer.clone();
                let newline = buffer[self.cursor.y].split_off(self.cursor.x);
                self.cursor.y += 1;
                buffer.insert(self.cursor.y, newline);
                self.buffer = buffer;

                self.cursor.x = 0;
            }
            Action::NewLine => {
                self.cursor.y += 1;
                self.cursor.x = 0;
                self.buffer.insert(self.cursor.y, Vec::new());
            }
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
            Action::Quit => { return true },
            Action::Multi(actions) => {
                for action in actions.iter() {
                    self.execute_action(action.clone());
                }
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
        let mut file = OpenOptions::new().read(true).write(true).create(true).open(path).unwrap();
        //let mut f = File::create(path).unwrap();
        for line in self.buffer.iter() {
            for &c in line.iter() {
                let _ = file.write_all(&[c as u8]);
            }
            let _ = file.write_all(&['\n' as u8]);
        }

        let status = format!("Saved \"{}\" ({} bytes)",
                             self.filepath.clone().unwrap(),
                             file.metadata().unwrap().len());
        self.status = Some(status);
    }
}

