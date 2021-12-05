use std::fs;

const BOARD_DIM: usize = 5;

#[derive(Debug, Clone, Copy)]
struct Board {
    spots: [[i32; BOARD_DIM]; BOARD_DIM],
    marks: [[bool; BOARD_DIM]; BOARD_DIM],
}

fn str_to_row(input: &str) -> [i32; BOARD_DIM] {
    let mut a = [0; BOARD_DIM];
    input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .enumerate()
        .for_each(|(i, x)| a[i] = x);
    a
}

impl Board {
    fn new() -> Board {
        Board {
            spots: [[0; BOARD_DIM]; BOARD_DIM],
            marks: [[false; BOARD_DIM]; BOARD_DIM],
        }
    }

    fn add_row(&self, row: [i32; BOARD_DIM], i: usize) -> Board {
        let mut spots = self.spots;
        spots[i] = row;
        Board {
            marks: self.marks,
            spots,
        }
    }

    fn mark(&mut self, n: i32) {
        self.spots
            .into_iter()
            .enumerate()
            .for_each(|(row_idx, row)| {
                if let Some(i) = row.into_iter().position(|x| x == n) {
                    self.marks[row_idx][i] = true
                }
            })
    }

    fn is_winner(&self) -> bool {
        let is_row_winner = self
            .marks
            .iter()
            .map(|row| row.iter().all(|x| *x))
            .any(|x| x);

        let is_col_winner = (0..BOARD_DIM)
            .map(|col_idx| self.marks.iter().map(|row| row[col_idx]).all(|x| x))
            .any(|x| x);

        is_row_winner || is_col_winner
    }

    fn board_score(&self) -> i32 {
        let mut board_score = 0;
        for row in 0..BOARD_DIM {
            for col in 0..BOARD_DIM {
                if !self.marks[row][col] {
                    board_score += self.spots[row][col];
                }
            }
        }
        board_score
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut input_iter = contents.lines();

    let numbers: Vec<i32> = input_iter
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    input_iter.next(); // skip first empty line

    let boards: (Vec<Board>, Board, usize) =
        input_iter.fold((vec![], Board::new(), 0), |acc, curr| {
            if curr.is_empty() {
                (
                    acc.0.into_iter().chain(vec![acc.1].into_iter()).collect(),
                    Board::new(),
                    0,
                )
            } else {
                (
                    acc.0,
                    acc.1.add_row(str_to_row(curr), acc.2),
                    (acc.2 + 1) % BOARD_DIM,
                )
            }
        });

    // Part One
    let mut all_boards_one: Vec<Board> = boards
        .0
        .clone()
        .into_iter()
        .chain(vec![boards.1].into_iter())
        .collect();

    'outer_one: for n in numbers.clone() {
        for b in &mut all_boards_one {
            b.mark(n);
            if b.is_winner() {
                println!("One: {:?}", b.board_score() * n);
                break 'outer_one;
            }
        }
    }

    // Part Two
    let mut all_boards_two: Vec<Board> = boards
        .0
        .into_iter()
        .chain(vec![boards.1].into_iter())
        .collect();

    'outer_two: for n in numbers {
        for b in &mut all_boards_two {
            b.mark(n);
        }
        if all_boards_two.len() > 1 {
            all_boards_two = all_boards_two
                .into_iter()
                .filter(|b| !b.is_winner())
                .collect();
        } else {
            let last_board = all_boards_two.get(0).unwrap();
            if last_board.is_winner() {
                println!("Two: {:?}", last_board.board_score() * n);
                break 'outer_two;
            }
        }
    }
}
