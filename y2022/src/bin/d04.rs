use std::fs;

fn main() {
    let data = fs::read_to_string("src/files/d04").unwrap();
    part1(&data);
    part2(&data);
}

fn part1(data: &str) {
    let slices = data
        .split("\n")
        .map(|line| {
            let parts = line
                .split(&[',', '-'])
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let s1 = parts[1] - parts[0];
            let s2 = parts[3] - parts[2];
            if s2 > s1 {
                return (parts[2], parts[3], parts[0], parts[1]);
            } else {
                return (parts[0], parts[1], parts[2], parts[3]);
            }
        })
        .filter(|x| x.2 >= x.0 && x.3 <= x.1)
        .collect::<Vec<_>>();

    println!("{:?}", &slices.len());
}

fn part2(data: &str) {
    let slices = data
        .split("\n")
        .map(|line| {
            let parts = line
                .split(&[',', '-'])
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            if parts[0] > parts[2] {
                return (parts[2], parts[3], parts[0], parts[1]);
            } else {
                return (parts[0], parts[1], parts[2], parts[3]);
            }
        })
        .filter(|x| x.1 >= x.2)
        .collect::<Vec<_>>();

    println!("{:?}", &slices.len());
}
