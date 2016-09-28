extern crate rustbox;

use rustbox::Key;

use keys::{KeyMap,KeyMatch};
use state::Action;
use state::Action::*;
use point::Direction::*;

#[derive(Eq,PartialEq,Debug,Copy,Clone,Hash)]
pub enum ModeType {
    Insert,
    Normal,
    Replace
}

pub struct Mode {
    keymap: KeyMap,
    default_action: fn(Key) -> Option<Action>,
    pub on_exit: fn() -> Option<Action>,
    pub color: u16,
    pub display: &'static str,
}

impl Mode {
    pub fn keys_pressed(&self, keys: &[rustbox::Key]) -> Option<Action> {
        match self.keymap.match_keys(keys) {
            KeyMatch::Action(action) => Some(action),
            KeyMatch::Partial => Some(Action::PartialKey),
            KeyMatch::None => (self.default_action)(keys[0]),
        }
    }

    pub fn on_exit(&self) -> Option<Action> {
        (self.on_exit)()
    }

    pub fn normal_mode() -> Mode {
        fn default_f(_key: Key) -> Option<Action> { None };
        fn on_exit() -> Option<Action> { None };

        let mut km = KeyMap::new();
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
        km.bind(&[Key::Char('.')], Repeat);
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

        Mode {
            keymap: km,
            default_action: default_f,
            on_exit: on_exit,
            display: "NORMAL",
            color: 220,
        }
    }

    pub fn insert_mode() -> Mode {
        fn default_f(key: Key) -> Option<Action> {
            if let Key::Char(c) = key {
                Some(Action::Insert(c))
            } else {
                None
            }
        }
        fn on_exit() -> Option<Action> { Some(MoveCursor(Left)) };

        let mut km = KeyMap::new();
        km.bind_defaults();
        km.bind(&[Key::Backspace], BackwardDelete);
        km.bind(&[Key::Enter], NewLineAtPoint);

        Mode {
            keymap: km,
            default_action: default_f,
            on_exit: on_exit,
            display: "INSERT",
            color: 2,
        }
    }

    pub fn replace_mode() -> Mode {
        fn default_f(key: Key) -> Option<Action> {
            if let Key::Char(c) = key {
                Some(Action::Replace(c))
            } else {
                None
            }
        }
        fn on_exit() -> Option<Action> { None };

        let mut km = KeyMap::new();
        km.bind_defaults();
        km.bind(&[Key::Backspace], MoveCursor(Left));
        km.bind(&[Key::Enter], NewLineAtPoint);

        Mode {
            keymap: km,
            default_action: default_f,
            on_exit: on_exit,
            display: "REPLACE",
            color: 160,
        }
    }
}
