use std::io::{Read, Write, stdout, stdin, BufRead};
use std::str::FromStr;
use chess::{Board, BoardStatus};
use crate::search;
use crate::board;
use std::process;



pub fn start() {
    let stdin = stdin();
    let mut stdout = stdout();
    let mut fen = String::new();

    for line in stdin.lock().lines() {
        match line {
            Ok(v) => {
                let value = v.as_str();

                match (value) {
                    "uci" => {
                        // No action
                        // Respond with engine details and uciok
                        stdout.write_all(b"id name rusty");
                        stdout.write_all(b"id author kai-pereira");
                        stdout.write_all(b"uciok");
                    },
                    "isready" => {
                        // Action: Start pondering
                        // Respond readyok
                        stdout.write_all(b"readyok");
                    },
                    _ => {
                        // Dynamic commands
                        if value.contains("position") {
                            // Save the board fen
                            fen = value.replace("position", "").to_string(); 
                        }

                        if value.contains("go") {
                            // Setup the board using fen and return best move 
                            let board = Board::from_str(&fen.trim());

                            match (board) {
                                Ok(v) => {
                                    if let Some(best_move) = search::get_move(&v, 4, true) {                                    
                                        let written_string = format!("bestmove {}", best_move);   
                                        stdout.write_all(written_string.to_string().as_bytes());
                                    } else {
                                        process::exit(0x0100);
                                    }
                                },
                                Err(e) => {
                                    println!("ERROR: {e}");
                                }
                            }
                        }
                    }
                }

                // Push all stdout to terminal
                stdout.flush();
            },
            Err(e) => {
                println!("ERR: {e}");
            }
        }
    }
}