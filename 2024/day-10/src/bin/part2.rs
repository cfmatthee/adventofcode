#![allow(unused)]

use day_10::{find_trails, parse};

fn process(input: &str) -> String {
    let input = parse(input);
    let trailheads = input
        .iter()
        .filter(|(pos, val)| *val == &0_u8)
        .map(|(pos, _)| find_trails(&input, pos))
        .sum::<u32>();

    trailheads.to_string()
}

fn main() {
    let file = include_str!("../../input2.txt");
    let result = process(file);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;
        assert_eq!("81", process(input));
    }
}
