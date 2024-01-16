use chess::{Board, Color, BoardStatus};
use crate::positions;
use crate::material;


pub fn get_eval(board: &Board) -> i32 {
    // If the move is checkmate, give the worse possible positions
    if (board.side_to_move() == Color::White) && board.status() == BoardStatus::Checkmate {
        return std::i32::MIN;
    } else if (board.side_to_move() == Color::Black) && board.status() == BoardStatus::Checkmate {
        return std::i32::MAX;
    }

    let position: i32 = positions::get_pos_eval(board);
    let material: i32 = material::get_material(board);

    // let new_board = board.to_string();
    
    // println!("Pos: {position} Mat {material} Board: {new_board}");

    let total_eval: i32 = position + material;

    total_eval
}