extern crate rustbox;

use rustbox::Key;

use state::Action;
use point::Direction::{Up,Down,Left,Right};

#[derive(PartialEq,Debug,Copy,Clone)]
pub enum Mode {
    Insert,
    Command
}

impl Mode {
    pub fn key_pressed(self, key: rustbox::Key) -> Option<Action> {
        match self {
            Mode::Insert => {
                match key {
                    Key::Esc  => Some(Action::ChangeMode(Mode::Command)),
                    Key::Backspace => Some(Action::BackwardDelete),
                    Key::Enter => Some(Action::NewLine),
                    Key::Char(c) => Some(Action::Insert(c)),
                    Key::Up => Some(Action::MoveCursor(Up)),
                    Key::Down => Some(Action::MoveCursor(Down)),
                    Key::Left => Some(Action::MoveCursor(Left)),
                    Key::Right => Some(Action::MoveCursor(Right)),
                    _ => None
                }
            },
            Mode::Command => {
                match key {
                    Key::Char('k') | Key::Up => Some(Action::MoveCursor(Up)),
                    Key::Char('j') | Key::Down => Some(Action::MoveCursor(Down)),
                    Key::Char('h') | Key::Left => Some(Action::MoveCursor(Left)),
                    Key::Char('l') | Key::Right => Some(Action::MoveCursor(Right)),
                    Key::Char('i') => Some(Action::ChangeMode(Mode::Insert)),
                    Key::Char('x') => Some(Action::Delete),
                    Key::Char('q') => Some(Action::Quit),

                    Key::Ctrl('s') => Some(Action::Save),
                    _ => None
                }
            }
        }
    }
}
