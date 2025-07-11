#![allow(unused)]

use itertools::Itertools;

type Result<T> = std::result::Result<T, anyhow::Error>;

fn process(file: &str) -> Result<String> {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in file.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<i32>().unwrap());
        right.push(items.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let result: i32 = std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum();

    Ok(result.to_string())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input1.txt");
    let result = process(file)?;
    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{process, Result};

    #[test]
    fn test_process() -> Result<()> {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
        assert_eq!("11", process(input)?);
        Ok(())
    }
}
