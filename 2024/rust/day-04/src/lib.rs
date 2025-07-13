use std::{collections::HashMap, ops::Add};

use itertools::enumerate;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Position {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl Add<&(i32, i32)> for &Position {
    type Output = Position;

    fn add(self, rhs: &(i32, i32)) -> Self::Output {
        Position {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

pub fn parse(input: &str) -> HashMap<Position, char> {
    let mut result: HashMap<Position, char> = HashMap::new();
    for (y, line) in enumerate(input.lines()) {
        for (x, item) in enumerate(line.chars()) {
            let pos = Position {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
            };
            result.insert(pos, item);
        }
    }
    result
}

pub static OFFSETS1: [[(i32, i32); 3]; 8] = [
    [(0, -1), (0, -2), (0, -3)],
    [(1, -1), (2, -2), (3, -3)],
    [(1, 0), (2, 0), (3, 0)],
    [(1, 1), (2, 2), (3, 3)],
    [(0, 1), (0, 2), (0, 3)],
    [(-1, 1), (-2, 2), (-3, 3)],
    [(-1, 0), (-2, 0), (-3, 0)],
    [(-1, -1), (-2, -2), (-3, -3)],
];

pub static OFFSETS2: [[(i32, i32); 3]; 2] =
    [[(-1, -1), (0, 0), (1, 1)], [(1, -1), (0, 0), (-1, 1)]];
