#![allow(unused)]

use day_07::{calculate_possible_answers, parse};

fn process(input: &str) -> String {
    let input = parse(input);

    let result = input
        .iter()
        .filter(|input| {
            calculate_possible_answers(vec![], &input.numbers, 2).contains(&input.total)
        })
        .map(|input| input.total)
        .sum::<u64>();
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
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#;
        assert_eq!("3749", process(input));
    }
}
