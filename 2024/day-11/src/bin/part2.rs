#![allow(unused)]

use day_11::blink;

// not sure how long this will take to get an answer, or even if it will get an answer
// there is propably a caching or other clever method to do this
fn process(input: &str) -> String {
    let input: Vec<_> = input
        .split_whitespace()
        .map(|item| item.parse::<usize>().unwrap())
        .collect();

    let total: usize = input
        .into_iter()
        .map(|item| {
            let mut tmp_input = vec![item];
            for i in 0..75 {
                tmp_input = blink(tmp_input);
            }
            let len = tmp_input.len();
            println!("Total so far: {len}");
            len
        })
        .sum();

    let result = total;
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
        let input = r#"125 17"#;
    }
}
