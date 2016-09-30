extern crate rustbox;
use rustbox::Key;
use keys::{KeyMap,KeyMatch};
use mode::{Mode,ModeType};
use point::Direction::*;
use state::Action;
use state::Action::*;

pub struct NormalMode {
    keymap: KeyMap,
    operator_pending: String,
}

impl Mode for NormalMode {
    fn color(&self) -> Option<u16> { Some(220) }
    fn display(&self) -> &'static str { "Normal" }

    fn keys_pressed(&mut self, keys: &[rustbox::Key]) -> Option<Action> {
        // Special case to allow zero as a binding while still having the repeat operations
        if keys[0] == Key::Char('0') && !self.operator_pending.is_empty() {
            self.operator_pending.push('0');
            return None;
        };

        match self.keymap.match_keys(keys) {
            KeyMatch::Action(action) => self.maybe_repeat(action),
            KeyMatch::Partial => Some(Action::PartialKey),
            KeyMatch::None => {
                match keys[0] {
                    Key::Char(c) if c.is_digit(10) => {
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
        km.bind(&[Key::Char('k')], MoveCursor(Up));
        km.bind(&[Key::Char('j')], MoveCursor(Down));
        km.bind(&[Key::Char('h')], MoveCursor(Left));
        km.bind(&[Key::Char('l')], MoveCursor(Right));
        km.bind(&[Key::Char('o')], NewLine);
        km.bind(&[Key::Char('O')], Multi(vec!(MoveCursor(Up), NewLine)));
        km.bind(&[Key::Char('i')], ChangeMode(ModeType::Insert));
        km.bind(&[Key::Char('R')], ChangeMode(ModeType::Replace));
        km.bind(&[Key::Char('x')], Delete);
        km.bind(&[Key::Char('p')], Paste);
        km.bind(&[Key::Char('.')], RepeatPrevious);
        km.bind(&[Key::Char('0')], MoveCursor(BeginningOfLine));
        km.bind(&[Key::Char('$')], MoveCursor(EndOfLine));
        km.bind(&[Key::Char('G')], MoveCursor(EndOfFile));
        km.bind(&[Key::Char('g'), Key::Char('g')], MoveCursor(BeginningOfFile));
        km.bind(&[Key::Char('y'), Key::Char('y')], YankLine);
        km.bind(&[Key::Char('d'), Key::Char('d')], DeleteLine);

        km.bind(&[Key::Char(' '), Key::Char('b'), Key::Char('n')], NextBuffer);

        km.bind(&[Key::Char('A')], Multi(vec!(
            MoveCursor(EndOfLine),
            ChangeMode(ModeType::Insert),
            MoveCursor(Right),
        )));

        km.bind(&[Key::Char('I')], Multi(vec!(
            MoveCursor(BeginningOfLine),
            ChangeMode(ModeType::Insert),
        )));

        km.bind(&[Key::Char('a')], Multi(vec!(
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
