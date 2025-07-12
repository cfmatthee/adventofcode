#![allow(unused)]

use day_02::is_valid;

fn process(file: &str) -> String {
    let result = file
        .lines()
        .filter(|&line| {
            let items: Vec<_> = line
                .split_whitespace()
                .map(|i| i.parse::<usize>().unwrap())
                .collect();
            let size = items.len();
            (0..size).any(|i| {
                let mut copy = items.clone();
                copy.remove(i);
                is_valid(copy)
            })
        })
        .count();
    result.to_string()
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
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;
        assert_eq!("4", process(input));
    }
}
