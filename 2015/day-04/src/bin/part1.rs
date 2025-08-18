#![allow(unused)]

fn process(input: &str) -> String {
    let input = input.trim();
    let mut result = 0;
    loop {
        let input = format!("{input}{result}");
        let hash = format!("{:x}", md5::compute(input));
        if hash.starts_with("00000") {
            break;
        }
        result += 1;
    }
    result.to_string()
}

fn main() {
    let file = include_str!("../../input1.txt");
    let result = process(file);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::process;

    #[rstest]
    #[case("abcdef", "609043")]
    #[case("pqrstuv", "1048970")]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(process(input), expected);
    }
}
