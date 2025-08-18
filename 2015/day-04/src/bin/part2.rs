#![allow(unused)]

fn process(input: &str) -> String {
    let input = input.trim();
    let mut result = 0;
    loop {
        let input = format!("{input}{result}");
        let hash = format!("{:x}", md5::compute(input));
        if hash.starts_with("000000") {
            break;
        }
        result += 1;
    }
    result.to_string()
}

fn main() {
    let file = include_str!("../../input2.txt");
    let result = process(file);
    println!("{result}");
}
