use std::io::{Read, Write};
use std::fs::{File, OpenOptions};

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

    pub fn insert(&mut self, location: Point, c: char) {
        let line = &mut self.data[location.y];
        let x = location.x + 1;

        if x > line.len() {
            line.push(c);
        } else {
            line.insert(x, c);
        }
    }

    pub fn delete(&mut self, location: Point) {
        self.data[location.y].remove(location.x);
    }

    pub fn new_line(&mut self, location: Point) {
        self.data.insert(location.y + 1, Vec::new());
    }

    pub fn delete_line(&mut self, location: Point) {
        self.data.remove(location.y);
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
            let _ = file.write_all(&[b'\n']);
        }

        file.metadata().unwrap().len()
    }
}

