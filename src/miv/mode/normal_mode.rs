extern crate rustbox;
use rustbox::Key;
use keys::{KeyMap,KeyMatch};
use mode::{Mode,ModeType};
use point::Direction::*;
use state::Action;
use state::Action::*;

pub struct NormalMode {
    keymap: KeyMap,
}

impl NormalMode {
    pub fn new() -> NormalMode {
        let mut mode = NormalMode {
            keymap: KeyMap::new(),
        };
        mode.bind_defaults();
        mode
    }

    pub fn color(&self) -> u16 { 220 }

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
        km.bind(&[Key::Char('q')], Quit);
        km.bind(&[Key::Ctrl('c')], Quit);
        km.bind(&[Key::Ctrl('s')], Save);
        km.bind(&[Key::Char('d'), Key::Char('d')], DeleteLine);

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
}

impl Mode for NormalMode {
    fn keys_pressed(&self, keys: &[rustbox::Key]) -> Option<Action> {
        match self.keymap.match_keys(keys) {
            KeyMatch::Action(action) => Some(action),
            KeyMatch::Partial => Some(Action::PartialKey),
            KeyMatch::None => self.default_action(keys[0]),
        }
    }
}

// pub fn normal_mode() -> Mode {
//     fn default_f(key: Key) -> Option<Action> {
//         match key {
//             Key::Char(c) if c.is_digit(10) => {
//                 // whew lad... unsafe much?
//                 let n = i32::from_str_radix(c.to_string().as_ref(), 10).ok().unwrap();
//                 Some(OperatorPending(n as usize))
//             }
//             _ => None,
//         }
//     };
