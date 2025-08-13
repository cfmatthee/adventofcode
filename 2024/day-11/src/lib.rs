pub fn blink(input: Vec<usize>) -> Vec<usize> {
    input
        .into_iter()
        .flat_map(|num| {
            if num == 0 {
                return vec![1];
            }
            let num_digits = num.ilog10() + 1;
            if num_digits % 2 == 0 {
                let div: usize = 10_usize.pow(num_digits / 2);
                return vec![num / div, num % div];
            }
            vec![num * 2024]
        })
        .collect::<Vec<_>>()
}
