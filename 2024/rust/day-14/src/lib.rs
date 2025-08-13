use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    IResult, Parser,
};

#[derive(Clone, Debug)]
pub struct Robot {
    pub position: (i32, i32),
    pub velocity: (i32, i32),
}

#[derive(Debug)]
pub struct Grid {
    size: (i32, i32),
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

fn vector(input: &str) -> IResult<&str, (i32, i32)> {
    let mut parser = separated_list1(tag(","), complete::i32);
    let (input, result) = parser.parse(input)?;
    Ok((input, (result[0], result[1])))
}

impl Robot {
    pub fn step(self, grid_size: &(i32, i32)) -> Self {
        let mut new_pos = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
        );

        if new_pos.0 < 0 {
            new_pos.0 += grid_size.0;
        }
        if new_pos.0 >= grid_size.0 {
            new_pos.0 -= grid_size.0;
        }
        if new_pos.1 < 0 {
            new_pos.1 += grid_size.1;
        }
        if new_pos.1 >= grid_size.1 {
            new_pos.1 -= grid_size.1;
        }

        Robot {
            position: new_pos,
            velocity: self.velocity,
        }
    }
}

impl Grid {
    pub fn new(size: (usize, usize), robots: Vec<Robot>) -> Self {
        Self {
            size: (size.0 as i32, size.1 as i32),
            robots,
        }
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
        let x_boundary = self.size.0 / 2;
        let y_boundary = self.size.1 / 2;
        for robot in self.robots.iter() {
            match robot.position {
                (x, y) if x < x_boundary && y < y_boundary => count[0] += 1,
                (x, y) if x > x_boundary && y < y_boundary => count[1] += 1,
                (x, y) if x < x_boundary && y > y_boundary => count[2] += 1,
                (x, y) if x > x_boundary && y > y_boundary => count[3] += 1,
                (_, _) => (),
            };
        }
        count.iter().product()
    }
}
