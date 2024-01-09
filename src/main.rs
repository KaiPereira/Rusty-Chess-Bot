fn main() {
    parse_fen()
}


pub fn parse_fen() {
    let fen: &str = "rnbqkbnr/ppp1pppp/8/3p4/2PP4/8/PP2PPPP/RNBQKBNR b KQkq c3 0 2";
    let fen_values: Vec<_> = fen.split_whitespace().collect();
    let board_fen: &str = fen_values[0];
    
    // Our board
    let mut board: Vec<char> = Vec::new();

    for section in board_fen.split("/").collect::<Vec<&str>>() {

        for char in section.chars() {    
            if char.is_numeric() {
                let amount_of_zeros = char.to_digit(10).unwrap();

                // Push that many zeros to an array or push the letter
                for _ in 0..amount_of_zeros {
                    board.push('0');
                }

            } else {
                board.push(char);
            }
        }
    }

    println!("{:?}", board)
}