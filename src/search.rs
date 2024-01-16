use std::string;

use chess::{Board, MoveGen, Square, ChessMove, Color, BoardStatus};
use crate::evaluation;


// Minimax without alpha beta pruning
pub fn search(board: &Board, depth: i32, maximizing: bool, mut alpha: i32, mut beta: i32) -> i32 {
    if (depth == 0) || (board.status() == BoardStatus::Checkmate) {
        return evaluation::get_eval(board);
    }

    if maximizing {
        let mut max_eval = std::i32::MIN;

        for chess_move in MoveGen::new_legal(board) {
            let mut new_board = board.clone();
            let new_new_board = new_board.make_move_new(chess_move);

            let eval = search(&new_new_board, depth - 1, false, alpha, beta);
            max_eval = max_eval.max(eval);

            alpha = alpha.max(max_eval);

            if beta <= alpha {
                break;
            };
        }

        max_eval
    } else {
        let mut min_eval = std::i32::MAX;

        for chess_move in MoveGen::new_legal(board) {
            let mut new_board = board.clone();
            let new_new_board = new_board.make_move_new(chess_move);

            let eval = search(&new_new_board, depth - 1, true, alpha, beta);  // Pass true for maximizing in the recursive call
            min_eval = min_eval.min(eval);

            beta = beta.min(min_eval);

            if beta <= alpha {
                break;
            };
        }

        min_eval
    }
}


pub fn get_move(board: &Board, depth: i32, maximizing: bool) -> Option<ChessMove> {
    let mut best_move = None;

    let alpha = std::i32::MIN;
    let beta = std::i32::MAX;


    let current_color = board.side_to_move();
    

    let mut best_eval = if current_color == Color::White {
        std::i32::MIN
    } else {
        std::i32::MAX
    };


    let starter_move = ChessMove::new(Square::E2, Square::E4, None);

    for boardMove in MoveGen::new_legal(board) {
        if boardMove == starter_move {
            return Some(starter_move)
        }
    }

    for boardMove in MoveGen::new_legal(board) {

        let new_board = board.clone();
        let new_move = new_board.make_move_new(boardMove);

        let eval = search(&new_move, depth, maximizing, alpha, beta);

        println!("Move: {boardMove} Eval: {eval}");

        if (current_color == Color::White && eval > best_eval)
            || (current_color == Color::Black && eval < best_eval)
        {
            best_eval = eval;
            best_move = Some(boardMove);
        }
    }
    

    best_move
}