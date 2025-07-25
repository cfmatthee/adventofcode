#![allow(unused)]

use day_11::blink;

fn process(input: &str) -> String {
    let mut input: Vec<_> = input
        .split_whitespace()
        .map(|item| item.parse::<usize>().unwrap())
        .collect();

    for _ in 0..25 {
        input = blink(input);
    }

    let result = input.len();
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
        let input = r#"125 17"#;
        assert_eq!("55312", process(input));
    }
}
