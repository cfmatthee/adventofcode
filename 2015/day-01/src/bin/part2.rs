#![allow(unused)]

fn process(input: &str) -> String {
    let result =
        input
            .trim()
            .chars()
            .enumerate()
            .fold((0_i32, 0_usize), |(count, pos), (idx, value)| {
                if pos > 0 {
                    return (count, pos);
                }
                let count = match value {
                    '(' => count + 1,
                    ')' => count - 1,
                    _ => panic!("should only have '(' and ')' in input"),
                };
                if count == -1 {
                    return (count, idx + 1);
                }
                (count, 0)
            });
    result.1.to_string()
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
        let input = r#"(()))"#;
        assert_eq!("5", process(input));
    }
}
