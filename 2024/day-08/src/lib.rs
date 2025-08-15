use std::collections::HashMap;

use itertools::enumerate;
use shared::Vec2;

#[allow(clippy::type_complexity)]
pub fn parse(input: &str) -> (HashMap<char, Vec<Vec2>>, usize, usize) {
    let mut rows = 0;
    let mut cols = 0;
    let input = enumerate(input.lines())
        .flat_map(|(row, line)| {
            rows = row + 1;
            cols = line.len();
            enumerate(line.chars())
                .map(|(col, char)| (char, (row, col)))
                .collect::<Vec<_>>()
        })
        .filter(|(char, _)| char != &'.')
        .collect::<Vec<_>>();

    let mut result: HashMap<char, Vec<Vec2>> = HashMap::new();
    for (char, (row, col)) in input {
        let array = result.get(&char);
        match array {
            Some(array) => {
                let mut array = array.clone();
                array.push(Vec2::new(col as i32, row as i32));
                result.insert(char, array);
            }
            None => {
                let array = vec![Vec2::new(col as i32, row as i32)];
                result.insert(char, array);
            }
        };
    }
    (result, rows, cols)
}

pub fn is_valid(pos: &Vec2, rows: &usize, cols: &usize) -> bool {
    let c = pos.x;
    let r = pos.y;
    r >= 0 && r < (*rows as i32) && c >= 0 && c < (*cols as i32)
}
