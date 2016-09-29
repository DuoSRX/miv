extern crate rustbox;

use rustbox::Key;
use state::Action;

pub use self::insert_mode::InsertMode;
pub use self::normal_mode::NormalMode;
pub use self::replace_mode::ReplaceMode;

mod insert_mode;
mod normal_mode;
mod replace_mode;

#[derive(Eq,PartialEq,Debug,Copy,Clone,Hash)]
pub enum ModeType {
    Insert,
    Normal,
    Replace
}

pub trait Mode {
    fn keys_pressed(&mut self, keys: &[rustbox::Key]) -> Option<Action>;
    fn default_action(&self, Key) -> Option<Action> { None }
    fn on_exit(&self) -> Option<Action> { None }
    fn color(&self) -> Option<u16> { None }
    fn display(&self) -> &'static str;
}
