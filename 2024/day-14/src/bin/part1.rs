#![allow(unused)]

use day_14::{parse, Grid};

fn process(input: &str, grid_size: (usize, usize)) -> String {
    let robots = parse(input);
    let mut grid = Grid::new(grid_size, robots);

    for _ in 0..100 {
        grid.step();
    }
    let result = grid.count();
    result.to_string()
}

fn main() {
    let file = include_str!("../../input1.txt");
    let result = process(file, (101, 103));
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"#;
        assert_eq!("12", process(input, (11, 7)));
    }
}
