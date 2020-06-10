use crossterm::event::{KeyCode,KeyEvent,KeyModifiers};
use crossterm::style::Color;
use crate::keys::{KeyMap,KeyMatch};
use crate::mode::{Mode,ModeType};
use crate::point::Direction::*;
use crate::state::Action;
use crate::state::Action::*;

pub struct NormalMode {
    keymap: KeyMap,
    operator_pending: String,
}

impl Mode for NormalMode {
    fn color(&self) -> Option<Color> { Some(Color::DarkBlue) }
    fn display(&self) -> &'static str { "Normal" }

    fn keys_pressed(&mut self, keys: &[KeyEvent]) -> Option<Action> {
        // Special case to allow zero as a binding while still having the repeat operations
        if keys[0].code == KeyCode::Char('0') && !self.operator_pending.is_empty() {
            self.operator_pending.push('0');
            return None;
        };

        match self.keymap.match_keys(keys) {
            KeyMatch::Action(action) => self.maybe_repeat(action),
            KeyMatch::Partial => Some(Action::PartialKey),
            KeyMatch::None => {
                match keys[0].code {
                    KeyCode::Char(c) if c.is_digit(10) => {
                        self.operator_pending.push(c);
                        None
                    }
                    _ => self.default_action(keys[0]),
                }
            }
        }
    }
}

impl NormalMode {
    pub fn new() -> NormalMode {
        let mut mode = NormalMode {
            keymap: KeyMap::new(),
            operator_pending: String::new(),
        };
        mode.bind_defaults();
        mode
    }

    fn bind_defaults(&mut self) {
        let ref mut km = self.keymap;
        km.bind_defaults();
        km.bind(&[KeyCode::Char('k').into()], MoveCursor(Up));
        km.bind(&[KeyCode::Char('j').into()], MoveCursor(Down));
        km.bind(&[KeyCode::Char('h').into()], MoveCursor(Left));
        km.bind(&[KeyCode::Char('l').into()], MoveCursor(Right));
        km.bind(&[KeyCode::Char('o').into()], NewLine);
        km.bind(&[KeyCode::Char('O').into()], Multi(vec!(MoveCursor(Up), NewLine)));
        km.bind(&[KeyCode::Char('i').into()], ChangeMode(ModeType::Insert));
        km.bind(&[KeyCode::Char('R').into()], ChangeMode(ModeType::Replace));
        km.bind(&[KeyCode::Char('x').into()], Delete);
        km.bind(&[KeyCode::Char('p').into()], Paste);
        km.bind(&[KeyCode::Char('.').into()], RepeatPrevious);
        km.bind(&[KeyCode::Char('0').into()], MoveCursor(BeginningOfLine));
        km.bind(&[KeyCode::Char('$').into()], MoveCursor(EndOfLine));
        km.bind(&[KeyCode::Char('G').into()], MoveCursor(EndOfFile));
        km.bind(&[KeyCode::Char('g').into(), KeyCode::Char('g').into()], MoveCursor(BeginningOfFile));
        km.bind(&[KeyCode::Char('y').into(), KeyCode::Char('y').into()], YankLine);
        km.bind(&[KeyCode::Char('d').into(), KeyCode::Char('d').into()], DeleteLine);

        km.bind(&[KeyCode::Char(' ').into(), KeyCode::Char('b').into(), KeyCode::Char('n').into()], NextBuffer);
        km.bind(&[KeyCode::Char(' ').into(), KeyCode::Char('b').into(), KeyCode::Char('p').into()], PrevBuffer);

        km.bind(&[KeyCode::Char('A').into()], Multi(vec!(
            MoveCursor(EndOfLine),
            ChangeMode(ModeType::Insert),
            MoveCursor(Right),
        )));

        km.bind(&[KeyCode::Char('I').into()], Multi(vec!(
            MoveCursor(BeginningOfLine),
            ChangeMode(ModeType::Insert),
        )));

        km.bind(&[KeyCode::Char('a').into()], Multi(vec!(
            MoveCursor(Right),
            ChangeMode(ModeType::Insert),
        )));
    }

    fn maybe_repeat(&mut self, action: Action) -> Option<Action> {
        if self.operator_pending.is_empty() {
            Some(action)
        } else {
            let n = i32::from_str_radix(self.operator_pending.as_ref(), 10).ok().unwrap_or(1);
            self.operator_pending = String::new();
            Some(Repeat(Box::new(action), n as usize))
        }
    }
}
//