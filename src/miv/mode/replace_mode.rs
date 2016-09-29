extern crate rustbox;
use rustbox::Key;
use keys::{KeyMap,KeyMatch};
use mode::Mode;
use point::Direction::*;
use state::Action;
use state::Action::*;

pub struct ReplaceMode {
    keymap: KeyMap,
}

impl ReplaceMode {
    pub fn new() -> ReplaceMode {
        let mut mode = ReplaceMode {
            keymap: KeyMap::new(),
        };
        mode.bind_defaults();
        mode
    }

    fn bind_defaults(&mut self) {
        let ref mut km = self.keymap;
        km.bind_defaults();
        km.bind(&[Key::Backspace], MoveCursor(Left));
        km.bind(&[Key::Enter], NewLineAtPoint);
    }

    fn default_action(&self, key: Key) -> Option<Action> {
        if let Key::Char(c) = key {
            Some(Action::Replace(c))
        } else {
            None
        }
    }
}

impl Mode for ReplaceMode {
    fn color(&self) -> Option<u16> { Some(160) }
    fn display(&self) -> &'static str { "Replace" }

    fn keys_pressed(&mut self, keys: &[rustbox::Key]) -> Option<Action> {
        match self.keymap.match_keys(keys) {
            KeyMatch::Action(action) => Some(action),
            KeyMatch::Partial => Some(Action::PartialKey),
            KeyMatch::None => self.default_action(keys[0]),
        }
    }
}
