use crate::board::{Board, Error};
use crate::consts::{COLS, ROWS};
use crate::player::Player;

pub fn evaluate_board<B: Board<Error>>(board: &B, player: Player) -> i32 {
    let mut opponent = player.clone();
    opponent.switch();

    let mut score = 0;

    // Evaluate horizontal sequences
    for row in 0..ROWS {
        for col in 0..COLS - 3 {
            score += evaluate_sequence(board, row, col, player, opponent);
        }
    }

    // Evaluate vertical sequences
    for col in 0..COLS {
        for row in 0..ROWS - 3 {
            score += evaluate_sequence(board, row, col, player, opponent);
        }
    }

    // Evaluate diagonal sequences (bottom-left to top-right)
    for row in 0..ROWS - 3 {
        for col in 0..COLS - 3 {
            score += evaluate_sequence(board, row, col, player, opponent);
        }
    }

    // Evaluate diagonal sequences (top-left to bottom-right)
    for row in 3..ROWS {
        for col in 0..COLS - 3 {
            score += evaluate_sequence(board, row, col, player, opponent);
        }
    }

    score
}

fn evaluate_sequence<B: Board<Error>>(
    // i have no idea if this works or not
    board: &B,
    row: usize,
    col: usize,
    player: Player,
    opponent: Player,
) -> i32 {
    let mut player_count = 0;
    let mut opponent_count = 0;

    let mut score = 0;

    for offset in 0..4 {
        match board.get(row + offset, col + offset) {
            Ok(Some(p)) if p == player => player_count += 1,
            Ok(Some(p)) if p == opponent => opponent_count += 1,
            _ => (),
        }
    }

    if player_count == 3 && opponent_count == 0 {
        score += 10000; // Winning move
    } else if opponent_count == 3 && player_count == 0 {
        score += 100000; // blocking opponent's winning move please for the love of god do this please
    } else if player_count == 2 && opponent_count == 0 {
        score += 100; // Connecting 2 in a row
    } else if opponent_count == 2 && player_count == 0 {
        score += 50; // Preventing opponent's 2 in a row
    }

    score
}

pub fn minimax<B: Board<Error>>(board: &mut B, max_depth: usize, mut current_player: Player) -> (i32, Option<usize>) {
    let mut best_move = None;
    let mut best_eval = std::i32::MIN;

    for col in 0..COLS {
        if board.is_valid(col).unwrap() {
            let mut board_copy = board.clone();
            if let Ok(_) = board_copy.make_move(col, current_player) {
                let eval = minimax_recursive(
                    &mut board_copy,
                    max_depth - 1,
                    current_player.switch(),
                    best_eval
                );

                if eval > best_eval {
                    best_eval = eval;
                    best_move = Some(col);
                }
            }
        }
    }

    (best_eval, best_move)
}

fn minimax_recursive<B: Board<Error>>(
    board: &mut B,
    depth: usize,
    mut current_player: Player,
    mut best_eval: i32
) -> i32 {
    if depth == 0 {
        return evaluate_board(board, current_player);
    }

    for col in 0..COLS {
        if board.is_valid(col).unwrap() {
            let mut board_copy = board.clone();
            if let Ok(_) = board_copy.make_move(col, current_player) {
                let mut eval =
                    -minimax_recursive(&mut board_copy, depth - 1, current_player.switch(), best_eval);

                if board_copy.check_winner() {
                    eval -= 100000; // punish his ass because this bot is blind
                }

                best_eval = best_eval.max(eval);
            }
        }
    }

    best_eval // me fr
}
