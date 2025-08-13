#![allow(unused)]

use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

use day_03::{mul, Instruction};

fn process(input: &str) -> String {
    let (_, instructions) = many1(many_till(anychar, mul).map(|(_, ins)| ins))
        .parse(input)
        .unwrap();
    let result: u32 = instructions
        .into_iter()
        .map(|ins| match ins {
            Instruction::Mul(a, b) => a * b,
            _ => panic!("should not have any other instruction"),
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
    use super::process;

    #[test]
    fn test_process() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        assert_eq!("161", process(input));
    }
}
