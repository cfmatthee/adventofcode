use std::fs;

fn main() {
    let data = fs::read_to_string("src/files/d01").unwrap();
    let answer = solve(&data);
    println!("{:?}", answer.iter().sum::<u32>());
}

fn solve(data: &str) -> Vec<u32> {
    let mut slices: Vec<_> = data
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|x| x.parse().unwrap_or(0))
                .sum::<u32>()
        })
        .collect();
    slices.sort();
    slices
        .iter()
        .rev()
        .take(3)
        .map(|x| x.clone())
        .collect::<Vec<_>>()
}
