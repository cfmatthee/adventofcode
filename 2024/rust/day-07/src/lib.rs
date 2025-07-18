#[derive(Debug)]
pub struct Data {
    pub total: u64,
    pub numbers: Vec<u64>,
}

pub fn parse(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|line| {
            let line: Vec<&str> = line.split(':').collect();
            let total = line[0].parse::<u64>().unwrap();
            let numbers = line[1]
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            Data { total, numbers }
        })
        .collect::<Vec<_>>()
}

pub fn calculate_possible_answers(totals: Vec<u64>, numbers: &[u64], num_ops: u8) -> Vec<u64> {
    if numbers.is_empty() {
        return totals;
    }

    let (next_number, numbers) = numbers.split_first().unwrap();
    if totals.is_empty() {
        return calculate_possible_answers(vec![*next_number], numbers, num_ops);
    }

    let totals = totals
        .iter()
        .flat_map(|total| {
            let new_totals = (0..num_ops)
                .map(|idx| match idx {
                    0 => total + next_number,
                    1 => total * next_number,
                    2 => (total.to_string() + &next_number.to_string())
                        .parse::<u64>()
                        .unwrap(),
                    _ => panic!("no such option"),
                })
                .collect::<Vec<_>>();
            new_totals
        })
        .collect::<Vec<_>>();
    calculate_possible_answers(totals, numbers, num_ops)
}
