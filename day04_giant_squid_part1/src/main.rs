#[allow(unused_imports)]
use day04_giant_squid_common::{BOARDSIZE, BingoBoard, InputData, SAMPLE_DATA};

struct CalledBoard {
    pub board: [[bool; BOARDSIZE]; BOARDSIZE]
}

impl CalledBoard {
    fn new() -> CalledBoard {
        CalledBoard { board: [[false; BOARDSIZE]; BOARDSIZE] }
    }
}

fn main() {
    let result = do_work(SAMPLE_DATA);
    println!("{}", result);
}

fn do_work<const N: usize, const B: usize>(data: InputData<N, B>) -> u32 {
    2
}

fn get_score_for_board<const N: usize>(draw_numbers: [u32; N], board: BingoBoard) -> (usize, u32) {
    let mut called_board = CalledBoard::new();

    for number in draw_numbers {
        mark_board(number, &board, &mut called_board);
    }
    (0, 0)
}

fn mark_board(drawn_number: u32, board: &BingoBoard, called_board: &mut CalledBoard) {
    for i in 0..BOARDSIZE {
        for j in 0..BOARDSIZE {
            if board.board[i][j] == drawn_number {
                called_board.board[i][j] = true;
            }
        }
    }
}

fn is_winning_board(called_board: &CallBoard) -> bool {
    for i in 0..BOARDSIZE {
        let mut winning = true;
        for j in 0..BOARDSIZE {
            winning = winning & called_board
        }
    }

    false
}