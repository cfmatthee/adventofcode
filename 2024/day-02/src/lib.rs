pub fn is_valid(items: Vec<usize>) -> bool {
    let partial = items[0..2].to_vec();
    let [first, second] = partial.as_slice() else {
        return false;
    };
    if first == second {
        return false;
    }
    let faults = if second > first {
        // increasing
        items
            .windows(2)
            .filter(|pair| {
                let i = *pair.first().unwrap();
                let j = *pair.get(1).unwrap();
                j <= i || j > i + 3
            })
            .count()
    } else {
        // decreasing
        items
            .windows(2)
            .filter(|pair| {
                let i = *pair.first().unwrap();
                let j = *pair.get(1).unwrap();
                j >= i || i > j + 3
            })
            .count()
    };
    faults == 0
}
