use crossterm::event::{KeyCode,KeyEvent};
use crossterm::style::Color;
use crate::keys::{KeyMap,KeyMatch};
use crate::mode::Mode;
use crate::point::Direction::*;
use crate::state::Action;
use crate::state::Action::*;

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
        km.bind(&[KeyCode::Backspace.into()], MoveCursor(Left));
        km.bind(&[KeyCode::Enter.into()], NewLineAtPoint);
    }

    fn default_action(&self, key: KeyCode) -> Option<Action> {
        if let KeyCode::Char(c) = key {
            Some(Action::Replace(c))
        } else {
            None
        }
    }
}

impl Mode for ReplaceMode {
    fn color(&self) -> Option<Color> { Some(Color::Green) }
    fn display(&self) -> &'static str { "Replace" }

    fn keys_pressed(&mut self, keys: &[KeyEvent]) -> Option<Action> {
        match self.keymap.match_keys(keys) {
            KeyMatch::Action(action) => Some(action),
            KeyMatch::Partial => Some(Action::PartialKey),
            KeyMatch::None => self.default_action(keys[0].code),
        }
    }
}
