extern crate rustbox;

use rustbox::{Color, RustBox};

use mode::Mode;
use point::Point;
use state::State;

pub struct View<'a> {
    rustbox: &'a RustBox
}

impl<'a> View<'a> {
    pub fn new(rustbox: &'a RustBox) -> View {
        View {
            rustbox: rustbox
        }
    }

    pub fn render(&self, state: &State) {
        let mut x = 0;
        let mut y = 0;

        self.rustbox.clear();

        for line in state.buffer.data.iter() {
            for &c in line.iter() {
                if c == '\n' { continue };
                self.print_at(Point::new(x, y), c);
                x += 1;
            }
            y += 1;
            x = 0;
        }

        self.rustbox.set_cursor(state.cursor.x as isize, state.cursor.y as isize);
        self.print_mode(&state);
        self.print_status(&state);
        self.rustbox.present();
    }


    pub fn print_at(&self, point: Point, character: char) {
        self.rustbox.print_char(point.x, point.y, rustbox::RB_BOLD, Color::Yellow, Color::Black, character);
    }

    pub fn print_mode(&self, state: &State) {
        let mode = match state.mode {
            Mode::Insert => "-- Insert Mode --",
            Mode::Normal => "-- Normal Mode --",
        };
        let coords = format!("{}:{}", state.cursor.y + 1, state.cursor.x);

        self.rustbox.print(0, self.rustbox.height() - 1, rustbox::RB_BOLD, Color::White, Color::Black, mode);
        self.rustbox.print(self.rustbox.width() - 2 - coords.len(), self.rustbox.height() - 1, rustbox::RB_BOLD, Color::White, Color::Black, coords.as_ref());
    }

    pub fn print_status(&self, state: &State) {
        if let Some(status) = state.status.clone() {
            self.rustbox.print(20, self.rustbox.height() - 1, rustbox::RB_BOLD, Color::White, Color::Black, status.as_ref());
        }
    }
}
