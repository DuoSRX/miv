extern crate rustbox;

use buffer::Buffer;
use mode::Mode;
use point::{Direction,Point};

#[derive(Eq,PartialEq,Debug,Clone)]
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

pub struct State {
    pub mode: Mode,
    pub cursor: Point,
    pub buffer: Buffer,
    pub width: usize,
    pub height: usize,
    pub status: Option<String>, // text to be displayed in the bottom bar
}

impl State {
    pub fn new(width: usize, height: usize) -> State {
        State {
            mode: Mode::Normal,
            cursor: Point::new(0, 0),
            buffer: Buffer::new(),
            width: width,
            height: height,
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
                self.buffer.split_line(self.cursor);
                self.cursor.y += 1;
                self.cursor.x = 0;
            }
            Action::NewLine => {
                self.buffer.new_line(self.cursor);
                self.cursor.y += 1;
                self.cursor.x = 0;
            }
            Action::Insert(c) => {
                self.buffer.insert(self.cursor, c);
                self.cursor.x += 1;
            }
            Action::Delete => {
                self.buffer.delete(self.cursor);
            }
            Action::BackwardDelete => {
                self.cursor.x -= 1;
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
}

