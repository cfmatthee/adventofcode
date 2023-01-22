use regex::Regex;
use std::fs;

fn main() {
    let data = fs::read_to_string("src/files/d05").unwrap();

    let slices = data.split("\n\n").collect::<Vec<_>>();
    let mut data = get_start_position(&slices[0]);
    // move_crates1(&slices[1], &mut data);
    move_crates2(&slices[1], &mut data);
    report(&data);
}

fn get_start_position(lines: &str) -> Vec<Vec<char>> {
    let mut start = lines.split("\n").collect::<Vec<_>>();
    let size = start.pop().unwrap().len() / 4 + 1;
    let mut data: Vec<Vec<char>> = vec![];
    for _ in 0..size {
        data.push(vec![]);
    }
    while start.len() > 0 {
        let line = start.pop().unwrap().chars().collect::<Vec<_>>();
        for i in 0..size {
            let idx = i * 4 + 1;
            let ch = line[idx];
            if ch != ' ' {
                data[i].push(ch);
            }
        }
    }
    println!("Start: {:?}", data);
    data
}

fn move_crates1(instructions: &str, data: &mut Vec<Vec<char>>) {
    let instructions = instructions.split("\n").collect::<Vec<_>>();
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();
    for line in instructions {
        let cap = re.captures(line).unwrap();
        let number: usize = cap.get(1).unwrap().as_str().parse().unwrap();
        let from: usize = cap.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to: usize = cap.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
        for _ in 0..number {
            let item = data[from].pop().unwrap();
            data[to].push(item);
        }
    }
    println!("After: {:?}", data);
}

fn move_crates2(instructions: &str, data: &mut Vec<Vec<char>>) {
    let instructions = instructions.split("\n").collect::<Vec<_>>();
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();
    for line in instructions {
        let cap = re.captures(line).unwrap();
        let number: usize = cap.get(1).unwrap().as_str().parse().unwrap();
        let from: usize = cap.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to: usize = cap.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let mut stack: Vec<char> = vec![];
        for _ in 0..number {
            let item = data[from].pop().unwrap();
            stack.push(item);
        }
        for item in stack.iter().rev() {
            data[to].push(*item);
        }
    }
    println!("After: {:?}", &data);
}

fn report(data: &Vec<Vec<char>>) {
    let mut answer = "".to_owned();
    for v in data {
        let item = v.last().unwrap();
        answer.push(item.clone());
    }
    println!("Answer: {:?}", answer);
}
