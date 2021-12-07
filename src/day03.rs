pub fn power_consumption(input: &str) -> u64 {
    let numbers: Vec<u64> = input
        .trim()
        .lines()
        .map(|line| u64::from_str_radix(line, 2).unwrap())
        .collect();

    let mut gamma = 0;
    for column in 0..12 {
        let mut zeroes = 0;
        let mut ones = 0;
        for &num in &numbers {
            let bit = (num >> (11 - column)) & 1;
            if bit == 0 {
                zeroes += 1;
            } else if bit == 1 {
                ones += 1;
            } else {
                panic!("bit was not 0 or 1");
            }
        }
        if zeroes > ones {
            gamma <<= 1;
        } else {
            gamma = (gamma << 1) | 1;
        }
    }
    let epsilon = (!gamma) & 0b1111_1111_1111;
    gamma * epsilon
}

pub fn life_support_rating(input: &str) -> u64 {
    let numbers = input
        .trim()
        .lines()
        .map(|line| u64::from_str_radix(line, 2).unwrap())
        .collect::<Vec<u64>>()
        .into_boxed_slice();
    let mut oxygen_gen_candidates = numbers.to_vec();
    let mut co2_scrubber_candidates = numbers.to_vec();

    {
        let mut column = 0;
        while oxygen_gen_candidates.len() > 1 {
            let mut zeroes = 0;
            let mut ones = 0;
            for &num in &oxygen_gen_candidates {
                let bit = (num >> (11 - column)) & 1;
                if bit == 0 {
                    zeroes += 1;
                } else if bit == 1 {
                    ones += 1;
                } else {
                    panic!("bit was not 0 or 1");
                }
            }
            let bit_keep = if zeroes > ones { 0 } else { 1 };
            oxygen_gen_candidates.retain(|x| ((x >> (11 - column)) & 1) == bit_keep);
            column += 1;
        }
    }
    {
        let mut column = 0;
        while co2_scrubber_candidates.len() > 1 {
            let mut zeroes = 0;
            let mut ones = 0;
            for &num in &co2_scrubber_candidates {
                let bit = (num >> (11 - column)) & 1;
                if bit == 0 {
                    zeroes += 1;
                } else if bit == 1 {
                    ones += 1;
                } else {
                    panic!("bit was not 0 or 1");
                }
            }
            let bit_keep = if zeroes > ones { 1 } else { 0 };
            co2_scrubber_candidates.retain(|x| ((x >> (11 - column)) & 1) == bit_keep);
            column += 1;
        }
    }

    let oxygen_generator_rating = oxygen_gen_candidates[0];
    let co2_scrubber_rating = co2_scrubber_candidates[0];
    oxygen_generator_rating * co2_scrubber_rating
}
