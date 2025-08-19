#![allow(unused)]

use day_06::{convert_position, get_positions, parse, Instuction};

fn process(input: &str) -> String {
    let input = parse(input);
    let mut lights = [0_u32; 1000000];
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
                Instuction::TurnOff(_, _) => {
                    if lights[pos] > 0 {
                        lights[pos] -= 1;
                    }
                }
                Instuction::TurnOn(_, _) => lights[pos] += 1,
                Instuction::Toggle(_, _) => lights[pos] += 2,
            }
        }
    }

    let result = lights.iter().sum::<u32>();
    result.to_string()
}
fn main() {
    let file = include_str!("../../input2.txt");
    let result = process(file);
    println!("{result}");
}
