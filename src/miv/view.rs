extern crate rustbox;

use rustbox::{Color, RustBox};

use keys::key_to_string;
use mode::ModeType;
use point::Point;
use state::State;

const BG_COLOR: Color = Color::Byte(234);
const FG_COLOR: Color = Color::Byte(0);
const BAR_BG_COLOR: Color = Color::Byte(237);
const BAR_FG_COLOR: Color = Color::Byte(233);
const BAR_BG_MODE_COLOR: Color = Color::Byte(26);

const BAR_HEIGHT: usize = 2;

pub struct View<'a> {
    rustbox: &'a RustBox,

    top_line: usize, // highest visible buffer line

    width: usize,  // entire width (including bottom bar)
    height: usize, // entire height (including bottom bar)

    window_height: usize, // buffer window height
    window_width: usize,  // buffer window width
}

impl<'a> View<'a> {
    pub fn new(rustbox: &'a RustBox) -> View {
        View {
            rustbox: rustbox,
            top_line: 0,
            width: 0,
            height: 0,
            window_height: 0,
            window_width: 0,
        }
    }

    pub fn render(&mut self, state: &State) {
        self.height = state.height - BAR_HEIGHT - 1; // Room for the status line
        self.width = state.width - 1;

        // Adjust `top_line` for scrolling vertically.
        if state.cursor.y < self.top_line {
            self.top_line = state.cursor.y;
        } else if state.cursor.y > self.height + self.top_line {
            self.top_line = state.cursor.y - self.height;
        }

        self.rustbox.clear();
        self.fill_background(state);

        for (y, line) in state.buffer.data.iter().skip(self.top_line).take(self.height + 1).enumerate() {
            for (x, &character) in line.iter().enumerate() {
                if character == '\n' { continue };
                self.rustbox.print_char(x, y, rustbox::RB_NORMAL, FG_COLOR, BG_COLOR, character);
            }
        }

        let cursor = self.adjusted_cursor(state.cursor);
        self.rustbox.set_cursor(cursor.x as isize, cursor.y as isize);
        self.print_bar(state);

        self.rustbox.present();
    }

    // The cursor passed by State is the absolute position in the text buffer.
    // This adjusts the position by `top_line`. Used for vertical scrolling.
    fn adjusted_cursor(&self, cursor: Point) -> Point {
        Point { x: cursor.x, y: cursor.y - self.top_line }
    }

    fn print_bar(&self, state: &State) {
        if !state.keystrokes.is_empty() {
            let keys: String = state.keystrokes.iter()
                .filter_map(|&k| key_to_string(k).or(None))
                .collect();
            self.rustbox.print(18, self.rustbox.height() - 2, rustbox::RB_BOLD, Color::White, BAR_BG_COLOR, keys.as_ref());
        }

        self.print_coords(state);
        self.print_status(state);
        self.print_mode(state);
    }

    fn print_mode(&self, state: &State) {
        let mode = format!(" {}  ", state.mode().display);
        self.rustbox.print(0, self.rustbox.height() - 2, rustbox::RB_BOLD, BAR_FG_COLOR, Color::Byte(state.mode().color), mode.as_ref());
    }

    fn print_coords(&self, state: &State) {
        let coords = format!("  {}:{}  ", state.cursor.y + 1, state.cursor.x);
        let color = Color::Byte(state.mode().color);
        self.rustbox.print(self.rustbox.width() - 1 - coords.len(), self.rustbox.height() - 2, rustbox::RB_BOLD, BAR_FG_COLOR, color, coords.as_ref());
    }

    fn print_status(&self, state: &State) {
        if let Some(status) = state.status.clone() {
            self.rustbox.print(0, self.rustbox.height() - 1, rustbox::RB_BOLD, Color::White, BG_COLOR, status.as_ref());
        }
    }

    fn fill_background(&self, state: &State) {
        // Background
        for y in 0..self.rustbox.height() {
            for x in 0..self.rustbox.width() {
                self.rustbox.print(x, y, rustbox::RB_NORMAL, Color::White, BG_COLOR, " ");
            }
        }

        // Info bar
        let y = self.rustbox.height() - 2;
        let bg_color = self.bar_bg_color(state);
        for x in 0..self.rustbox.width() {
            self.rustbox.print(x, y, rustbox::RB_NORMAL, Color::White, bg_color, " ");
        }
    }

    fn bar_bg_color(&self, state: &State) -> Color {
        if state.mode == ModeType::Normal {
            BAR_BG_COLOR
        } else {
            BAR_BG_MODE_COLOR
        }
    }
}
