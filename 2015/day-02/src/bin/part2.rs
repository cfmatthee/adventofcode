#![allow(unused)]

use day_02::parse;

fn process(input: &str) -> String {
    let input = parse(input);
    let result = input
        .into_iter()
        .map(|mut b| {
            b.sort();
            let size1 = 2 * (b[0] + b[1]);
            let size2 = b[0] * b[1] * b[2];
            size1 + size2
        })
        .sum::<u32>();
    result.to_string()
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
    #[case("2x3x4", "34")]
    #[case("1x1x10", "14")]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process(input), expected);
    }
}
