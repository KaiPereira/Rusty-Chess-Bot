use chess::{Board};
use crate::positions;
use crate::material;


pub fn get_eval(board: &Board) -> i32 {
    let position = positions::get_pos_eval(board);
    let material = material::get_material(board);

    let new_board = board.to_string();
    
    // println!("Pos: {position} Mat {material} Board: {new_board}");

    let total_eval = position + material;

    total_eval
}