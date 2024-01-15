use chess::{Piece, Board, Color, BitBoard};
use crate::positions;

pub fn get_black_pieces(piece: Piece, board: &Board) -> u32 {
    let pieces = (board.pieces(piece) & board.color_combined(Color::Black));
    let pieces_amount = pieces.popcnt();

    return pieces_amount;
}

pub fn get_white_pieces(piece: Piece, board: &Board) -> u32 {
    let pieces = (board.pieces(piece) & board.color_combined(Color::White));
    let pieces_amount = pieces.popcnt();

    return pieces_amount;
}