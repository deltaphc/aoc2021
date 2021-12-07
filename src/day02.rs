pub fn navigate_sub(input: &str) -> i64 {
    let mut position = 0;
    let mut depth = 0;
    for line in input.trim().lines() {
        let mut line_split_iter = line.split_whitespace();
        let (cmd, arg) = (
            line_split_iter.next().unwrap(),
            line_split_iter.next().unwrap().parse::<i64>().unwrap(),
        );
        match cmd {
            "forward" => position += arg,
            "up" => depth -= arg,
            "down" => depth += arg,
            _ => panic!("wut"),
        }
    }

    position * depth
}

pub fn navigate_sub_with_aim(input: &str) -> i64 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input.trim().lines() {
        let mut line_split_iter = line.split_whitespace();
        let (cmd, arg) = (
            line_split_iter.next().unwrap(),
            line_split_iter.next().unwrap().parse::<i64>().unwrap(),
        );
        match cmd {
            "forward" => {
                position += arg;
                depth += aim * arg;
            }
            "up" => aim -= arg,
            "down" => aim += arg,
            _ => panic!("wut"),
        }
    }

    position * depth
}
