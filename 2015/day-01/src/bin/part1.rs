#![allow(unused)]

fn process(input: &str) -> String {
    let result: i32 = input
        .trim()
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("should only have '(' and ')' in input"),
        })
        .sum();
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
    #[case("(())", "0")]
    #[case("()()", "0")]
    #[case("(((", "3")]
    #[case("(()(()(", "3")]
    #[case("))(((((", "3")]
    #[case(")))", "-3")]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process(input), expected);
    }
}
