pub fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}

pub fn run_n_steps(start: u64, n: usize) -> u64 {
    (0..n).fold(start, |acc, _| {
        let acc = (acc ^ (acc * 64)) % 16777216;
        let acc = (acc ^ (acc / 32)) % 16777216;
        (acc ^ (acc * 2048)) % 16777216
    })
}
