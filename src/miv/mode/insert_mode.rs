extern crate rustbox;
use rustbox::Key;
use keys::{KeyMap,KeyMatch};
use mode::Mode;
use state::Action;
use state::Action::*;

pub struct InsertMode {
    keymap: KeyMap,
}

impl InsertMode {
    pub fn new() -> InsertMode {
        let mut mode = InsertMode {
            keymap: KeyMap::new(),
        };
        mode.bind_defaults();
        mode
    }

    fn bind_defaults(&mut self) {
        let ref mut km = self.keymap;
        km.bind_defaults();
        km.bind(&[Key::Char('d'), Key::Char('d')], DeleteLine);
        km.bind(&[Key::Backspace], BackwardDelete);
        km.bind(&[Key::Enter], NewLineAtPoint);
    }

    fn default_action(&self, key: Key) -> Option<Action> {
        if let Key::Char(c) = key {
            Some(Action::Insert(c))
        } else {
            None
        }
    }
}

impl Mode for InsertMode {
    fn color(&self) -> Option<u16> { Some(2) }
    fn display(&self) -> &'static str { "Insert" }

    fn keys_pressed(&mut self, keys: &[rustbox::Key]) -> Option<Action> {
        match self.keymap.match_keys(keys) {
            KeyMatch::Action(action) => Some(action),
            KeyMatch::Partial => Some(Action::PartialKey),
            KeyMatch::None => self.default_action(keys[0]),
        }
    }
}
