#![allow(unused)]

use std::collections::HashMap;

use day_04::{parse, OFFSETS1};

fn process(input: &str) -> String {
    let items = parse(input);
    let result = items
        .iter()
        .filter_map(|(position, value)| {
            if *value != 'X' {
                return None;
            }
            let sum = OFFSETS1
                .iter()
                .map(|offset_list| {
                    offset_list
                        .iter()
                        .filter_map(|offsets| {
                            let new_pos: shared::Vec2 = position + &From::from(offsets);
                            let item = items.get(&new_pos);
                            item
                        })
                        .collect::<String>()
                })
                .filter(|result| result == "MAS")
                .count();
            Some(sum)
        })
        .sum::<usize>();
    result.to_string()
}

fn main() {
    let file = include_str!("../../input1.txt");
    let result = process(file);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!("18", process(input));
    }
}
