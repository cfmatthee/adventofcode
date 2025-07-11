#![allow(unused)]

fn process(file: &str) -> anyhow::Result<String> {
    todo!("part-1");
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
        todo!("set up test part-1");
        let input = r#""#;
        assert_eq!("", process(input)?);
        Ok(())
    }
}
