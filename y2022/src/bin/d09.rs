use std::{collections::HashSet, fs};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, *,
};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_moves(input: &str) -> IResult<&str, Vec<Direction>> {
    let (input, moves) =
        separated_list1(newline, separated_pair(direction, tag(" "), complete::u32))(input)?;
    let moves = moves
        .iter()
        .flat_map(|(direction, count)| vec![*direction; *count as usize])
        .collect();
    Ok((input, moves))
}

fn direction(input: &str) -> IResult<&str, Direction> {
    let (input, dir) = alt((
        complete::char('U').map(|_| Direction::Up),
        complete::char('D').map(|_| Direction::Down),
        complete::char('L').map(|_| Direction::Left),
        complete::char('R').map(|_| Direction::Right),
    ))(input)?;
    Ok((input, dir))
}

fn main() {
    let data = fs::read_to_string("src/files/d09").unwrap();
    println!("{}", do_part_1(&data));
    println!("{}", do_part_2(&data));
}

fn do_part_1(input: &str) -> usize {
    let (_, moves) = get_moves(input).unwrap();

    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut positions = HashSet::from([tail]);
    for m in moves.iter() {
        let old_head = head.clone();
        match m {
            Direction::Up => head.1 += 1,
            Direction::Down => head.1 -= 1,
            Direction::Left => head.0 -= 1,
            Direction::Right => head.0 += 1,
        };
        let distance: i32 = ((head.0 - tail.0) as i32).pow(2) + ((head.1 - tail.1) as i32).pow(2);
        if distance > 2 {
            tail = old_head;
            positions.insert(tail.clone());
        }
    }

    positions.len()
}

fn do_part_2(input: &str) -> usize {
    let (_, moves) = get_moves(input).unwrap();

    let mut rope = vec![(0, 0); 10];
    let mut positions = HashSet::from([*rope.last().unwrap()]);
    for m in moves.iter() {
        let old_rope = rope.clone();
        match m {
            Direction::Up => rope[0].1 += 1,
            Direction::Down => rope[0].1 -= 1,
            Direction::Left => rope[0].0 -= 1,
            Direction::Right => rope[0].0 += 1,
        };
        for idx in 1..rope.len() {
            let head = &rope[idx - 1];
            let tail = &rope[idx];
            let distance: i32 =
                ((head.0 - tail.0) as i32).pow(2) + ((head.1 - tail.1) as i32).pow(2);
            if distance > 2 {
                if head.0 == tail.0 {
                    if head.1 > tail.1 {
                        rope[idx].1 += 1;
                    } else {
                        rope[idx].1 -= 1;
                    }
                } else if head.1 == tail.1 {
                    if head.0 > tail.0 {
                        rope[idx].0 += 1;
                    } else {
                        rope[idx].0 -= 1;
                    }
                } else {
                    // diagonal
                    if head.1 > tail.1 {
                        if head.0 > tail.0 {
                            rope[idx].0 += 1;
                            rope[idx].1 += 1;
                        } else {
                            rope[idx].0 -= 1;
                            rope[idx].1 += 1;
                        }
                    } else {
                        if head.0 > tail.0 {
                            rope[idx].0 += 1;
                            rope[idx].1 -= 1;
                        } else {
                            rope[idx].0 -= 1;
                            rope[idx].1 -= 1;
                        }
                    }
                }
            }
        }
        positions.insert(*rope.last().unwrap());
    }

    positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part1() {
        let result = do_part_1(&INPUT);
        assert_eq!(result, 13);
    }

    #[test]
    fn part2() {
        let result = do_part_2(&INPUT);
        assert_eq!(result, 1);
    }

    #[test]
    fn longer_part2() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let result = do_part_2(input);
        assert_eq!(result, 36);
    }
}
