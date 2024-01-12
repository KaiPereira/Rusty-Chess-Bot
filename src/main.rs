mod positions;
mod material;
mod board;
mod pieces;

use chess::{Piece};

fn main() {
    let board = board::create_board();

    // println!("{:?}", material::get_material(&board));

    positions::get_pos_eval(&board);
}