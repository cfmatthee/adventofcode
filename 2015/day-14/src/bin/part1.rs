#![allow(unused)]

use std::cmp::min;

use day_14::parse;

fn process(input: &str, duration: u32) -> String {
    let input = parse(input);
    let result = input
        .into_iter()
        .map(|reindeer| {
            let cycle_length = reindeer.flight_time + reindeer.rest_time;
            let full_cycles = duration / cycle_length;
            let remainder = duration % cycle_length;
            let mut distance = full_cycles * reindeer.speed * reindeer.flight_time;
            distance += reindeer.speed * min(remainder, reindeer.flight_time);
            distance
        })
        .max()
        .unwrap();
    result.to_string()
}

fn main() {
    let file = include_str!("../../input1.txt");
    let result = process(file, 2503);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::process;

    #[rstest]
    #[case(
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
        "1120"
    )]
    #[case(
        "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        "1056"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, process(input, 1000));
    }
}
