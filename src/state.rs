extern crate rustbox;

use std::collections::{HashMap,VecDeque};
use std::usize;
use rustbox::Key;
use buffer::Buffer;
use mode::{Mode,ModeType};
use point::{Direction,Point};
use point::Direction::*;

#[derive(Eq,PartialEq,Debug,Clone)]
pub enum Action {
    BackwardDelete,
    Cancel,
    ChangeMode(ModeType),
    Delete,
    DeleteLine,
    Insert(char),
    NewLine,
    NewLineAtPoint,
    MoveCursor(Direction),
    PartialKey,
    Paste,
    Repeat,
    Save,
    Quit,
    YankLine,
}

pub struct State {
    pub cursor: Point,
    pub buffer: Buffer,
    pub width: usize,
    pub height: usize,
    pub status: Option<String>, // text to be displayed in the bottom bar
    pub keystrokes: Vec<Key>,
    pub mode: ModeType, // current mode

    yanked: VecDeque<Vec<char>>,
    modes: HashMap<ModeType, Mode>, // available modes
    previous_action: Option<Action>,
    last_col: usize,
}

impl State {
    pub fn new(width: usize, height: usize) -> State {
        let mut modes = HashMap::new();
        modes.insert(ModeType::Insert, Mode::insert_mode());
        modes.insert(ModeType::Normal, Mode::normal_mode());

        State {
            cursor: Point::new(0, 0),
            buffer: Buffer::new(),
            width: width,
            height: height,
            status: None,
            modes: modes,
            mode: ModeType::Normal,
            keystrokes: Vec::new(),
            yanked: VecDeque::new(),
            previous_action: None,
            last_col: 0,
        }
    }

    pub fn mode(&self) -> &Mode {
        self.modes.get(&self.mode).expect(format!("Unknown mode {:?}", self.mode).as_ref())
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
            Action::Repeat => {
                if let Some(action) = self.previous_action.clone() {
                    self.execute_action(action);
                }
            }
            Action::NewLineAtPoint => {
                self.buffer.split_line(self.cursor);
                self.move_cursor(Down);
                self.move_cursor(BeginningOfLine);
            }
            Action::NewLine => {
                self.buffer.new_line(self.cursor);
                self.move_cursor(Down);
                self.move_cursor(BeginningOfLine);
                self.mode = ModeType::Insert;
            }
            Action::Insert(c) => {
                self.buffer.insert(self.cursor, c);
                self.move_cursor(Right);
            }
            Action::Delete => {
                let character = self.buffer.delete(self.cursor);
                self.yanked.push_front(vec!(character));
            }
            Action::DeleteLine => {
                let line = self.buffer.delete_line(self.cursor);
                self.yanked.push_front(line);
                self.move_cursor(BeginningOfLine);
            }
            Action::BackwardDelete => {
                self.move_cursor(Left);
                self.buffer.delete(self.cursor);
            }
            Action::Paste => {
                if self.yanked.is_empty() {
                    self.status = Some("Nothing to paste!".to_string());
                } else {
                    self.paste();
                }
            }
            Action::MoveCursor(direction) => {
                self.move_cursor(direction);
            }
            Action::ChangeMode(mode) => self.mode = mode,
            Action::Cancel => {
                self.mode = ModeType::Normal;
                self.keystrokes = Vec::new();
            }
            Action::Save => {
                let bytes = self.buffer.save_file();
                if bytes > 0 {
                    let path = self.buffer.filepath.clone().unwrap(); // We know the filepath is set
                    let status = format!("Saved \"{}\" ({} bytes)", path, bytes);
                    self.status = Some(status);
                }
            }
            Action::YankLine => {
                if let Some(line) = self.buffer.line_at(self.cursor.y) {
                    self.yanked.push_front(line.clone());
                }
            }
            Action::Quit => { return true },
            _ => {},
        }

        if action != Action::Repeat && action != Action::Cancel {
            self.previous_action = Some(action);
        }

        self.keystrokes = Vec::new();
        false
    }

    pub fn move_cursor(&mut self, direction: Direction) {
        let mut cur = self.cursor.with_direction(direction);

        match direction {
            EndOfLine => { cur.x = usize::max_value() } // This is so ugly...
            EndOfFile => { cur.y = self.buffer.line_len() - 1 }
            _ => {}
        }

        cur.x = self.buffer.last_non_empty_col(cur);

        self.cursor = cur;
        self.last_col = cur.y;
    }

    fn paste(&mut self) {
        let mut yanked = self.yanked.front().unwrap().clone();

        if yanked.iter().any(|&c| c == '\n') {
            self.buffer.new_line(self.cursor);
            self.move_cursor(Down);
            self.move_cursor(BeginningOfLine);
            // Remove the \n
            let last = yanked.len() - 1;
            yanked.remove(last);
        } else {
            self.move_cursor(Right);
        }

        self.buffer.insert_text(self.cursor, yanked);
    }
}
