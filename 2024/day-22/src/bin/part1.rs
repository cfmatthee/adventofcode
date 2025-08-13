#![allow(unused)]

use day_22::{parse, run_n_steps};

fn process(input: &str) -> String {
    let input = parse(input);
    let result = input
        .into_iter()
        .map(|val| run_n_steps(val, 2000))
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
    use day_22::run_n_steps;

    use super::process;

    #[test]
    fn test_process() {
        let input = r#"1
10
100
2024
"#;
        assert_eq!("37327623", process(input));
    }

    #[test]
    fn test_one() {
        assert_eq!(run_n_steps(123, 10), 5908254);
    }
}
