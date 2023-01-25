// Learned using nom and BTreeMap by following the code from Christopher Biscardi
// See https://github.com/ChristopherBiscardi/advent-of-code/tree/main/2022/rust/day-07

use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
enum Operation<'a> {
    CdRoot,
    CdUp,
    CdDowm(&'a str),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug)]
enum Files<'a> {
    File(u32),
    Dir(&'a str),
}

fn commands(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, cmds) = separated_list1(newline, alt((cd, ls)))(input)?;
    Ok((input, cmds))
}

fn cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag("/"), tag(".."), alpha1))(input)?;
    let op = match dir {
        "/" => Operation::CdRoot,
        ".." => Operation::CdUp,
        name => Operation::CdDowm(name),
    };
    Ok((input, op))
}

fn ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((file, dir)))(input)?;
    Ok((input, Operation::Ls(files)))
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, (size, _name)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        is_a("qwertyuioplkjhgfdsazxcvbnm."),
    )(input)?;
    Ok((input, Files::File(size)))
}

fn dir(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, Files::Dir(name)))
}

fn calculate_sizes<'a>(
    (mut context, mut sizes): (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>),
    cmd: &'a Operation,
) -> (Vec<&'a str>, BTreeMap<Vec<&'a str>, u32>) {
    match cmd {
        Operation::CdRoot => {
            context.push("");
        }
        Operation::CdUp => {
            context.pop();
        }
        Operation::CdDowm(value) => {
            context.push(value);
        }
        Operation::Ls(files) => {
            let sum: u32 = files
                .iter()
                .filter_map(|f| {
                    if let Files::File(size) = f {
                        Some(size)
                    } else {
                        None
                    }
                })
                .sum();
            for i in 0..context.len() {
                sizes
                    .entry(context[0..=i].to_vec())
                    .and_modify(|s| *s += sum)
                    .or_insert(sum);
            }
        }
    };
    (context, sizes)
}

fn main() {
    println!("{}", do_part_1(include_str!("../files/d07")));
    println!("{}", do_part_2(include_str!("../files/d07")));
}

fn do_part_1(input: &str) -> u32 {
    let (_, cmds) = commands(input).unwrap();
    let (_, sizes) = cmds.iter().fold((vec![], BTreeMap::new()), calculate_sizes);
    sizes
        .iter()
        .map(|(_, &size)| size)
        .filter(|&s| s <= 100000)
        .sum::<u32>()
}

fn do_part_2(input: &str) -> u32 {
    let (_, cmds) = commands(input).unwrap();
    let (_, sizes) = cmds.iter().fold((vec![], BTreeMap::new()), calculate_sizes);
    let outermost = &sizes[&[""].to_vec()];
    let available = 70000000 - outermost;
    let required = 30000000 - available;
    let mut result = sizes
        .iter()
        .map(|(_, &size)| size)
        .filter(|&s| s >= required)
        .collect::<Vec<_>>();
    result.sort();
    result[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let result = do_part_1(include_str!("../files/d07_example"));
        assert_eq!(result, 95437);
    }

    #[test]
    fn part2() {
        let result = do_part_2(include_str!("../files/d07_example"));
        assert_eq!(result, 24933642);
    }
}
