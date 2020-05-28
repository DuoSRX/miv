use crate::state::Action;

use crossterm::event::KeyEvent;
use crossterm::style::Color;

pub use self::insert_mode::InsertMode;
pub use self::normal_mode::NormalMode;
// pub use self::replace_mode::ReplaceMode;

mod insert_mode;
mod normal_mode;
mod replace_mode;

#[derive(Eq,PartialEq,Debug,Copy,Clone,Hash)]
pub enum ModeType {
    Insert,
    Normal,
    // Replace
}

pub trait Mode {
    /// The meat of the mode. Defines how to react to key presses.
    fn keys_pressed(&mut self, keys: &[KeyEvent]) -> Option<Action>;
    /// Default action in case nothing matches in the mode keymap.
    fn default_action(&self, _key: KeyEvent) -> Option<Action> { None }
    /// Action to run when the mode is replace by another.
    fn on_exit(&self) -> Option<Action> { None }
    /// Color to use for the bottom bar.
    fn color(&self) -> Option<Color> { None }
    /// The name of the mode. Displayed in the bottom bar.
    fn display(&self) -> &'static str;
}
