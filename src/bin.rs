use davbjor_chess::{ChessBoard, PieceType, GameResult};
use std::io;


/*
Converts chess notation to a square on the board, on bad input it returns square 64 (non-existant),
*/
pub fn string_to_square(s: String) -> usize {
    let x = s.to_uppercase();
    return match x.as_str() {
        "A1"  => 0,
        "B1"  => 1,
        "C1"  => 2,
        "D1"  => 3,
        "E1"  => 4,
        "F1"  => 5,
        "G1"  => 6,
        "H1"  => 7,
        "A2"  => 8,
        "B2"  => 9,
        "C2"  => 10,
        "D2"  => 11,
        "E2"  => 12,
        "F2"  => 13,
        "G2"  => 14,
        "H2"  => 15,
        "A3"  => 16,
        "B3"  => 17,
        "C3"  => 18,
        "D3"  => 19,
        "E3"  => 20,
        "F3"  => 21,
        "G3"  => 22,
        "H3"  => 23,
        "A4"  => 24,
        "B4"  => 25,
        "C4"  => 26,
        "D4"  => 27,
        "E4"  => 28,
        "F4"  => 29,
        "G4"  => 30,
        "H4"  => 31,
        "A5"  => 32,
        "B5"  => 33,
        "C5"  => 34,
        "D5"  => 35,
        "E5"  => 36,
        "F5"  => 37,
        "G5"  => 38,
        "H5"  => 39,
        "A6"  => 40,
        "B6"  => 41,
        "C6"  => 42,
        "D6"  => 43,
        "E6"  => 44,
        "F6"  => 45,
        "G6"  => 46,
        "H6"  => 47,
        "A7"  => 48,
        "B7"  => 49,
        "C7"  => 50,
        "D7"  => 51,
        "E7"  => 52,
        "F7"  => 53,
        "G7"  => 54,
        "H7"  => 55,
        "A8"  => 56,
        "B8"  => 57,
        "C8"  => 58,
        "D8"  => 59,
        "E8"  => 60,
        "F8"  => 61,
        "G8"  => 62,
        "H8"  => 63,
        _ => 64
    }
}
    

fn main() {
    let mut chess = ChessBoard::new(); 
    //chess.load("k5rr/8/8/8/8/8/7p/7K w ---- - 0 1".to_string());
    
    loop {
        //Check the state of the game
        match chess.game_result {
            GameResult::White => { println!("White has won!"); break; },
            GameResult::Black => { println!("Black has won!"); break; },
            GameResult::Draw => { println!("Game is a draw!"); break; },
            _ => ()
        } 

        for i in (0..8).rev() {
            let mut s = "".to_string();
            print!("{}    ", i+1);
            for j in 0..8 {
                match chess.board[i*8+j] {
                    PieceType::WhitePawn => s.push_str("P "),
                    PieceType::WhiteKnight => s.push_str("N "),
                    PieceType::WhiteBishop => s.push_str("B "),
                    PieceType::WhiteRook => s.push_str("R "),
                    PieceType::WhiteQueen => s.push_str("Q "),
                    PieceType::WhiteKing => s.push_str("K "),
                    PieceType::BlackPawn => s.push_str("P "),
                    PieceType::BlackKnight => s.push_str("N "),
                    PieceType::BlackBishop => s.push_str("B "),
                    PieceType::BlackRook => s.push_str("R "),
                    PieceType::BlackQueen => s.push_str("Q "),
                    PieceType::BlackKing => s.push_str("K "),
                    PieceType::Empty => s.push_str(". "),
                    _ => (),
                }
            }
            println!("{s}");
        }
        println!("\n     A B C D E F G H");

        // Read input n
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("failed to readline");

        //let line = input.next().unwrap().unwrap();
        let vec: Vec<&str> = line.split(" ").collect();

        if vec.len() >= 2 {
            let w1 = vec[0];
            let w2 = vec[1];
            let sq1 = string_to_square(w1.trim().to_string());
            let sq2 = string_to_square(w2.trim().to_string());
            
            if sq1 == 64 || sq2 == 64 { break; }
            match chess.move_piece(sq1, sq2) {
                Ok(true) => {
                    ()
                },
                Ok(false) => (),
                Err(s) => println!("Error: {s}")
            }
        }
        else if vec.len() == 1 {
            let w1 = vec[0];
            let sq1 = string_to_square(w1.trim().to_string());

            if sq1 == 64 { break; }
            let moves = chess.get_moves_list(sq1);
            for m in moves {
                println!("{m}");
            }
        }

    }
}