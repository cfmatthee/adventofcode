#![allow(unused)]

use day_05::{is_nice, parse};

fn process(input: &str) -> String {
    let input = parse(input);
    let result = input.into_iter().filter(|item| is_nice(item)).count();
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
    #[case("ugknbfddgicrmopn", "1")]
    #[case("aaa", "1")]
    #[case("jchzalrnumimnmhp", "0")]
    #[case("haegwjzuvuyypxyu", "0")]
    #[case("dvszwmarrgswjxmb", "0")]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process(input));
    }
}
