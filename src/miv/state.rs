use crossterm::event::{KeyCode,KeyEvent};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::usize;
use crate::buffer::Buffer;
use crate::mode::{Mode,ModeType,NormalMode,InsertMode};
use crate::point::{Direction,Point};
use crate::point::Direction::*;

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
    NewBuffer,
    NextBuffer,
    /// Used when in the middle of key sequence such as `yy`.
    /// See `keystrokes`.
    PartialKey,
    Paste,
    PrevBuffer,
    RepeatPrevious,
    Replace(char),
    Save,
    Quit,
    YankLine,
    /// Multiple actions in a row
    Multi(Vec<Action>),
    /// The same action `n` times
    Repeat(Box<Action>, usize)
}

#[derive(Eq,PartialEq,Debug,Clone)]
pub enum MicroState {
    /// The default microstate. Most events are delegated to the current mode.
    Mode,
    /// When entering data in the minibuffer.
    /// Keystrokes are stored in `minibuffer` until enter or esc is pressed.
    MiniBuffer
}

pub struct State<'a> {
    /// Position of the cursor in the buffer.
    /// This is *not* the cursor position on the screen.
    /// See `View` for more details on this.
    pub cursor: Point,
    /// Width of the whole window in the terminal.
    pub width: usize,
    /// Height of the whole window in the terminal.
    pub height: usize,
    /// Status message that will be displayed in the bottom bar.
    pub status: Option<String>,
    /// Recorded keystrokes. Used for compound actions like `dd`.
    pub keystrokes: Vec<KeyEvent>,
    /// Current mode type.
    pub mode_type: ModeType,
    /// Current mode.
    pub mode: Box<dyn Mode + 'a>,
    /// The content of the minibuffer. Empty string if none.
    pub minibuffer: String,
    /// Used for instance when entering data in the minibuffer.
    pub microstate: MicroState,

    /// Vector of all the open buffers.
    pub buffers: Vec<Rc<RefCell<Buffer>>>,
    /// Reference to the current active buffer.
    pub buffer: Rc<RefCell<Buffer>>,

    // used to cycle through the buffers
    buffer_idx: usize,
    yanked: VecDeque<String>,
    previous_action: Option<Action>,
}

impl<'a> State<'a> {
    pub fn new(width: usize, height: usize) -> State<'a> {
        let buffer = Buffer::new();
        let mut buffers = Vec::new();
        buffers.push(Rc::new(RefCell::new(buffer)));

        State {
            cursor: Point::new(0, 0),
            buffer: buffers[0].clone(),
            buffers: buffers,
            buffer_idx: 0,
            width: width,
            height: height,
            status: None,
            mode_type: ModeType::Normal,
            mode: Box::new(NormalMode::new()),
            keystrokes: Vec::new(),
            minibuffer: String::new(),
            microstate: MicroState::Mode,
            yanked: VecDeque::new(),
            previous_action: None,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        self.status = None;

        if self.microstate == MicroState::MiniBuffer {
            return self.handle_minibuffer_input(key);
        }

        match key.code {
            KeyCode::Char(':') if self.microstate == MicroState::Mode => {
                self.microstate = MicroState::MiniBuffer;
                false
            }
            _ => {
                self.keystrokes.push(key);
                match self.mode.keys_pressed(self.keystrokes.as_slice()) {
                    Some(Action::PartialKey) => false,
                    Some(action) => self.execute_action(action),
                    None => { self.keystrokes = Vec::new(); false }
                }
            }
        }

    }

