use ndarray::prelude::*;
use ndarray_linalg::Solve;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, newline},
    multi::separated_list1,
    IResult, Parser,
};

#[derive(Debug)]
pub struct Button {
    pub name: String,
    pub x: u64,
    pub y: u64,
}

#[derive(Debug)]
pub struct Prize {
    pub x: u64,
    pub y: u64,
}

#[derive(Debug)]
pub struct Machine {
    pub buttons: Vec<Button>,
    pub prize: Prize,
}

pub fn parse(input: &str) -> Vec<Machine> {
    let (_, machines) = separated_list1((newline, newline), machine)
        .parse(input)
        .unwrap();
    machines
}

fn machine(input: &str) -> IResult<&str, Machine> {
    let mut parser = (separated_list1(newline, button), newline, prize);
    let (input, (buttons, _, prize)) = parser.parse(input)?;
    assert_eq!(buttons.len(), 2);
    Ok((input, Machine { buttons, prize }))
}

fn button(input: &str) -> IResult<&str, Button> {
    let mut parser = (
        tag("Button "),
        alphanumeric1,
        tag(": X+"),
        complete::u64,
        tag(", Y+"),
        complete::u64,
    );
    let (input, (_, name, _, x, _, y)) = parser.parse(input)?;
    Ok((
        input,
        Button {
            name: name.to_string(),
            x,
            y,
        },
    ))
}

fn prize(input: &str) -> IResult<&str, Prize> {
    let mut parser = (tag("Prize: X="), complete::u64, tag(", Y="), complete::u64);
    let (input, (_, x, _, y)) = parser.parse(input)?;
    Ok((input, Prize { x, y }))
}

impl Machine {
    #[allow(clippy::result_unit_err)]
    pub fn solve(&self) -> Result<u64, ()> {
        assert_eq!(self.buttons.len(), 2);
        let button1 = self.buttons.first().unwrap();
        let button2 = self.buttons.last().unwrap();
        assert_eq!(button1.name, "A");
        assert_eq!(button2.name, "B");
        let a: Array2<f64> = array![
            [button1.x as f64, button2.x as f64],
            [button1.y as f64, button2.y as f64]
        ];
        let b: Array1<f64> = array![self.prize.x as f64, self.prize.y as f64];
        let x = a.solve_into(b).map_err(|_| ())?;
        let b1 = x[0];
        let b2 = x[1];
        if (b1 - b1.round()).abs() > 1E-2 || (b2 - b2.round()).abs() > 1E-2 {
            return Err(());
        }
        let b1 = b1.round() as u64;
        let b2 = b2.round() as u64;
        let cost = 3 * b1 + b2;
        Ok(cost)
    }
}
