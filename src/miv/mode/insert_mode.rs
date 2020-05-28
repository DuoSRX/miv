use crossterm::event::{KeyEvent,KeyCode};
use crate::keys::{KeyMap,KeyMatch};
use crate::mode::Mode;
use crate::state::Action;
use crate::state::Action::*;

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

        km.bind(&[KeyCode::Char('d').into()], DeleteLine);
        km.bind(&[KeyCode::Backspace.into()], BackwardDelete);
        km.bind(&[KeyCode::Enter.into()], NewLineAtPoint);
    }

    fn default_action(&self, key: KeyCode) -> Option<Action> {
        if let KeyCode::Char(c) = key {
            Some(Action::Insert(c))
        } else {
            None
        }
    }
}

impl Mode for InsertMode {
    fn color(&self) -> Option<u16> { Some(2) }
    fn display(&self) -> &'static str { "Insert" }

    fn keys_pressed(&mut self, keys: &[KeyEvent]) -> Option<Action> {
        match self.keymap.match_keys(keys) {
            KeyMatch::Action(action) => Some(action),
            KeyMatch::Partial => Some(Action::PartialKey),
            KeyMatch::None => self.default_action(keys[0].code),
        }
    }
}
