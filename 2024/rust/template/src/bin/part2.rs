#![allow(unused)]

type Result<T> = std::result::Result<T, anyhow::Error>;

fn process(file: &str) -> Result<String> {
    todo!("part-2");
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input2.txt");
    let result = process(file)?;
    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{process, Result};

    #[test]
    fn test_process() -> Result<()> {
        todo!("set up test part-2");
        let input = r#""#;
        assert_eq!("", process(input)?);
        Ok(())
    }
}
