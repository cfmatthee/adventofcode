use std::collections::HashSet;
use std::fs;

fn main() {
    let data = fs::read_to_string("src/files/d03").unwrap();
    part1(&data);
    part2(&data);
}

fn part1(data: &str) {
    let slices = data
        .split("\n")
        .map(|line| {
            let len = line.len() / 2;
            let part1 = line[..len].chars().map(|c| c as u32).collect::<Vec<_>>();
            let part2 = line[len..].chars().map(|c| c as u32).collect::<Vec<_>>();
            let part1: HashSet<u32> = HashSet::<u32>::from_iter(part1.iter().cloned());
            let part2: HashSet<u32> = HashSet::<u32>::from_iter(part2.iter().cloned());
            let intersection = part1.intersection(&part2).cloned().collect::<Vec<_>>();
            intersection
        })
        .flat_map(|i| i)
        .map(|x| {
            if x >= ('a' as u32) {
                return x - ('a' as u32) + 1;
            } else {
                return x - ('A' as u32) + 27;
            }
        })
        .sum::<u32>();
    // .collect::<Vec<_>>();

    println!("{:?}", &slices);
}

fn part2(data: &str) {
    let slices = data.split("\n").collect::<Vec<_>>();

    let mut values = vec![];
    let len = slices.len();
    for i in (0..len).step_by(3) {
        let group = slices
            .iter()
            .skip(i)
            .take(3)
            .map(|s| s.chars().map(|c| c as u32).collect::<Vec<_>>())
            .map(|v| HashSet::<u32>::from_iter(v.iter().cloned()))
            .collect::<Vec<_>>();
        let mut g = group[0].clone();
        g.retain(|e| group[1].contains(e));
        g.retain(|e| group[2].contains(e));
        let x = *g.iter().next().unwrap();
        if x >= ('a' as u32) {
            values.push(x - ('a' as u32) + 1);
        } else {
            values.push(x - ('A' as u32) + 27);
        }
    }

    println!("{:?}", values.iter().sum::<u32>());
}
