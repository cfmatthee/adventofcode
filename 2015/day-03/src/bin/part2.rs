#![allow(unused)]

use std::collections::HashSet;

use shared::Vec2;

fn process(input: &str) -> String {
    let mut santa = Vec2::new(0, 0);
    let mut robot = Vec2::new(0, 0);
    let mut result: HashSet<Vec2> = HashSet::new();
    result.insert(santa);

    let mut is_santas_turn = true;

    for c in input.chars() {
        let pos = match is_santas_turn {
            true => santa,
            false => robot,
        };
        let pos = match c {
            '<' => pos + Vec2::new(-1, 0),
            '>' => pos + Vec2::new(1, 0),
            '^' => pos + Vec2::new(0, -1),
            'v' => pos + Vec2::new(0, 1),
            _ => panic!("invalid input"),
        };
        result.insert(pos);
        match is_santas_turn {
            true => santa = pos,
            false => robot = pos,
        };
        is_santas_turn = !is_santas_turn;
        result.insert(pos);
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
    use rstest::rstest;

    use super::process;

    #[rstest]
    #[case("^v", "3")]
    #[case("^>v<", "3")]
    #[case("^v^v^v^v^v", "11")]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process(input), expected);
    }
}