    fn execute_action(&mut self, action: Action) -> bool {
        match action {
            Action::RepeatPrevious => {
                if let Some(action) = self.previous_action.clone() {
                    self.execute_action(action);
                }
            }
            Action::NewLineAtPoint => {
                self.buffer.borrow_mut().split_line(self.cursor);
                self.move_cursor(Down);
                self.move_cursor(BeginningOfLine);
            }
            Action::NewLine => {
                self.buffer.borrow_mut().new_line(self.cursor);
                self.move_cursor(Down);
                self.move_cursor(BeginningOfLine);
                self.switch_mode(ModeType::Insert);
            }
            Action::Insert(c) => {
                self.buffer.borrow_mut().insert(self.cursor, c);
                self.move_cursor(Right);
            }
            Action::Replace(c) => {
                self.buffer.borrow_mut().upsert(self.cursor, c);
                self.move_cursor(Right);
            }
            Action::Delete => {
                let character = self.buffer.borrow_mut().delete(self.cursor);
                self.yanked.push_front(character.to_string());
            }
            Action::DeleteLine => {
                let line = self.buffer.borrow_mut().delete_line(self.cursor);
                self.yanked.push_front(line);
                self.move_cursor(BeginningOfLine);
            }
            Action::BackwardDelete => {
                self.move_cursor(Left);
                self.buffer.borrow_mut().delete(self.cursor);
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
            Action::ChangeMode(mode_type) => {
                self.switch_mode(mode_type);
            }
            Action::Cancel => {
                self.keystrokes = Vec::new();
                self.execute_action(Action::ChangeMode(ModeType::Normal));
            }
            Action::Save => {
                let bytes = self.buffer.borrow_mut().save_file();
                if bytes > 0 {
                    let path = self.buffer.borrow_mut().filepath.clone().unwrap(); // We know the filepath is set
                    let status = format!("Saved \"{}\" ({} bytes)", path, bytes);
                    self.status = Some(status);
                }
            }
            Action::YankLine => {
                if let Some(line) = self.buffer.borrow_mut().line_at(self.cursor.y) {
                    self.yanked.push_front(line.clone());
                }
            }
            Action::NewBuffer => {
                let buffer = Buffer::new();
                self.buffer = Rc::new(RefCell::new(buffer));
                self.buffers.push(self.buffer.clone());
                self.buffer_idx += 1;
            }
            Action::NextBuffer => {
                self.buffer_idx = (self.buffer_idx + 1) % self.buffers.len();
                self.buffer = self.buffers[self.buffer_idx].clone();
            }
            Action::PrevBuffer => {
                self.buffer_idx = (self.buffer_idx.wrapping_sub(1)) % self.buffers.len();
                self.buffer = self.buffers[self.buffer_idx].clone();
            }
            Action::Multi(ref actions) => {
                let mut result = false;
                for action in actions { result = self.execute_action(action.clone()); }
                return result
            }
            Action::Repeat(ref action, times) => {
                for _ in 0..times { self.execute_action(*action.clone()); }
            }
            Action::Quit => { return true },
            _ => {},
        }

        if action != Action::RepeatPrevious && action != Action::Cancel {
            self.previous_action = Some(action);
        }

        self.keystrokes = Vec::new();
        false
    }

    fn switch_mode(&mut self, mode_type: ModeType) {
        if let Some(action) = self.mode.on_exit() {
            self.execute_action(action);
        }

        self.mode_type = mode_type;
        self.mode = match mode_type {
            ModeType::Insert =>  Box::new(InsertMode::new()) as Box<dyn Mode>,
            ModeType::Normal =>  Box::new(NormalMode::new()) as Box<dyn Mode>,
            // ModeType::Replace => Box::new(ReplaceMode::new()) as Box<Mode>,
        };
    }

    fn move_cursor(&mut self, direction: Direction) {
        let mut cur = self.cursor.with_direction(direction);

        match direction {
            EndOfLine => { cur.x = usize::max_value() } // This is so ugly...
            EndOfFile => { cur.y = self.buffer.borrow_mut().line_len() - 1 }
            _ => {}
        }

        let max_x = self.buffer.borrow_mut().last_non_empty_col(cur);
        let max_y = self.buffer.borrow_mut().line_len() - 1;
        cur.clamp_by(max_x, max_y);

        self.cursor = cur;
    }

    fn paste(&mut self) {
        let mut yanked = self.yanked.front().unwrap().clone();

        // Pasting a new line
        if let Some(_) = yanked.rfind('\n') {
            self.buffer.borrow_mut().new_line(self.cursor);
            self.move_cursor(Down);
            self.move_cursor(BeginningOfLine);
            yanked.pop(); // remove the last \n
        } else {
            self.move_cursor(Right);
        }

        self.buffer.borrow_mut().insert_text(self.cursor, yanked);
    }

    fn handle_minibuffer_input(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char(c) => {
                self.minibuffer.push(c);
            }
            KeyCode::Backspace => {
                self.minibuffer.pop();
            }
            KeyCode::Enter => {
                return self.handle_minibuffer_command()
            }
            KeyCode::Esc => {
                self.microstate = MicroState::Mode;
                self.minibuffer = String::new();
            }
            _ => {}
        }
        false
    }

    // TODO: Extract this. I think this deserves its own module.
    fn handle_minibuffer_command(&mut self) -> bool {
        let result = match self.minibuffer.clone().split_whitespace().collect::<Vec<&str>>().as_slice() {
            &["w"] => self.execute_action(Action::Save),
            &["q"] => self.execute_action(Action::Quit),
            &["wq"] => self.execute_action(Action::Multi(vec!(Action::Save, Action::Quit))),
            &["new"] => self.execute_action(Action::NewBuffer),
            &["bn"] => self.execute_action(Action::NextBuffer),
            &["bp"] => self.execute_action(Action::PrevBuffer),
            &["e", path] => {
                self.execute_action(Action::NewBuffer);
                self.buffer.borrow_mut().load_file(path.into());
                false
            }
            _ => {
                self.status = Some(format!("Not a valid command: {}", self.minibuffer));
                false
            }
        };

        self.microstate = MicroState::Mode;
        self.minibuffer = String::new();
        result
    }
}
