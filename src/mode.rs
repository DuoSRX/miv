extern crate rustbox;

use rustbox::Key;

use state::Action;
use point::Direction::{Up,Down,Left,Right};

#[derive(PartialEq,Debug,Copy,Clone)]
pub enum Mode {
    Insert,
    Normal
}

impl Mode {
    pub fn key_pressed(self, key: rustbox::Key) -> Option<Action> {
        use state::Action::*;

        match self {
            Mode::Insert => {
                match key {
                    Key::Esc  => Some(ChangeMode(Mode::Normal)),
                    Key::Backspace => Some(BackwardDelete),
                    Key::Enter => Some(NewLineAtPoint),
                    Key::Char(c) => Some(Insert(c)),
                    Key::Up => Some(MoveCursor(Up)),
                    Key::Down => Some(MoveCursor(Down)),
                    Key::Left => Some(MoveCursor(Left)),
                    Key::Right => Some(MoveCursor(Right)),
                    _ => None
                }
            },
            Mode::Normal => {
                match key {
                    Key::Char('k') | Key::Up => Some(MoveCursor(Up)),
                    Key::Char('j') | Key::Down => Some(MoveCursor(Down)),
                    Key::Char('h') | Key::Left => Some(MoveCursor(Left)),
                    Key::Char('l') | Key::Right => Some(MoveCursor(Right)),
                    Key::Char('i') => Some(ChangeMode(Mode::Insert)),
                    Key::Char('o') => Some(Multi(vec!(ChangeMode(Mode::Insert), NewLine))),
                    Key::Char('x') => Some(Delete),
                    Key::Char('q') => Some(Quit),
                    Key::Ctrl('c') => Some(Quit),
                    Key::Ctrl('s') => Some(Save),
                    _ => None
                }
            }
        }
    }
}
