use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn read_lines(input: &str) -> Vec<Line> {
    input
        .trim()
        .lines()
        .map(|input_line| {
            let (left_str, right_str) = input_line.split_once(" -> ").unwrap();

            let (x1_str, y1_str) = left_str.split_once(',').unwrap();
            let (x1, y1): (i32, i32) = (x1_str.parse().unwrap(), y1_str.parse().unwrap());

            let (x2_str, y2_str) = right_str.split_once(',').unwrap();
            let (x2, y2): (i32, i32) = (x2_str.parse().unwrap(), y2_str.parse().unwrap());

            Line { x1, y1, x2, y2 }
        })
        .collect()
}

fn touch_cell(grid: &mut HashMap<(i32, i32), i32>, x: i32, y: i32) {
    grid.entry((x, y)).and_modify(|n| *n += 1).or_insert(1);
}

pub fn count_overlapping_points(input: &str, diagonals: bool) -> i32 {
    let lines = read_lines(input);
    let mut grid = HashMap::new();

    for line in &lines {
        if line.y1 == line.y2 {
            // horizontal line
            for x in line.x1.min(line.x2)..=line.x1.max(line.x2) {
                touch_cell(&mut grid, x, line.y1);
            }
        } else if line.x1 == line.x2 {
            // vertical line
            for y in line.y1.min(line.y2)..=line.y1.max(line.y2) {
                touch_cell(&mut grid, line.x1, y);
            }
        } else if diagonals {
            // diagonal line
            let mut x = line.x1;
            let mut y = line.y1;
            let x_inc = if line.x1 < line.x2 { 1 } else { -1 };
            let y_inc = if line.y1 < line.y2 { 1 } else { -1 };

            while x != line.x2 && y != line.y2 {
                touch_cell(&mut grid, x, y);
                x += x_inc;
                y += y_inc;
            }
            touch_cell(&mut grid, x, y); // touch the last position
        }
    }

    grid.values().filter(|&&n| n >= 2).count() as i32
}
