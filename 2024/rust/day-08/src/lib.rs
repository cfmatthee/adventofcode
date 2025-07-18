use std::collections::HashMap;

use itertools::enumerate;

pub fn parse(input: &str) -> (HashMap<char, Vec<(u32, u32)>>, usize, usize) {
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

    let mut result: HashMap<char, Vec<(u32, u32)>> = HashMap::new();
    for (char, (row, col)) in input {
        let array = result.get(&char);
        match array {
            Some(array) => {
                let mut array = array.clone();
                array.push((row as u32, col as u32));
                result.insert(char, array);
            }
            None => {
                let array = vec![(row as u32, col as u32)];
                result.insert(char, array);
            }
        };
    }
    (result, rows, cols)
}

pub fn is_valid(pos: &(i32, i32), rows: &usize, cols: &usize) -> bool {
    let c = pos.0;
    let r = pos.1;
    r >= 0 && r < (*rows as i32) && c >= 0 && c < (*cols as i32)
}
