#![allow(unused)]

use day_04::{parse, OFFSETS2};
use shared::Vec2;

fn process(input: &str) -> String {
    let items = parse(input);
    let result = items
        .iter()
        .filter(|(position, value)| {
            if **value != 'A' {
                return false;
            }
            let count = OFFSETS2
                .iter()
                .map(|offset_list| {
                    offset_list
                        .iter()
                        .filter_map(|offsets| {
                            let new_pos: Vec2 = *position + &From::from(offsets);
                            let item = items.get(&new_pos);
                            item
                        })
                        .collect::<String>()
                })
                .filter(|result| result == "MAS" || result == "SAM")
                .count();
            count == 2
        })
        .count();
    result.to_string()
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
        assert_eq!("9", process(input));
    }
}
