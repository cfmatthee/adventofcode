use std::fs;

#[derive(Debug)]
enum Element {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Result {
    Win,
    Loose,
    Draw,
}

fn main() {
    let data = fs::read_to_string("src/files/d02").unwrap();
    println!("{:?}", part1(&data));
    println!("{:?}", part2(&data));
}

fn part1(data: &str) -> u32 {
    let slices: Vec<_> = data
        .split("\n")
        .map(|s| {
            let opponent = match &s.chars().nth(0).unwrap() {
                'A' => Element::Rock,
                'B' => Element::Paper,
                'C' => Element::Scissors,
                _ => panic!("Illegal symbol"),
            };
            let me = match &s.chars().nth(2).unwrap() {
                'X' => Element::Rock,
                'Y' => Element::Paper,
                'Z' => Element::Scissors,
                _ => panic!("Illegal symbol"),
            };
            (opponent, me)
        })
        .map(|(opponent, me)| {
            let result = match (&opponent, &me) {
                (Element::Rock, Element::Rock) => Result::Draw,
                (Element::Rock, Element::Paper) => Result::Win,
                (Element::Rock, Element::Scissors) => Result::Loose,
                (Element::Paper, Element::Rock) => Result::Loose,
                (Element::Paper, Element::Paper) => Result::Draw,
                (Element::Paper, Element::Scissors) => Result::Win,
                (Element::Scissors, Element::Rock) => Result::Win,
                (Element::Scissors, Element::Paper) => Result::Loose,
                (Element::Scissors, Element::Scissors) => Result::Draw,
            };
            (opponent, me, result)
        })
        .map(|(opponent, me, result)| {
            let shapescore = match &me {
                Element::Rock => 1,
                Element::Paper => 2,
                Element::Scissors => 3,
            };
            let roundscore = match &result {
                Result::Win => 6,
                Result::Loose => 0,
                Result::Draw => 3,
            };
            (opponent, me, result, shapescore + roundscore)
        })
        .collect();

    slices.iter().map(|data| data.3).sum::<u32>()
}

fn part2(data: &str) -> u32 {
    let slices: Vec<_> = data
        .split("\n")
        .map(|s| {
            let opponent = match &s.chars().nth(0).unwrap() {
                'A' => Element::Rock,
                'B' => Element::Paper,
                'C' => Element::Scissors,
                _ => panic!("Illegal symbol"),
            };
            let result = match &s.chars().nth(2).unwrap() {
                'X' => Result::Loose,
                'Y' => Result::Draw,
                'Z' => Result::Win,
                _ => panic!("Illegal symbol"),
            };
            (opponent, result)
        })
        .map(|(opponent, result)| {
            let me = match (&opponent, &result) {
                (Element::Rock, Result::Win) => Element::Paper,
                (Element::Rock, Result::Loose) => Element::Scissors,
                (Element::Rock, Result::Draw) => Element::Rock,
                (Element::Paper, Result::Win) => Element::Scissors,
                (Element::Paper, Result::Loose) => Element::Rock,
                (Element::Paper, Result::Draw) => Element::Paper,
                (Element::Scissors, Result::Win) => Element::Rock,
                (Element::Scissors, Result::Loose) => Element::Paper,
                (Element::Scissors, Result::Draw) => Element::Scissors,
            };
            (opponent, me, result)
        })
        .map(|(opponent, me, result)| {
            let shapescore = match &me {
                Element::Rock => 1,
                Element::Paper => 2,
                Element::Scissors => 3,
            };
            let roundscore = match &result {
                Result::Win => 6,
                Result::Loose => 0,
                Result::Draw => 3,
            };
            (opponent, me, result, shapescore + roundscore)
        })
        .collect();

    slices.iter().map(|data| data.3).sum::<u32>()
}
