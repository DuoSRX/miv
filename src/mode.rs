extern crate rustbox;

use rustbox::Key;

use keys::{KeyMap,KeyMatch};
use state::Action;
use state::Action::*;
use point::Direction::{Up,Down,Left,Right};

#[derive(Eq,PartialEq,Debug,Copy,Clone)]
pub enum ModeType {
    Insert,
    Normal
}

pub struct Mode {
    keymap: KeyMap,
    default_action: fn(Key) -> Option<Action>,
}

impl Mode {
    pub fn key_pressed(&self, key: rustbox::Key) -> Option<Action> {
        match self.keymap.match_keys(&[key]) {
            KeyMatch::Action(action) => Some(action),
            KeyMatch::Partial => { panic!() },
            KeyMatch::None => (self.default_action)(key),
        }
    }

    pub fn insert_mode() -> Mode {
        fn f(key: Key) -> Option<Action> {
            if let Key::Char(c) = key {
                Some(Action::Insert(c))
            } else {
                None
            }
        }

        let mut km = KeyMap::new();
        km.bind(&[Key::Esc], ChangeMode(ModeType::Normal));
        km.bind(&[Key::Backspace], BackwardDelete);
        km.bind(&[Key::Enter], NewLineAtPoint);
        km.bind(&[Key::Up], MoveCursor(Up));
        km.bind(&[Key::Down], MoveCursor(Down));
        km.bind(&[Key::Left], MoveCursor(Left));
        km.bind(&[Key::Right], MoveCursor(Right));

        Mode {
            keymap: km,
            default_action: f,
        }
    }

    pub fn normal_mode() -> Mode {
        fn f(_: Key) -> Option<Action> { None };

        let mut km = KeyMap::new();
        km.bind(&[Key::Char('k')], MoveCursor(Up));
        km.bind(&[Key::Char('j')], MoveCursor(Down));
        km.bind(&[Key::Char('h')], MoveCursor(Left));
        km.bind(&[Key::Char('l')], MoveCursor(Right));
        km.bind(&[Key::Char('i')], ChangeMode(ModeType::Insert));
        km.bind(&[Key::Char('x')], Delete);
        km.bind(&[Key::Char('q')], Quit);
        km.bind(&[Key::Ctrl('c')], Quit);
        km.bind(&[Key::Ctrl('s')], Save);

        Mode {
            keymap: km,
            default_action: f,
        }
    }
}
