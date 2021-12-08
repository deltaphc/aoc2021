pub fn lowest_fuel_cost(input: &str, nonlinear: bool) -> u64 {
    let positions = input
        .trim()
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let &min = positions.iter().min().unwrap();
    let &max = positions.iter().max().unwrap();

    if nonlinear {
        (min..=max)
            .map(|pos_to_try| {
                positions
                    .iter()
                    .map(|&p| (1..=(p.max(pos_to_try) - p.min(pos_to_try))).sum::<u64>())
                    .sum::<u64>()
            })
            .min()
            .unwrap()
    } else {
        (min..=max)
            .map(|pos_to_try| {
                positions
                    .iter()
                    .map(|&p| p.max(pos_to_try) - p.min(pos_to_try))
                    .sum::<u64>()
            })
            .min()
            .unwrap()
    }
}
