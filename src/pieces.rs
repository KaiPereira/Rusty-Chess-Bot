use chess::{Piece, Board, Color, BitBoard};
use crate::positions;

pub fn get_black_pieces(piece: Piece, board: &Board) -> u32 {
    return (board.pieces(piece) & board.color_combined(Color::Black)).popcnt();
}

pub fn get_white_pieces(piece: Piece, board: &Board) -> u32 {
    return (board.pieces(piece) & board.color_combined(Color::Black)).popcnt();
}