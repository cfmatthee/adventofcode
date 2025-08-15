#![allow(unused)]

use std::collections::HashSet;

use day_08::{is_valid, parse};
use shared::Vec2;

fn process(input: &str) -> String {
    let (input, rows, cols) = parse(input);

    let mut result: HashSet<Vec2> = HashSet::new();
    for (_, positions) in input.iter() {
        let len = positions.len();
        for item_idx in 0..len - 1 {
            let item = &positions[item_idx];
            for pos in positions[(item_idx + 1)..(len)].iter() {
                let diff = item - pos;
                let mut new = *item;
                loop {
                    result.insert(new);
                    new = new + diff;
                    if !is_valid(&new, &rows, &cols) {
                        break;
                    }
                }
                new = *pos;
                loop {
                    result.insert(new);
                    new = new - diff;
                    if !is_valid(&new, &rows, &cols) {
                        break;
                    }
                }
            }
        }
    }

    result.len().to_string()
}

fn main() {
    let file = include_str!("../../input2.txt");
    let result = process(file);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;
        assert_eq!("34", process(input));
    }
}
