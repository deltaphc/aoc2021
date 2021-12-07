#[derive(Debug, Clone, Default)]
struct BingoBoard {
    board: [[u64; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl BingoBoard {
    fn mark(&mut self, mark_num: u64) {
        for (y, row) in self.board.iter().enumerate() {
            for (x, num) in row.iter().enumerate() {
                if *num == mark_num {
                    self.marked[y][x] = true;
                    return;
                }
            }
        }
    }

    fn is_win(&self) -> bool {
        // check horizontal wins
        for row in self.marked.iter() {
            if row.iter().all(|x| *x) {
                return true;
            }
        }

        // check vertical wins
        for x in 0..5 {
            let column = [self.marked[0][x], self.marked[1][x], self.marked[2][x], self.marked[3][x], self.marked[4][x] ];
            if column.iter().all(|x| *x) {
                return true;
            }
        }

        false
    }
}

fn make_bingo_data(input: &str) -> (Vec<u64>, Vec<BingoBoard>) {
    let mut sections_iter = input.trim().split("\n\n");

    let numbers_to_draw: Vec<u64> = sections_iter.next().unwrap().split(',').map(|s| s.parse::<u64>().unwrap()).collect();
    let mut boards = Vec::new();
    for board_section in sections_iter {
        let mut num_iter = board_section.split_whitespace().map(|s| s.parse::<u64>().unwrap());
        boards.push(BingoBoard {
            board: [ // hey. if it works, it works. don't judge me
                [num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap()],
                [num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap()],
                [num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap()],
                [num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap()],
                [num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap(), num_iter.next().unwrap()],
            ],
            marked: [[false; 5]; 5],
        });
    }
    (numbers_to_draw, boards)
}

pub fn winning_board_score(input: &str) -> u64 {
    let (numbers_to_draw, mut boards) = make_bingo_data(input);
    
    let mut score = 0;
    'outer: for &num in &numbers_to_draw {
        for board in &mut boards {
            board.mark(num);
            if board.is_win() {
                let mut sum = 0;
                for y in 0..5 {
                    for x in 0..5 {
                        if !board.marked[y][x] {
                            sum += board.board[y][x];
                        }
                    }
                }
                score = num * sum;
                break 'outer;
            }
        }
    }
    score
}

pub fn last_winning_board_score(input: &str) -> u64 {
    let (numbers_to_draw, mut boards) = make_bingo_data(input);
    let mut last_winning_board = BingoBoard::default();
    let mut last_winning_num = 0;
    
    for &num in &numbers_to_draw {
        for board in &mut boards {
            // we don't want to check boards that were already a verified winner
            if board.is_win() { continue; }
            
            board.mark(num);
            if board.is_win() {
                last_winning_board = board.clone();
                last_winning_num = num;
            }
        }
    }
    
    let mut sum = 0;
    for y in 0..5 {
        for x in 0..5 {
            if !last_winning_board.marked[y][x] {
                sum += last_winning_board.board[y][x];
            }
        }
    }
    last_winning_num * sum
}
