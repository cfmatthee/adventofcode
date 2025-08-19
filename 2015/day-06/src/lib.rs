use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use shared::Vec2;

#[derive(Debug)]
pub enum Instuction {
    TurnOff(Vec2, Vec2),
    TurnOn(Vec2, Vec2),
    Toggle(Vec2, Vec2),
}

pub fn parse(input: &str) -> Vec<Instuction> {
    let (_, instructions) = separated_list1(newline, instruction).parse(input).unwrap();
    instructions
}

pub fn get_positions(start: Vec2, stop: Vec2) -> Vec<Vec2> {
    let mut result: Vec<Vec2> = Vec::new();
    for x in start.x..=stop.x {
        for y in start.y..=stop.y {
            result.push(Vec2::new(x, y));
        }
    }
    result
}

pub fn convert_position(pos: Vec2) -> usize {
    (pos.y as usize) * 1000 + (pos.x as usize)
}

fn instruction(input: &str) -> IResult<&str, Instuction> {
    alt((turn_on, turn_off, toggle)).parse(input)
}

fn coords(input: &str) -> IResult<&str, Vec2> {
    let (input, (d1, d2)) = separated_pair(digit1, tag(","), digit1).parse(input)?;
    let result = Vec2::new(d1.parse().unwrap(), d2.parse().unwrap());
    Ok((input, result))
}

fn turn_on(input: &str) -> IResult<&str, Instuction> {
    let (input, (_, start, _, stop)) =
        (tag("turn on "), coords, tag(" through "), coords).parse(input)?;
    Ok((input, Instuction::TurnOn(start, stop)))
}

fn turn_off(input: &str) -> IResult<&str, Instuction> {
    let (input, (_, start, _, stop)) =
        (tag("turn off "), coords, tag(" through "), coords).parse(input)?;
    Ok((input, Instuction::TurnOff(start, stop)))
}

fn toggle(input: &str) -> IResult<&str, Instuction> {
    let (input, (_, start, _, stop)) =
        (tag("toggle "), coords, tag(" through "), coords).parse(input)?;
    Ok((input, Instuction::Toggle(start, stop)))
}
