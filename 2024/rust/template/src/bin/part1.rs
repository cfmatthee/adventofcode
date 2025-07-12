#![allow(unused)]

fn process(file: &str) String {
    todo!("part-1");
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
        todo!("set up test part-1");
        let input = r#""#;
        assert_eq!("", process(input));
    }
}
