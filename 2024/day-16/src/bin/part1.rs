#![allow(unused)]

use day_16::parse;
use glam::IVec2;
use pathfinding::prelude::*;

fn process(input: &str) -> String {
    let map = parse(input);

    let result = dijkstra(
        &(map.start_position, IVec2::X),
        |(pos, direction)| {
            let next = pos + direction;
            let mut options = vec![
                ((*pos, direction.perp()), 1000),
                ((*pos, -direction.perp()), 1000),
            ];
            if !map.walls.contains(&next) {
                options.push(((next, *direction), 1));
            }
            options
        },
        |&(pos, _)| pos == map.stop_position,
    )
    .expect("valid result");

    result.1.to_string()
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
        let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"#;
        assert_eq!("7036", process(input));
    }
}
