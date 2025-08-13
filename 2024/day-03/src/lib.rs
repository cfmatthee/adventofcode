#![allow(unused)]

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self},
    combinator::value,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Clone, Debug)]
pub enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

pub fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        mul,
        value(Instruction::Do, tag("do()")),
        value(Instruction::Dont, tag("don't()")),
    ))
    .parse(input)
}

pub fn mul(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(complete::u32, tag(","), complete::u32),
        tag(")"),
    )
    .parse(input)?;
    Ok((input, Instruction::Mul(pair.0, pair.1)))
}
