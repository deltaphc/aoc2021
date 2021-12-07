pub fn simulate_fish(input: &str, days: usize) -> usize {
    let initial_fish = input
        .trim()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
        .into_boxed_slice();
    
        let mut fish_state = initial_fish.to_vec();
    for _ in 0..days {
        let current_len = fish_state.len(); // explicitly keep the length from before more fish are added
        for i in 0..current_len {
            match fish_state[i] {
                0 => {
                    fish_state[i] = 6;
                    fish_state.push(8);
                }
                _ => {
                    fish_state[i] -= 1;
                }
            };
        }
    }
    fish_state.len()
}

pub fn simulate_fish_fast(input: &str, days: usize) -> u64 {
    let initial_fish = input
        .trim()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
        .into_boxed_slice();

    let mut fish_totals = [0_u64; 9];
    for fish in initial_fish.iter() {
        fish_totals[*fish as usize] += 1;
    }

    for _ in 0..days {
        fish_totals.rotate_left(1);
        fish_totals[6] += fish_totals[8];
    }

    fish_totals.iter().sum()
}
