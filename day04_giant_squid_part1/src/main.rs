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
    let result = do_work(&SAMPLE_DATA);
    println!("{}", result);
}

fn do_work<const N: usize, const B: usize>(data: &InputData<N, B>) -> u32 {
    let mut first_winner: (usize, u32) = (N + 1, 0);
    for board in &data.boards {
        let new_score = get_score_for_board(&data.draw_numbers, &board);
        let (current_first_win_number, _) = first_winner;
        let (new_board_win_number, _) = new_score;
        if new_board_win_number < current_first_win_number {
            first_winner = new_score;
        }
    }

    let (_, score) = first_winner;
    score
}

fn get_score_for_board<const N: usize>(draw_numbers: &[u32; N], board: &BingoBoard) -> (usize, u32) {
    let mut called_board = CalledBoard::new();

    for i in 0..draw_numbers.len() {
        let number = draw_numbers[i];
        mark_board(number, &board, &mut called_board);
        if is_winning_board(&called_board) {
            let score = get_board_score(board, &called_board);
            let score = score * number;
            return (i, score);
        }

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

fn is_winning_board(called_board: &CalledBoard) -> bool {
    for i in 0..BOARDSIZE {
        let mut winning = true;
        for j in 0..BOARDSIZE {
            winning = winning & called_board.board[i][j];
        }

        if winning {
            return winning;
        }
    }

    for j in 0..BOARDSIZE {
        let mut winning = true;
        for i in 0..BOARDSIZE {
            winning = winning & called_board.board[i][j];
        }

        if winning {
            return winning;
        }
    }

    false
}

fn get_board_score(board: &BingoBoard, called_board: &CalledBoard) -> u32 {
    let mut score: u32 = 0;

    for i in 0..BOARDSIZE {
        for j in 0..BOARDSIZE {
            if !called_board.board[i][j] {
                score += board.board[i][j]
            }
        }
    }

    score
}