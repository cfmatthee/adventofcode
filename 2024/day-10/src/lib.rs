use std::collections::{HashMap, HashSet};

use itertools::enumerate;
use shared::Vec2;

pub fn parse(input: &str) -> HashMap<Vec2, u8> {
    let mut result: HashMap<Vec2, u8> = HashMap::new();
    for (row, line) in enumerate(input.lines()) {
        for (col, char) in enumerate(line.chars()) {
            let pos: Vec2 = Vec2::new(col as i32, row as i32);
            result.insert(pos, char.to_digit(10).unwrap() as u8);
        }
    }
    result
}

pub fn find_endpoints(input: &HashMap<Vec2, u8>, start: &Vec2) -> Vec<Vec2> {
    let id = input.get(start).unwrap();
    if *id == 9 {
        return vec![*start];
    }

    let Vec2 { x, y } = start;
    let top = Vec2::new(*x, y - 1);
    let bottom = Vec2::new(*x, y + 1);
    let left = Vec2::new(x - 1, *y);
    let right = Vec2::new(x + 1, *y);
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

pub fn find_trails(input: &HashMap<Vec2, u8>, start: &Vec2) -> u32 {
    let id = input.get(start).unwrap();
    if *id == 9 {
        return 1;
    }

    let Vec2 { x, y } = start;
    let top = Vec2::new(*x, y - 1);
    let bottom = Vec2::new(*x, y + 1);
    let left = Vec2::new(x - 1, *y);
    let right = Vec2::new(x + 1, *y);
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
