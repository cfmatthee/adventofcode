#![allow(unused)]

use day_06::{convert_position, get_positions, parse, Instuction};

fn process(input: &str) -> String {
    let input = parse(input);
    let mut lights = [false; 1000000];
    for instruction in input {
        let (start, stop) = match instruction {
            Instuction::TurnOff(start, stop) => (start, stop),
            Instuction::TurnOn(start, stop) => (start, stop),
            Instuction::Toggle(start, stop) => (start, stop),
        };
        let positions = get_positions(start, stop);
        for pos in positions {
            let pos = convert_position(pos);
            match instruction {
                Instuction::TurnOff(_, _) => lights[pos] = false,
                Instuction::TurnOn(_, _) => lights[pos] = true,
                Instuction::Toggle(_, _) => lights[pos] = !lights[pos],
            }
        }
    }

    let result = lights.iter().filter(|&&state| state).count();
    result.to_string()
}

fn main() {
    let file = include_str!("../../input1.txt");
    let result = process(file);
    println!("{result}");
}
