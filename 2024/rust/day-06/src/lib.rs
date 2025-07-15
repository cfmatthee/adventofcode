use itertools::enumerate;

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
    pub row: i32,
    pub col: i32,
    pub facing: Direction,
}

pub fn find_guard(input: &Vec<Vec<char>>) -> Guard {
    for (row, line) in enumerate(input) {
        for (col, char) in enumerate(line) {
            if char == &'^' {
                return Guard {
                    row: row as i32,
                    col: col as i32,
                    facing: Direction::LookingUp,
                };
            }
        }
    }
    panic!("Guard not found");
}
