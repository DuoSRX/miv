use std::io::{Read, Write};
use std::fs::OpenOptions;
use std::cmp;

use point::Point;

pub struct Buffer {
    pub data: Vec<Vec<char>>,
    pub filepath: Option<String>,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            data: vec!(Vec::new()),
            filepath: None,
        }
    }

    pub fn line_at(&mut self, y: usize) -> Option<&Vec<char>> {
        self.data.get(y)
    }

    pub fn last_non_empty_col(&mut self, location: Point) -> usize {
        let x = self.line_at(location.y)
            .and_then(|line| match line.get(location.x) {
                Some(_) => Some(location.x as isize),
                None => Some(line.len() as isize - 2),
            }).unwrap();

        cmp::max(0, x) as usize
    }

    pub fn char_at(&mut self, location: Point) -> Option<&char> {
        self.line_at(location.y).and_then(|line| line.get(location.x))
    }

    pub fn insert(&mut self, location: Point, c: char) {
        self.data[location.y].insert(location.x, c);
    }

    pub fn delete(&mut self, location: Point) -> char {
        self.data[location.y].remove(location.x)
    }

    pub fn insert_text(&mut self, location: Point, chars: Vec<char>) {
        for (x, &c) in chars.iter().enumerate() {
            self.data[location.y].insert(x + location.x, c);
        }
    }

    pub fn new_line(&mut self, location: Point) {
        self.data.insert(location.y + 1, vec!('\n'));
    }

    pub fn delete_line(&mut self, location: Point) -> Vec<char> {
        self.data.remove(location.y)
    }

    pub fn split_line(&mut self, location: Point) {
        let newline = self.data[location.y].split_off(location.x);
        self.data.insert(location.y + 1, newline);
    }

    pub fn load_file(&mut self, path: String) {
        let mut file = OpenOptions::new().read(true).write(true).create(true).open(path).unwrap();
        let mut s = String::new();
        let _ = file.read_to_string(&mut s);
        let mut buf = Vec::new();

        for line in s.lines() {
            let mut l = Vec::with_capacity(line.len());
            for c in line.chars() {
                l.push(c);
            }
            l.push('\n');
            buf.push(l)
        }

        if buf.is_empty() {
            buf.push(Vec::new());
        }

        self.data = buf;
    }

    pub fn save_file(&mut self) -> u64 {
        if self.filepath.is_none() { return 0 } // TODO: choose filepath

        let path = self.filepath.clone().unwrap();
        let mut file = OpenOptions::new().read(true).write(true).create(true).open(path).unwrap();
        for line in &self.data {
            for &c in line.iter() {
                let _ = file.write_all(&[c as u8]);
            }
        }

        file.metadata().unwrap().len()
    }
}

