use itertools::enumerate;
use shared::Vec2;

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.to_string().chars().collect())
        .collect()
}

#[derive(Debug)]
pub enum Direction {
    LookingUp,
    LookingRight,
    LookingDown,
    LookingLeft,
}

#[derive(Debug)]
pub struct Guard {
    pub position: Vec2,
    pub facing: Direction,
}

pub fn find_guard(input: &Vec<Vec<char>>) -> Guard {
    for (y, line) in enumerate(input) {
        for (x, char) in enumerate(line) {
            if char == &'^' {
                return Guard {
                    position: Vec2 {
                        x: x as i32,
                        y: y as i32,
                    },
                    facing: Direction::LookingUp,
                };
            }
        }
    }
    panic!("Guard not found");
}
