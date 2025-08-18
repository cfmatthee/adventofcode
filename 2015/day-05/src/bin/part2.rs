#![allow(unused)]

use day_05::parse;

fn process(input: &str) -> String {
    let input = parse(input);
    todo!("part-2");
    let result = "";
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
        todo!("set up test part-2");
        let input = r#""#;
        assert_eq!("", process(input));
    }
}
