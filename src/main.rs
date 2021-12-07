use std::fmt::Display;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    run_day(1, day01::count_increasing, day01::count_sliding_increasing);
    run_day(2, day02::navigate_sub, day02::navigate_sub_with_aim);
    run_day(3, day03::power_consumption, day03::life_support_rating);
    run_day(
        4,
        day04::winning_board_score,
        day04::last_winning_board_score,
    );
    run_day(
        5,
        |input| day05::count_overlapping_points(input, false),
        |input| day05::count_overlapping_points(input, true),
    );
    run_day(
        6,
        |input| day06::simulate_fish(input, 80),
        |input| day06::simulate_fish_fast(input, 256),
    );
    run_day(
        7,
        |input| day07::lowest_fuel_cost(input, false),
        |input| day07::lowest_fuel_cost(input, true));
}

fn run_day<P1, P2, A, B>(day: u8, part1: P1, part2: P2)
where
    P1: Fn(&str) -> A,
    P2: Fn(&str) -> B,
    A: Display,
    B: Display,
{
    let input = std::fs::read_to_string(format!("input/day{:02}.txt", day)).unwrap();
    println!("Day {}:\n{}\n{}\n", day, part1(&input), part2(&input));
}
