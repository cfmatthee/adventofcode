#![allow(unused)]

use day_06::{find_guard, parse, Direction, Guard};
use shared::Vec2;

fn process(input: &str) -> String {
    let mut input = parse(input);
    let mut guard = find_guard(&input);
    let rows = input.len() as i32;
    let cols = input[0].len() as i32;

    #[allow(clippy::never_loop)]
    loop {
        let Guard {
            position: Vec2 { y, x },
            facing,
        } = &guard;
        input[*y as usize][*x as usize] = 'X';
        let (new_y, new_x) = match facing {
            Direction::LookingUp => (*y - 1, *x),
            Direction::LookingRight => (*y, *x + 1),
            Direction::LookingDown => (*y + 1, *x),
            Direction::LookingLeft => (*y, *x - 1),
        };
        if new_y < 0 || new_y >= rows || new_x < 0 || new_x >= cols {
            break;
        }

        if input[new_y as usize][new_x as usize] != '#' {
            guard.position.y = new_y;
            guard.position.x = new_x;
        } else {
            guard.facing = match guard.facing {
                Direction::LookingUp => Direction::LookingRight,
                Direction::LookingRight => Direction::LookingDown,
                Direction::LookingDown => Direction::LookingLeft,
                Direction::LookingLeft => Direction::LookingUp,
            };
        }
    }

    let result = input
        .into_iter()
        .flatten()
        .filter(|char| char == &'X')
        .count();
    result.to_string()
}

fn main() {
    let file = include_str!("../../input1.txt");
    let result = process(file);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;
        assert_eq!("41", process(input));
    }
}
