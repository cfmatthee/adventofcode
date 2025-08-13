#![allow(unused)]

use day_03::instruction;
use nom::{
    character::complete::anychar,
    multi::{many1, many_till},
    Parser,
};

fn process(input: &str) -> String {
    let (_, instructions) = many1(many_till(anychar, instruction).map(|(_, ins)| ins))
        .parse(input)
        .unwrap();
    let (_, result) = instructions
        .into_iter()
        .fold((true, 0), |(should_do, acc), ins| match ins {
            day_03::Instruction::Mul(a, b) => {
                if should_do {
                    (should_do, acc + a * b)
                } else {
                    (should_do, acc)
                }
            }
            day_03::Instruction::Do => (true, acc),
            day_03::Instruction::Dont => (false, acc),
        });

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
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        assert_eq!("48", process(input));
    }
}
