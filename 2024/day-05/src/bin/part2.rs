#![allow(unused)]

use std::cmp::Ordering;

use day_05::parse;

fn process(input: &str) -> String {
    let (rules, page_options) = parse(input);
    let result = page_options
        .iter()
        .filter(|pages| {
            // find all invalid groups
            let copy = (*pages).clone();
            for idx in 1..pages.len() {
                let (page, remainder) = copy.split_at(idx);
                let page = page.last().unwrap();
                if let Some(rule) = rules.get(page) {
                    let has_error = remainder.iter().any(|p| rule.contains(p));
                    if has_error {
                        return true;
                    }
                }
            }
            false
        })
        .map(|pages| {
            // sort the pages
            let mut pages = pages.clone();
            pages.sort_by(|a, b| {
                if let Some(rule) = rules.get(a) {
                    if rule.contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                } else {
                    Ordering::Equal
                }
            });
            pages
        })
        .map(|pages| {
            let middle = (pages.len() - 1) / 2;
            pages[middle]
        })
        .sum::<u32>();
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
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        assert_eq!("123", process(input));
    }
}
