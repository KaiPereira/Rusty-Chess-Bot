use chess::{Board, BitBoard, Square};


pub fn create_board() -> Board {
    let board: Board = Board::default();

    return board
}


pub fn get_bitboard(board: &Board) -> &BitBoard {
    return board.combined();
}