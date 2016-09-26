extern crate rustbox;

use rustbox::{Color, RustBox};

use keys::key_to_string;
use point::Point;
use state::State;

pub struct View<'a> {
    rustbox: &'a RustBox,
    top_line: usize,
}

impl<'a> View<'a> {
    pub fn new(rustbox: &'a RustBox) -> View {
        View {
            rustbox: rustbox,
            top_line: 0,
        }
    }

    pub fn relative_cursor(&self, cursor: Point) -> Point {
        Point { x: cursor.x, y: cursor.y - self.top_line }
    }

    pub fn render(&mut self, state: &State) {
        let mut x = 0;
        let mut y = 0;
        let height = state.height - 2; // Room for the status line

        if state.cursor.y < self.top_line {
            self.top_line = state.cursor.y;
        } else if state.cursor.y > height + self.top_line {
            self.top_line = state.cursor.y - height;
        }

        self.rustbox.clear();

        for line in state.buffer.data.iter().skip(self.top_line) {
            for &c in line.iter() {
                if c == '\n' { continue };
                self.print_at(Point::new(x, y), c);
                x += 1;
            }
            y += 1;
            x = 0;
        }

        let cursor = self.relative_cursor(state.cursor);
        self.rustbox.set_cursor(cursor.x as isize, cursor.y as isize);
        self.print_mode(state);
        self.print_status(state);
        self.rustbox.present();
    }

    pub fn print_at(&self, point: Point, character: char) {
        self.rustbox.print_char(point.x, point.y, rustbox::RB_NORMAL, Color::White, Color::Black, character);
    }

    pub fn print_mode(&self, state: &State) {
        let mode = format!("-- {} --", state.mode().display);
        let coords = format!("{}:{}", state.cursor.y + 1, state.cursor.x);

        if !state.keystrokes.is_empty() {
            let keys: String = state.keystrokes.iter()
                .filter_map(|&k| key_to_string(k).or(None))
                .collect();
            self.rustbox.print(18, self.rustbox.height() - 1, rustbox::RB_BOLD, Color::White, Color::Black, keys.as_ref());
        }

        self.rustbox.print(0, self.rustbox.height() - 1, rustbox::RB_BOLD, Color::White, Color::Black, mode.as_ref());
        self.rustbox.print(self.rustbox.width() - 2 - coords.len(), self.rustbox.height() - 1, rustbox::RB_BOLD, Color::White, Color::Black, coords.as_ref());
    }

    pub fn print_status(&self, state: &State) {
        if let Some(status) = state.status.clone() {
            self.rustbox.print(20, self.rustbox.height() - 1, rustbox::RB_BOLD, Color::White, Color::Black, status.as_ref());
        }
    }
}
