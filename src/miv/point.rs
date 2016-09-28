use std::cmp;

#[derive(PartialEq,Debug,Copy,Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

use self::Direction::*;

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point {
            x: x,
            y: y
        }
    }

    pub fn with_direction(&self, direction: Direction) -> Point {
        match direction {
            Up => self.offset(0, -1),
            Down => self.offset(0, 1),
            Left => self.offset(-1, 0),
            Right => self.offset(1, 0),
            BeginningOfLine => Point::new(0, self.y),
            BeginningOfFile => Point::new(0, 0),
            _ => *self,
        }
    }

    pub fn offset(&self, dx: isize, dy: isize) -> Point {
        let x = self.x as isize + dx;
        let y = self.y as isize + dy;
        Point::new(cmp::max(x, 0) as usize, cmp::max(y, 0) as usize)
    }

    pub fn clamp_by(&mut self, max_x: usize, max_y: usize) {
        self.x = cmp::min(self.x, max_x);
        self.y = cmp::min(self.y, max_y);
    }
}

#[derive(PartialEq,Eq,Copy,Clone,Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    BeginningOfLine,
    EndOfLine,
    BeginningOfFile,
    EndOfFile,
}
