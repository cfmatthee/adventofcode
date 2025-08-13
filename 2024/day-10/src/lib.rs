use std::collections::{HashMap, HashSet};

use itertools::enumerate;

pub type Position = (i16, i16);

pub fn parse(input: &str) -> HashMap<Position, u8> {
    let mut result: HashMap<Position, u8> = HashMap::new();
    for (row, line) in enumerate(input.lines()) {
        for (col, char) in enumerate(line.chars()) {
            let pos: Position = (row as i16, col as i16);
            result.insert(pos, char.to_digit(10).unwrap() as u8);
        }
    }
    result
}

pub fn find_endpoints(input: &HashMap<Position, u8>, start: &Position) -> Vec<Position> {
    let id = input.get(start).unwrap();
    if *id == 9 {
        return vec![*start];
    }

    let (row, col) = start;
    let top = (row - 1, *col);
    let bottom = (row + 1, *col);
    let left = (*row, col - 1);
    let right = (*row, col + 1);
    let trails = [top, bottom, left, right]
        .into_iter()
        .filter(|pos| {
            let val = input.get(pos).map(|&v| v == id + 1).unwrap_or(false);
            val
        })
        .flat_map(|pos| find_endpoints(input, &pos))
        .collect::<Vec<_>>();
    (HashSet::<_>::from_iter(trails))
        .into_iter()
        .collect::<Vec<_>>()
}

pub fn find_trails(input: &HashMap<Position, u8>, start: &Position) -> u32 {
    let id = input.get(start).unwrap();
    if *id == 9 {
        return 1;
    }

    let (row, col) = start;
    let top = (row - 1, *col);
    let bottom = (row + 1, *col);
    let left = (*row, col - 1);
    let right = (*row, col + 1);
    let trails = [top, bottom, left, right]
        .into_iter()
        .filter(|pos| {
            let val = input.get(pos).map(|&v| v == id + 1).unwrap_or(false);
            val
        })
        .map(|pos| find_trails(input, &pos))
        .sum::<u32>();
    trails
}
