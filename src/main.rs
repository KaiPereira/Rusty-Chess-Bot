mod positions;
mod material;
mod board;
mod pieces;
mod evaluation;
mod search;

use std::str::FromStr;

use chess::{Piece, BoardStatus, Board, MoveGen};

fn main() {
    let mut board: Board = board::create_board();



    while (board.status() != BoardStatus::Checkmate) || (board.status() != BoardStatus::Stalemate) {
        if let Some(best_move) = search::get_move(&board, 3, true) {
            board = board.make_move_new(best_move)
        } else {
            println!("CHECKMATE!");
            break;
        }

        println!("{:?}", board.to_string())
    }
}