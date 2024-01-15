use chess::{Board, Piece};
use crate::pieces;

pub fn get_material(board: &Board) -> i32 {
    let black_pawn = pieces::get_black_pieces(Piece::Pawn, board) * 100;
    let black_knight = pieces::get_black_pieces(Piece::Knight, board) * 310; 
    let black_bishop = pieces::get_black_pieces(Piece::Bishop, board) * 340;
    let black_rook = pieces::get_black_pieces(Piece::Rook, board) * 500;
    let black_queen = pieces::get_black_pieces(Piece::Queen, board) * 900;
    let black_king = pieces::get_black_pieces(Piece::King, board) * 9999;

    let white_pawn = pieces::get_white_pieces(Piece::Pawn, board) * 100;
    let white_knight = pieces::get_white_pieces(Piece::Knight, board) * 310; 
    let white_bishop = pieces::get_white_pieces(Piece::Bishop, board) * 340;
    let white_rook = pieces::get_white_pieces(Piece::Rook, board) * 500;
    let white_queen = pieces::get_white_pieces(Piece::Queen, board) * 900;
    let white_king = pieces::get_white_pieces(Piece::King, board) * 9999;
    
    
    let white_sum = white_pawn + white_bishop + white_king + white_knight + white_queen + white_rook;
    let black_sum = black_bishop + black_king + black_knight + black_pawn + black_queen + black_rook;


    // println!("white: {white_sum} black: {black_sum}");

    let total_material = (white_sum as i32) - (black_sum as i32);
    
    total_material
}