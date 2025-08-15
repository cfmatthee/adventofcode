use std::collections::HashMap;

use itertools::enumerate;
use shared::Vec2;

pub fn parse(input: &str) -> HashMap<Vec2, char> {
    let mut result: HashMap<Vec2, char> = HashMap::new();
    for (y, line) in enumerate(input.lines()) {
        for (x, item) in enumerate(line.chars()) {
            let pos = Vec2 {
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
