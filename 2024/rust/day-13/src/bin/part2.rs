#![allow(unused)]

use day_13::{parse, Machine, Prize};

fn process(input: &str) -> String {
    let machines = parse(input);
    // 10000000000000

    let result: u64 = machines
        .into_iter()
        .map(|m| {
            let m = Machine {
                prize: Prize {
                    x: m.prize.x + 10000000000000,
                    y: m.prize.y + 10000000000000,
                },
                buttons: m.buttons,
            };
            m.solve().unwrap_or(0)
        })
        .sum();
    result.to_string()
}

fn main() {
    let file = include_str!("../../input2.txt");
    let result = process(file);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;
        assert_eq!("", process(input));
    }
}
