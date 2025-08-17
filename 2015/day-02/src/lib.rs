pub fn parse(input: &str) -> Vec<[u32; 3]> {
    input
        .lines()
        .map(|line| {
            let sizes: Vec<_> = line
                .split('x')
                .map(|item| item.parse::<u32>().unwrap())
                .collect();
            [sizes[0], sizes[1], sizes[2]]
        })
        .collect()
}
