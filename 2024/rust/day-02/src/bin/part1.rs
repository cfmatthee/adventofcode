#![allow(unused)]

use day_02::is_valid;

fn process(file: &str) -> anyhow::Result<String> {
    let result = file
        .lines()
        .filter(|&line| {
            let items: Vec<_> = line
                .split_whitespace()
                .map(|i| i.parse::<usize>().unwrap())
                .collect();
            is_valid(items)
        })
        .count();
    Ok(result.to_string())
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input1.txt");
    let result = process(file)?;
    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"#;
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
