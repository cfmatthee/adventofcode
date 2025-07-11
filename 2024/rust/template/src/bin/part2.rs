#![allow(unused)]

fn process(file: &str) -> anyhow::Result<String> {
    todo!("part-2");
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let file = include_str!("../../input2.txt");
    let result = process(file)?;
    println!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() -> anyhow::Result<()> {
        todo!("set up test part-2");
        let input = r#""#;
        assert_eq!("", process(input)?);
        Ok(())
    }
}
