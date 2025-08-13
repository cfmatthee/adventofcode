#![allow(unused)]

use std::iter;

use day_09::parse;
use itertools::enumerate;

fn process(input: &str) -> String {
    let mut disk = parse(input);
    while disk.contains(&0_u16) {
        let id = disk.pop().unwrap();
        if id == 0 {
            continue;
        }
        let idx = enumerate(disk.clone())
            .find(|(i, val)| val == &0_u16)
            .unwrap()
            .0;
        disk[idx] = id;
    }

    let result = enumerate(disk).fold(0_usize, |acc, (pos, val)| acc + pos * ((val - 1) as usize));
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
        let input = r#"2333133121414131402"#;
        assert_eq!("1928", process(input));
    }
}
