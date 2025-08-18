use itertools::Itertools;

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn is_nice(input: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let num_vowels = input.chars().filter(|c| vowels.contains(c)).count();
    if num_vowels < 3 {
        return false;
    }

    let illegal = ["ab", "cd", "pq", "xy"];
    let num_illegal = input
        .chars()
        .tuple_windows()
        .filter(|(a, b)| {
            let s = format!("{a}{b}");
            illegal.contains(&s.as_str())
        })
        .count();
    if num_illegal > 0 {
        return false;
    }

    input.chars().tuple_windows().any(|(a, b)| a == b)
}
