use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    IResult, Parser,
};
use shared::Vec2;

#[derive(Clone, Debug)]
pub struct Robot {
    pub position: Vec2,
    pub velocity: Vec2,
}

#[derive(Debug)]
pub struct Grid {
    size: Vec2,
    robots: Vec<Robot>,
}

pub fn parse(input: &str) -> Vec<Robot> {
    let (_, robots) = separated_list1(newline, entry).parse(input).unwrap();
    robots
}

fn entry(input: &str) -> IResult<&str, Robot> {
    let mut parser = (tag("p="), vector, tag(" v="), vector);
    let (input, (_, position, _, velocity)) = parser.parse(input)?;
    Ok((input, Robot { position, velocity }))
}

fn vector(input: &str) -> IResult<&str, Vec2> {
    let mut parser = separated_list1(tag(","), complete::i32);
    let (input, result) = parser.parse(input)?;
    Ok((input, Vec2::new(result[0], result[1])))
}

impl Robot {
    pub fn step(self, grid_size: &Vec2) -> Self {
        let mut new_pos = self.position + self.velocity;

        if new_pos.x < 0 {
            new_pos.x += grid_size.x;
        }
        if new_pos.x >= grid_size.x {
            new_pos.x -= grid_size.x;
        }
        if new_pos.y < 0 {
            new_pos.y += grid_size.y;
        }
        if new_pos.y >= grid_size.y {
            new_pos.y -= grid_size.y;
        }

        Robot {
            position: new_pos,
            velocity: self.velocity,
        }
    }
}

impl Grid {
    pub fn new(size: Vec2, robots: Vec<Robot>) -> Self {
        Self { size, robots }
    }

    pub fn step(&mut self) {
        self.robots = self
            .robots
            .clone()
            .into_iter()
            .map(|robot| robot.step(&self.size))
            .collect();
    }

    pub fn count(&self) -> usize {
        let mut count = [0_usize; 4];
        let x_boundary = self.size.x / 2;
        let y_boundary = self.size.y / 2;
        for robot in self.robots.iter() {
            match robot.position {
                Vec2 { x, y } if x < x_boundary && y < y_boundary => count[0] += 1,
                Vec2 { x, y } if x > x_boundary && y < y_boundary => count[1] += 1,
                Vec2 { x, y } if x < x_boundary && y > y_boundary => count[2] += 1,
                Vec2 { x, y } if x > x_boundary && y > y_boundary => count[3] += 1,
                _ => (),
            };
        }
        count.iter().product()
    }
}
