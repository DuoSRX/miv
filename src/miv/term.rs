extern crate termion;

use termion::raw::IntoRawMode;
use std::io::{Write, Stdout, stdout};

pub struct Term {
    stdout: termion::raw::RawTerminal<Stdout>,
}

impl Term {
    pub fn new() -> Self {
        Self {
            stdout: stdout().into_raw_mode().unwrap(),
        }
    }

    pub fn clear(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub fn print(&mut self, x: u16, y: u16, s: String) {
        self.goto(x, y);
        self.stdout.write(s.as_bytes()).unwrap();
    }

    pub fn print_char(&mut self, x: u16, y: u16, c: char) {
        self.goto(x, y);
        write!(self.stdout, "{}", c).unwrap();
    }

    pub fn goto(&mut self, x: u16, y: u16) {
        // Termion Goto is 1-based because reasons
        write!(self.stdout, "{}", termion::cursor::Goto(x + 1, y + 1)).unwrap();
    }

    pub fn show_cursor(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
    }

}
