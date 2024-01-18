use chess::{Board, BitBoard, Square, Piece, Color};
use crate::pieces;

pub fn create_board() -> Board {
    let board: Board = Board::default();

    return board
}


pub fn get_bitboard(board: &Board) -> &BitBoard {
    return board.combined();
}


pub fn display(board: &Board) {
    // CRAP CODE GO SCRATATATATTATATATATTATATA
    // Sooooo haha future kai, i'm so sorry for my programming sin :(
    // Let's be honest here though, I was having a tough day and decided to release it on my code, very therapeutic


    // All black pieces
    let black_pawn: String = pieces::get_piece_bitboard_string(Piece::Pawn, board, Color::Black);
    let black_knight: String = pieces::get_piece_bitboard_string(Piece::Knight, board, Color::Black);
    let black_bishop: String = pieces::get_piece_bitboard_string(Piece::Bishop, board, Color::Black);
    let black_rook: String = pieces::get_piece_bitboard_string(Piece::Rook, board, Color::Black);
    let black_queen: String = pieces::get_piece_bitboard_string(Piece::Queen, board, Color::Black);
    let black_king: String = pieces::get_piece_bitboard_string(Piece::King, board, Color::Black);

    // All white pieces
    let white_pawn: String = pieces::get_piece_bitboard_string(Piece::Pawn, board, Color::White);
    let white_knight: String = pieces::get_piece_bitboard_string(Piece::Knight, board, Color::White);
    let white_bishop: String = pieces::get_piece_bitboard_string(Piece::Bishop, board, Color::White);
    let white_rook: String = pieces::get_piece_bitboard_string(Piece::Rook, board, Color::White);
    let white_queen: String = pieces::get_piece_bitboard_string(Piece::Queen, board, Color::White);
    let white_king: String = pieces::get_piece_bitboard_string(Piece::King, board, Color::White);

    
    let mut display_board = [
        '.','.','.','.','.','.','.','.',
        '.','.','.','.','.','.','.','.',
        '.','.','.','.','.','.','.','.',
        '.','.','.','.','.','.','.','.',
        '.','.','.','.','.','.','.','.',
        '.','.','.','.','.','.','.','.',
        '.','.','.','.','.','.','.','.',
        '.','.','.','.','.','.','.','.'
    ];

    for (index, square) in black_pawn.chars().enumerate() {
        if square != '.' {
            display_board[index] = square
        }
    }
    for (index, square) in black_knight.chars().enumerate() {
        if square != '.' {
            display_board[index] = square
        }
    }
    for (index, square) in black_bishop.chars().enumerate() {
        if square != '.' {
            display_board[index] = square
        }
    }
    for (index, square) in black_rook.chars().enumerate() {
        if square != '.' {
            display_board[index] = square
        }
    }
    for (index, square) in black_queen.chars().enumerate() {
        if square != '.' {
            display_board[index] = square
        }
    }
    for (index, square) in black_king.chars().enumerate() {
        if square != '.' {
            display_board[index] = square
        }
    }


    for (index, square) in white_pawn.chars().enumerate() {
        if square != '.' {
            display_board[index] = square
        }
    }
    for (index, square) in white_knight.chars().enumerate() {
        if square != '.' {
            display_board[index] = square
        }
    }
    for (index, square) in white_bishop.chars().enumerate() {
        if square != '.' {
            display_board[index] = square
        }
    }
    for (index, square) in white_rook.chars().enumerate() {
        if square != '.' {
            display_board[index] = square
        }
    }
    for (index, square) in white_queen.chars().enumerate() {
        if square != '.' {
            display_board[index] = square
        }
    }
    for (index, square) in white_king.chars().enumerate() {
        if square != '.' {
            display_board[index] = square
        }
    }


    // Convert the array to a string
    let display_board_string: String = display_board.iter().collect();

    let formatted_display: String = display_board_string.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i > 0 && i % 8 == 0 {
                Some('\n')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect();

    let formatted_reversed_display: String = formatted_display.chars().rev().collect::<String>();

    println!("\n\n BLACK \n");
    println!("{}", formatted_reversed_display);
    println!("\n White \n\n");
}
