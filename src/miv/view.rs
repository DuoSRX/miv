use std::io::{Write, stdout};
pub use crossterm::{
  cursor::{SavePosition, RestorePosition, EnableBlinking, DisableBlinking},
  cursor,
  event::{self, Event, KeyCode, KeyEvent},
  execute, queue, style,
  terminal::{self, ClearType},
  QueueableCommand, Command, Result,
  style::{Colorize, Color, style}
};

use crate::keys::key_to_string;
// use crate::mode::ModeType;
use crate::point::Point;
use crate::state::{State,MicroState};

// const BG_COLOR: Color = Color::Byte(234);
// const FG_COLOR: Color = Color::Byte(0);
// const BAR_BG_COLOR: Color = Color::Byte(237);
// const BAR_FG_COLOR: Color = Color::Byte(233);
// const BAR_BG_MODE_COLOR: Color = Color::Byte(26);
const BAR_HEIGHT: usize = 2;
// const DEFAULT_MODE_COLOR: u16 = 220;

pub struct View {
    out: std::io::Stdout,
    /// Highest visible buffer line
    topline: usize,
    /// Leftmost visible buffer column
    leftcol: usize,
    /// Entire width (including bottom bar)
    width: usize,
    /// Entire height (including bottom bar)
    height: usize,
    /// Buffer window height
    window_height: usize,
    /// Buffer window width
    window_width: usize,
    /// Index of the last row before the bottom bar
    last_row: usize,
    /// The absolute cursor position (as oppose to the cursor position inside the buffer)
    cursor: Point,
}

/// The editor frontend.
///
/// Holds a reference to a `RustBox` instance created by the main app.
impl View {
    pub fn new() -> View {
        View {
            out: stdout(),
            topline: 0,
            leftcol: 0,
            width: 0,
            height: 0,
            window_height: 0,
            window_width: 0,
            last_row: 0,
            cursor: Point::new(0, 0),
        }
    }

    /// Renders the whole editor into the termbox: text, bottom bar, cursor... etc
    pub fn render(&mut self, state: &State) {
        self.height = state.height;
        self.width = state.width;
        self.window_height = state.height - BAR_HEIGHT;
        self.window_width = state.width - 1;
        self.last_row  = self.window_height - 1;

        if state.cursor.y < self.topline {
            self.topline = state.cursor.y;
        } else if state.cursor.y > self.last_row + self.topline {
            self.topline = state.cursor.y - self.last_row;
        }

        if state.cursor.x < self.leftcol {
            self.leftcol = state.cursor.x;
        } else if state.cursor.x > self.window_width + self.leftcol {
            self.leftcol = state.cursor.x - self.window_width
        }

        self.cursor = self.adjusted_cursor(state.cursor);

        self.out.queue(SavePosition).unwrap();
        self.out.queue(terminal::Clear(ClearType::All)).unwrap();
        self.fill_background(state);

        for (y, line) in state.buffer.borrow().data.iter().skip(self.topline).take(self.window_height).enumerate() {
            for (x, character) in line.chars().skip(self.leftcol).take(self.window_width + 1).enumerate() {
                if character == '\n' { continue };
                self.out.queue(cursor::MoveTo(x as u16, y as u16)).unwrap();
                self.out.queue(style::PrintStyledContent(character.to_string().white())).unwrap();
            }
        }

        self.print_bar(state);
        self.print_cursor(state);

        self.out.flush().unwrap();
    }

    /// The cursor passed by State is the absolute position in the text buffer.
    /// This adjusts the position by `topline` and `leftcol`. Used for scrolling.
    fn adjusted_cursor(&self, cursor: Point) -> Point {
        Point { x: cursor.x - self.leftcol, y: cursor.y - self.topline }
    }

    fn print_bar(&mut self, state: &State) {
        if !state.keystrokes.is_empty() {
            let keys: String = state.keystrokes.iter()
                .filter_map(|&k| key_to_string(k).or(None))
                .collect();
            self.out.queue(cursor::MoveTo(18, self.window_height as u16)).unwrap();
            self.out.queue(style::PrintStyledContent(keys.white().on_grey())).unwrap();
        }

        self.print_coords(state);
        self.print_status(state);
        self.print_mode(state);
    }

    fn print_mode(&mut self, state: &State) {
        let mode = format!(" {}  ", state.mode.display());
        let color = state.mode.color().unwrap_or(Color::DarkGrey);
        let styled = style(mode).with(Color::White).on(color);
        self.out.queue(cursor::MoveTo(0, self.window_height as u16)).unwrap();
        self.out.queue(style::PrintStyledContent(styled)).unwrap();
    }

    fn print_coords(&mut self, state: &State) {
        let coords = format!("  {}:{}  ", state.cursor.y + 1, state.cursor.x);
        let color = state.mode.color().unwrap_or(Color::DarkGrey);
        let styled = style(coords.clone()).with(Color::White).on(color);
        let x = self.window_width - coords.len();
        self.out.queue(cursor::MoveTo(x as u16, self.window_height as u16)).unwrap();
        self.out.queue(style::PrintStyledContent(styled)).unwrap();
    }

    fn print_cursor(&mut self, state: &State) {
        if state.microstate == MicroState::MiniBuffer {
            self.out.queue(cursor::MoveTo(state.minibuffer.len() as u16 + 1, self.height as u16)).unwrap();
        } else {
            let cursor = self.adjusted_cursor(state.cursor);
            self.out.queue(cursor::MoveTo(cursor.x as u16, cursor.y as u16)).unwrap();
        }
    }

    fn print_status(&self, _state: &State) {
        // if let Some(status) = state.status.clone() {
        //     self.rustbox.print(0, self.window_height + 1, rustbox::RB_BOLD, Color::White, BG_COLOR, status.as_ref());
        // }

        // if state.microstate == MicroState::MiniBuffer {
        //     self.rustbox.print(0, self.window_height + 1, rustbox::RB_BOLD, Color::White, BG_COLOR, ":");
        //     self.rustbox.print(1, self.window_height + 1, rustbox::RB_BOLD, Color::White, BG_COLOR, state.minibuffer.as_ref());
        // }
    }

    fn fill_background(&self, _state: &State) {
        // // Background
        // for y in 0..self.rustbox.height() {
        //     for x in 0..self.rustbox.width() {
        //         self.rustbox.print(x, y, rustbox::RB_NORMAL, Color::White, BG_COLOR, " ");
        //     }
        // }

        // // Info bar
        // let y = self.window_height;
        // let bg_color = self.bar_bg_color(state);
        // for x in 0..self.rustbox.width() {
        //     self.rustbox.print(x, y, rustbox::RB_NORMAL, Color::White, bg_color, " ");
        // }
    }

    // fn bar_bg_color(&self, state: &State) -> Color {
        // if state.mode_type == ModeType::Normal {
        //     BAR_BG_COLOR
        // } else {
        //     BAR_BG_MODE_COLOR
        // }
    // }
}
