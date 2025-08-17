#![allow(unused)]

use day_02::parse;
use itertools::min;

fn process(input: &str) -> String {
    let input = parse(input);
    let result = input
        .iter()
        .map(|b| {
            let side1 = b[0] * b[1];
            let side2 = b[1] * b[2];
            let side3 = b[0] * b[2];
            let sizes = [side1, side2, side3];
            let smallest = sizes.iter().min().unwrap();
            2 * side1 + 2 * side2 + 2 * side3 + smallest
        })
        .sum::<u32>();
    result.to_string()
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
    #[case("2x3x4", "58")]
    #[case("1x1x10", "43")]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process(input), expected);
    }
}
