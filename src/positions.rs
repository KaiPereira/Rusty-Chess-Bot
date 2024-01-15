use chess::{Board, Piece, Color, BitBoard};
use crate::pieces;

pub struct Positions;

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

pub fn get_pos_eval(board: &Board) -> i32 {
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

        let white_eval = piece_pos_eval(piece_type, board, Color::White);
        let black_eval = piece_pos_eval(piece_type, board, Color::Black);

        // println!("WHITE: {white_eval} BLACK: {black_eval} PIECE: {piece_type}");

        eval += white_eval;
        eval -= black_eval;
    }

    eval
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

        let square_index = square.to_index();

        // println!("SQUARE: {square} INDEX: {square_index} PIECE: {piece} COLOR: {:?}", color);

        if color == Color::Black {
            eval += pos[square_index];
        } else {
            eval += pos.clone().into_iter().rev().collect::<Vec<i32>>()[square_index];
        }
    }

    eval
}