extern crate rustbox;

use rustbox::Key;
use buffer::Buffer;
use mode::{Mode,ModeType};
use point::{Direction,Point};
use point::Direction::*;

#[derive(Eq,PartialEq,Debug,Clone)]
pub enum Action {
    BackwardDelete,
    ChangeMode(ModeType),
    Delete,
    DeleteLine,
    Insert(char),
    NewLine,
    NewLineAtPoint,
    MoveCursor(Direction),
    PartialKey,
    Save,
    Quit,
    // Multi(Vec<Action>),
}

pub struct State {
    pub mode: ModeType, // current mode
    pub cursor: Point,
    pub buffer: Buffer,
    pub width: usize,
    pub height: usize,
    pub status: Option<String>, // text to be displayed in the bottom bar

    insert_mode: Mode,
    normal_mode: Mode,
    keystrokes: Vec<Key>,
}

impl State {
    pub fn new(width: usize, height: usize) -> State {
        State {
            cursor: Point::new(0, 0),
            buffer: Buffer::new(),
            width: width,
            height: height,
            status: None,
            mode: ModeType::Normal,
            insert_mode: Mode::insert_mode(),
            normal_mode: Mode::normal_mode(),
            keystrokes: Vec::new(),
        }
    }

    fn mode(&self) -> &Mode {
        match self.mode {
            ModeType::Insert => &self.insert_mode,
            ModeType::Normal => &self.normal_mode,
        }
    }

    pub fn handle_key(&mut self, key: rustbox::Key) -> bool {
        self.keystrokes.push(key);

        match self.mode().keys_pressed(self.keystrokes.as_slice()) {
            Some(Action::PartialKey) => false,
            Some(action) => self.execute_action(action),
            None => { self.keystrokes = Vec::new(); false }
        }
    }

    fn execute_action(&mut self, action: Action) -> bool {
        match action {
            Action::NewLineAtPoint => {
                self.buffer.split_line(self.cursor);
                self.move_cursor(Down);
                self.move_cursor(BeginningOfLine);
            }
            Action::NewLine => {
                self.buffer.new_line(self.cursor);
                self.move_cursor(Down);
                self.move_cursor(BeginningOfLine);
            }
            Action::Insert(c) => {
                self.buffer.insert(self.cursor, c);
                self.move_cursor(Right);
            }
            Action::Delete => {
                self.buffer.delete(self.cursor);
            }
            Action::DeleteLine => {
                self.buffer.delete_line(self.cursor);
                self.move_cursor(BeginningOfLine);
            }
            Action::BackwardDelete => {
                self.move_cursor(Left);
                self.buffer.delete(self.cursor);
            }
            Action::MoveCursor(direction) => {
                self.move_cursor(direction);
            }
            Action::ChangeMode(mode) => self.mode = mode,
            Action::Save => {
                let bytes = self.buffer.save_file();
                if bytes > 0 {
                    let path = self.buffer.filepath.clone().unwrap(); // We know the filepath is set
                    let status = format!("Saved \"{}\" ({} bytes)", path, bytes);
                    self.status = Some(status);
                }
            }
            Action::Quit => { return true },
            _ => {},
            // Action::Multi(actions) => {
            //     for action in actions.iter() {
            //         self.execute_action(action.clone());
            //     }
            // }
        }
        self.keystrokes = Vec::new();
        false
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        let p = self.cursor.with_direction(direction);

        if p.x < self.width {
            self.cursor = p;
        }
    }
}
