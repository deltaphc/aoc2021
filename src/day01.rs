pub fn count_increasing(input: &str) -> usize {
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();
    let mut prev_number = numbers[0];
    let mut increases = 0;
    
    for &num in &numbers {
        if num > prev_number {
            increases += 1;
        }
        prev_number = num;
    }
    increases
}

pub fn count_sliding_increasing(input: &str) -> usize {
    let numbers = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();
    let mut prev_sum = numbers[0] + numbers[1] + numbers[2];
    let mut increases = 0;

    for sum in numbers.windows(3).map(|w| w.iter().sum()) {
        if sum > prev_sum {
            increases += 1
        }
        prev_sum = sum;
    }
    increases
}
