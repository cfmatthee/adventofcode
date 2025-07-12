#![allow(unused)]

fn process(file: &str) String {
    todo!("part-2");
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
        todo!("set up test part-2");
        let input = r#""#;
        assert_eq!("", process(input));
    }
}
