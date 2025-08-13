#![allow(unused)]

fn process(file: &str) -> String {
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();

    for line in file.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<usize>().unwrap());
        right.push(items.next().unwrap().parse::<usize>().unwrap());
    }

    let result: usize = left
        .iter()
        .map(|n| n * right.iter().filter(|r| r == &n).count())
        .sum();

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
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3
"#;
        assert_eq!("31", process(input));
    }
}
