use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};

type PageRules = HashMap<u32, Vec<u32>>;
type Pages = Vec<Vec<u32>>;

pub fn parse(input: &str) -> (PageRules, Pages) {
    let (input, page_rules) = page_rules(input).unwrap();
    let (_, pages) = parse_pages(input).unwrap();
    (page_rules, pages)
}

fn page_rules(input: &str) -> IResult<&str, PageRules> {
    let (input, page_rules) = terminated(
        many1(terminated(
            separated_pair(complete::u32, tag("|"), complete::u32),
            newline,
        )),
        newline,
    )
    .parse(input)?;

    let mut rules: PageRules = HashMap::new();
    for (lhs, rhs) in page_rules.into_iter() {
        let mut array = match rules.get(&rhs) {
            Some(array) => array.clone(),
            None => Vec::new(),
        };
        array.push(lhs);
        rules.insert(rhs, array);
    }
    Ok((input, rules))
}

fn parse_pages(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, pages) =
        separated_list1(newline, separated_list1(tag(","), complete::u32)).parse(input)?;
    Ok((input, pages))
}
