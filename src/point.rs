#[derive(PartialEq,Debug,Copy,Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

use self::Direction::*;

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x: x, y: y }
    }

    pub fn with_direction(&self, direction: Direction) -> Point {
        match direction {
            Up => Point { x: self.x, y: self.y - 1 },
            Down => Point { x: self.x, y: self.y + 1 },
            Left => Point { x: self.x - 1, y: self.y },
            Right => Point { x: self.x + 1, y: self.y  }
        }
    }
}

#[derive(PartialEq,Copy,Clone,Debug)]
pub enum Direction { Up, Down, Left, Right }
