mod positions;
mod material;
mod board;
mod pieces;
mod evaluation;
mod search;
mod uci;
use chess::{Piece, BoardStatus, Board, MoveGen};


fn main() {

    uci::start();


    // let mut board: Board = board::create_board();


    // while (board.status() != BoardStatus::Checkmate) || (board.status() != BoardStatus::Stalemate) {
    //     if let Some(best_move) = search::get_move(&board, 4, true) {
    //         board = board.make_move_new(best_move);
    //         board::display(&board);
    //     } else {
    //         println!("CHECKMATE!");
    //         board::display(&board);
    //         break;
    //     }

    //     println!("{:?}", board.to_string())
    // }


    // let board = Board::from_fen("r1bqk2r/pppp1ppp/2n1pn2/3N4/1b2P3/3B1N2/PPPP1PPP/R1BQK2R b KQkq - 0 1".to_string()).unwrap();
    // let board = Board::from_fen("r1b1kb1r/ppp2ppp/8/8/4q3/8/PPPB1PPP/R2QK2R w KQkq - 0 1".to_string()).unwrap();


    // let eval = evaluation::get_eval(&board);
    // println!("eval: {eval}");


    
    // if let Some(best_move) = search::get_move(&board, 3, true) {
    //     println!("BEST MOVE: {best_move}")
    // } else {
    //     println!("CHECKMATE!");
    // }
}