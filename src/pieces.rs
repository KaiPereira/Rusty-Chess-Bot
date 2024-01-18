use chess::{Piece, Board, Color, BitBoard};
use crate::positions;

pub fn get_black_pieces(piece: Piece, board: &Board) -> u32 {
    let pieces = get_piece_bitboard(piece, board, Color::Black);
    let pieces_amount = pieces.popcnt();

    return pieces_amount;
}

pub fn get_white_pieces(piece: Piece, board: &Board) -> u32 {
    let pieces = get_piece_bitboard(piece, board, Color::White);
    let pieces_amount = pieces.popcnt();

    return pieces_amount;
}


pub fn get_piece_bitboard(piece: Piece, board: &Board, color: Color) -> BitBoard {
    let pieces = (board.pieces(piece) & board.color_combined(color));

    pieces
}


pub fn get_piece_bitboard_string(piece: Piece, board: &Board, color: Color) -> String {

    let mut piece_char: &str = "p";

    if color == Color::Black {
        piece_char = match piece {
            Piece::Pawn=>"p",
            Piece::Knight=>"k",
            Piece::Bishop=>"b",
            Piece::Rook=>"r",
            Piece::Queen=>"q",
            Piece::King=>"x",
        }
    } else {
        piece_char = match piece {
            Piece::Pawn=>"P",
            Piece::Knight=>"K",
            Piece::Bishop=>"B",
            Piece::Rook=>"R",
            Piece::Queen=>"Q",
            Piece::King=>"X",
        }
    }


    let bitboard: String = get_piece_bitboard(piece, board, color)
        .to_string()
        .replace("X", piece_char)
        .replace('\n', "")
        .chars()
        .filter(|c| !c.is_whitespace()).collect();

    bitboard
}