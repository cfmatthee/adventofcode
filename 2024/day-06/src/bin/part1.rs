#![allow(unused)]

use day_06::{find_guard, parse, Direction, Guard};

fn process(input: &str) -> String {
    let mut input = parse(input);
    let mut guard = find_guard(&input);
    let rows = input.len() as i32;
    let cols = input[0].len() as i32;

    #[allow(clippy::never_loop)]
    loop {
        let Guard { row, col, facing } = &guard;
        input[*row as usize][*col as usize] = 'X';
        let (new_row, new_col) = match facing {
            Direction::LookingUp => (*row - 1, *col),
            Direction::LookingRight => (*row, *col + 1),
            Direction::LookingDown => (*row + 1, *col),
            Direction::LookingLeft => (*row, *col - 1),
        };
        if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
            break;
        }

        if input[new_row as usize][new_col as usize] != '#' {
            guard.row = new_row;
            guard.col = new_col;
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
