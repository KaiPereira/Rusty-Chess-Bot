use chess::{Board, Piece, Color, BitBoard};
use crate::pieces;

pub struct Positions {

}

impl Positions {
    pub fn pawn() -> Vec<i32> {
        return [
            0,  0,  0,  0,  0,  0,  0,  0,
            50, 50, 50, 50, 50, 50, 50, 50,
            10, 10, 20, 30, 30, 20, 10, 10,
            5,  5, 10, 25, 25, 10,  5,  5,
            0,  0,  0, 20, 20,  0,  0,  0,
            5, -5,-10,  0,  0,-10, -5,  5,
            5, 10, 10,-20,-20, 10, 10,  5,
            0,  0,  0,  0,  0,  0,  0,  0
        ].to_vec();
    }

    pub fn knight() -> Vec<i32> {
        return [
            -50,-40,-30,-30,-30,-30,-40,-50,
            -40,-20,  0,  0,  0,  0,-20,-40,
            -30,  0, 10, 15, 15, 10,  0,-30,
            -30,  5, 15, 20, 20, 15,  5,-30,
            -30,  0, 15, 20, 20, 15,  0,-30,
            -30,  5, 10, 15, 15, 10,  5,-30,
            -40,-20,  0,  5,  5,  0,-20,-40,
            -50,-40,-30,-30,-30,-30,-40,-50,
        ].to_vec();
    }

    pub fn bishop() -> Vec<i32> {
        return [
            -20,-10,-10,-10,-10,-10,-10,-20,
            -10,  0,  0,  0,  0,  0,  0,-10,
            -10,  0,  5, 10, 10,  5,  0,-10,
            -10,  5,  5, 10, 10,  5,  5,-10,
            -10,  0, 10, 10, 10, 10,  0,-10,
            -10, 10, 10, 10, 10, 10, 10,-10,
            -10,  5,  0,  0,  0,  0,  5,-10,
            -20,-10,-10,-10,-10,-10,-10,-20,
        ].to_vec();
    }

    pub fn rook() -> Vec<i32> {
        return [
            0,  0,  0,  0,  0,  0,  0,  0,
            5, 10, 10, 10, 10, 10, 10,  5,
            -5,  0,  0,  0,  0,  0,  0, -5,
            -5,  0,  0,  0,  0,  0,  0, -5,
            -5,  0,  0,  0,  0,  0,  0, -5,
            -5,  0,  0,  0,  0,  0,  0, -5,
            -5,  0,  0,  0,  0,  0,  0, -5,
            0,  0,  0,  5,  5,  0,  0,  0
        ].to_vec();
    }

    pub fn queen() -> Vec<i32> {
        return [
            -20,-10,-10, -5, -5,-10,-10,-20,
            -10,  0,  0,  0,  0,  0,  0,-10,
            -10,  0,  5,  5,  5,  5,  0,-10,
            -5,  0,  5,  5,  5,  5,  0, -5,
            0,  0,  5,  5,  5,  5,  0, -5,
            -10,  5,  5,  5,  5,  5,  0,-10,
            -10,  0,  5,  0,  0,  0,  0,-10,
            -20,-10,-10, -5, -5,-10,-10,-20
        ].to_vec();
    }

    pub fn king() -> Vec<i32> {
        return [
            -30,-40,-40,-50,-50,-40,-40,-30,
            -30,-40,-40,-50,-50,-40,-40,-30,
            -30,-40,-40,-50,-50,-40,-40,-30,
            -30,-40,-40,-50,-50,-40,-40,-30,
            -20,-30,-30,-40,-40,-30,-30,-20,
            -10,-20,-20,-20,-20,-20,-20,-10,
            20, 20,  0,  0,  0,  0, 20, 20,
            20, 30, 10,  0,  0, 10, 30, 20
        ].to_vec();
    }
}

pub fn get_pos_eval(board: &Board) {
    // Get all the pieces and their type
    // let black_pawn = pieces::get_black_pieces(Piece::Pawn, board);
    // let black_knight = pieces::get_black_pieces(Piece::Knight, board);
    // let black_bishop = pieces::get_black_pieces(Piece::Bishop, board);
    // let black_rook = pieces::get_black_pieces(Piece::Rook, board);
    // let black_queen = pieces::get_black_pieces(Piece::Queen, board);
    // let black_king = pieces::get_black_pieces(Piece::King, board);

    // let black_pawn = pieces::get_black_pieces(Piece::Pawn, board);
    // let black_knight = pieces::get_black_pieces(Piece::Knight, board);
    // let black_bishop = pieces::get_black_pieces(Piece::Bishop, board);
    // let black_rook = pieces::get_black_pieces(Piece::Rook, board);
    // let black_queen = pieces::get_black_pieces(Piece::Queen, board);
    // let black_king = pieces::get_black_pieces(Piece::King, board);

    // We want a mappable array of every piece type and what square they're on
    // First map over all pieces
    let pieces = ["pawn", "knight", "bishop", "rook", "queen", "king"];
    let mut piece_type = Piece::Pawn;
    let mut eval = 0;

    for piece in pieces {
        match piece {
            "pawn" => piece_type = Piece::Pawn,
            "knight" => piece_type = Piece::Knight,
            "bishop" => piece_type = Piece::Bishop,
            "rook" => piece_type = Piece::Rook,
            "queen" => piece_type = Piece::Queen,
            "king" => piece_type = Piece::King,
            _ => piece_type = Piece::Pawn
        };

        eval += piece_pos_eval(piece_type, board, Color::White);
        piece_pos_eval(piece_type, board, Color::Black);
    }
}

// Make another function to get all their squares too and put it in a vector
pub fn piece_pos_eval(piece: Piece, board: &Board, color: Color) -> i32 {
    let bitboard: BitBoard = (board.pieces(piece) & board.color_combined(color));
    let mut eval = 0;
    let mut pos = Positions::pawn();

    match piece {
        Piece::Pawn => pos = Positions::pawn(),
        Piece::Knight => pos = Positions::knight(),
        Piece::Bishop => pos = Positions::bishop(),
        Piece::Rook => pos = Positions::rook(),
        Piece::King => pos = Positions::king(),
        Piece::Queen => pos = Positions::queen(),
    }

    for square in bitboard {
        eval += pos[square.to_index()];
    }

    eval
}