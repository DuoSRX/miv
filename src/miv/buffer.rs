use std::io::{Read, Write};
use std::fs::OpenOptions;
use std::cmp;

use point::Point;

/// Holds the text data and the filepath, if any.
/// This buffer has no knowledge of the editor.
pub struct Buffer {
    /// Holds the actual data.
    /// This is a *very* inefficient data structure and will eventually be replace by a GapBuffer or Rope.
    /// This will also end up private so don't really too much on it.
    pub data: Vec<String>,

    /// The file path associated with that buffer.
    pub filepath: Option<String>,
}

impl Buffer {
    /// Make a new empty `Buffer`.
    pub fn new() -> Buffer {
        Buffer {
            data: vec!("\n".into()),
            filepath: None,
        }
    }

    /// Number of line in the buffer. Returns 1 if the buffer is empty.
    pub fn line_len(&self) -> usize {
        cmp::max(1, self.data.len())
    }

    /// Find the line at the given row.
    pub fn line_at(&mut self, y: usize) -> Option<&String> {
        self.data.get(y)
    }

    /// Find the last column that is non empty.
    /// Returns 0 if the whole line is empty.
    pub fn last_non_empty_col(&mut self, location: Point) -> usize {
        let x = self.line_at(location.y)
            .and_then(|line| match line.chars().nth(location.x) {
                Some(_) => Some(location.x as isize),
                None => Some(line.len() as isize - 2),
            }).unwrap();

        cmp::max(0, x) as usize
    }

    /// Find a char at a specific column/row.
    pub fn char_at(&mut self, location: Point) -> Option<char> {
        self.line_at(location.y).and_then(|line| line.chars().nth(location.x))
    }

    /// Insert a row at the specified point, pushing the other characters to the right.
    pub fn insert(&mut self, location: Point, c: char) {
        self.data[location.y].insert(location.x, c);
    }

    /// Replace a specific character.
    pub fn replace(&mut self, location: Point, c: char) {
        self.insert(location, c);
        self.delete(location.offset(1, 0));
    }

    /// Replace a character at the given location or insert it.
    pub fn upsert(&mut self, location: Point, c: char) {
        if let Some('\n') = self.char_at(location) {
            self.insert(location, c);
        } else {
            self.replace(location, c);
        }
    }

    /// Delete a character at the given location, shifting the rest of the line to the left.
    pub fn delete(&mut self, location: Point) -> char {
        self.data[location.y].remove(location.x)
    }

    /// Insert several characters at the given location. See `insert`.
    pub fn insert_text(&mut self, location: Point, string: String) {
        for (x, c) in string.chars().enumerate() {
            self.data[location.y].insert(location.x + x, c);
        }
    }

    /// Insert an empty line at the given location, shifting the subsequent lines down if any.
    pub fn new_line(&mut self, location: Point) {
        self.data.insert(location.y + 1, "\n".into());
    }

    /// Delete a line at the given location, shifting the subsequent lines up if any.
    pub fn delete_line(&mut self, location: Point) -> String {
        self.data.remove(location.y)
    }

    /// Split a line in half, inserting the second half as a new line below the first half.
    pub fn split_line(&mut self, location: Point) {
        let line = self.data[location.y].clone();
        let (left, right) = line.split_at(location.x);
        self.data[location.y] = String::from(left);
        self.data[location.y].push('\n');
        self.data.insert(location.y + 1, right.into());
    }

    /// Load a file from a path and populate the internal data buffer.
    /// Unfortunately replaces the original line endings with CRs for now.
    /// # Panics
    /// When the file can't be loaded, e.g. if it doesn't exist.
    pub fn load_file(&mut self, path: String) {
        let mut file = OpenOptions::new().read(true).write(true).create(true).open(path).unwrap();
        let mut s = String::new();
        let _ = file.read_to_string(&mut s);
        let mut buf = Vec::new();

        for line in s.lines() {
            let mut l = String::from(line);
            l.push('\n');
            buf.push(l);
        }

        if buf.is_empty() {
            buf.push("\n".into());
        }

        self.data = buf;
    }

    /// Save the buffer at the internal filepath, returning the number of bytes written.
    ///
    /// Probably doesn't handle UTF-8 very well du to the `char` to `u8` conversion happening.
    /// # Panics
    /// When no filepath was given when creating the `Buffer`.
    pub fn save_file(&mut self) -> u64 {
        if self.filepath.is_none() { return 0 } // TODO: choose filepath

        let path = self.filepath.clone().unwrap();
        let mut file = OpenOptions::new().read(true).write(true).create(true).open(path).unwrap();
        for line in &self.data {
            let _ = file.write_all(line.as_bytes());
        }

        file.metadata().unwrap().len()
    }
}
