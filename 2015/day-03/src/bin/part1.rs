#![allow(unused)]

use std::collections::HashSet;

use shared::Vec2;

fn process(input: &str) -> String {
    let mut pos = Vec2::new(0, 0);
    let mut result: HashSet<Vec2> = HashSet::new();
    result.insert(pos);

    for c in input.chars() {
        pos = match c {
            '<' => pos + Vec2::new(-1, 0),
            '>' => pos + Vec2::new(1, 0),
            '^' => pos + Vec2::new(0, -1),
            'v' => pos + Vec2::new(0, 1),
            _ => panic!("invalid input"),
        };
        result.insert(pos);
    }

    result.len().to_string()
}

fn main() {
    let file = include_str!("../../input1.txt");
    let result = process(file);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::process;

    #[rstest]
    #[case(">", "2")]
    #[case("^>v<", "4")]
    #[case("^v^v^v^v^v", "2")]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process(input), expected);
    }
}
