/*


TODO:
* Board representation
* -     Currently: Vector of squares (Empty or -> Vector of pieces)
* -     Future/Possibly -> 12 piece-types x 64 bit mask AKA bitboards
*
* Game mechanics
* D     Bit Scan (position of a bit)
* -     Square_To_Piece (fn square -> piece on that square)
* -     Attack patterns
* -         Pawn
* -         Knight
* -         Bishop
* -         Rook
* -         Queen
* -         King
* -     Is player in check
* -     Single piece possible moves (is in check-handling aswell)
* -     All pieces possible moves (Probably good for checkmate handling)
* -     Move piece (if possible)
* -     Is position checkmate
* -     Promoting (To N,B,R,Q)
*
* FEN data
* D     Import FEN
* -     Export FEN
* -     Player turn,
* -     En passant target square (If a pawn moves 2 places -> store the square behind it)
* -     Castling rights (king-side, queen-side, black, white)
* -     Halfmove count
* 
* Special positions/rules
* -     Moves since last pawn move or capture (for 50 move rule)
* -     Store old positions (for 3-fold repetition)
* -         Data-Format (perhaps - 12(piece-types) * 64bit mask AKA bitboards)
* -         Hashing function to compare positions
*
* Unit Testing
* -     Importing series of FEN-positions of a game
* -     


*/


/*

BitScan by modulo from:
https://www.chessprogramming.org/BitScan

*/
static MOD67: [usize; 67] = [
64, 0, 1, 39, 2, 15, 40, 23,
3, 12, 16, 59, 41, 19, 24, 54,
4, 64, 13, 10, 17, 62, 60, 28,
42, 30, 20, 51, 25, 44, 55, 47,
5, 32, 64, 38, 14, 22, 11, 58,
18, 53, 63,  9, 61, 27, 29, 50,
43, 46, 31, 37, 21, 57, 52, 8,
26, 49, 45, 36, 56, 7, 48, 35,
6, 34, 33 
];

pub fn bit_scan (bit: u64) -> usize {
    let remainder = (bit % 67) as usize;
    MOD67[remainder]
}


#[derive(Debug, PartialEq)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

// takes a lowercase
fn from_char(c: char) -> PieceType {
    let p: PieceType;
    match c {
        'p' | 'P' => p = PieceType::Pawn,
        'n' | 'N' => p = PieceType::Knight,
        'b' | 'B' => p = PieceType::Bishop,
        'r' | 'R' => p = PieceType::Rook,
        'q' | 'Q' => p = PieceType::Queen,
        'k' | 'K' => p = PieceType::King,
        _ => panic!("Type not found"),
    };

    p
}

#[derive(Debug, PartialEq)]
enum Color {
    White,
    Black
}

#[derive(Debug, PartialEq)]
struct Piece {
    pos: u64,
    color: Color,
    piece_type: PieceType
}

impl Piece {
    fn make_str(&self) -> String {
        let mut res = match self.piece_type {
            PieceType::Pawn => "p",
            PieceType::Knight => "n",
            PieceType::Bishop => "b",
            PieceType::Rook => "r",
            PieceType::Queen => "q",
            PieceType::King => "k"
        }.to_string();
        
        if self.color == Color::White {
            res = res.to_uppercase();
        }

        res
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Square {
    Empty,
    Occupied(usize)
}

/*

Game struct

*/
#[derive(Debug, PartialEq)]
pub struct Game {
    pieces: Vec<Piece>,
    squares: Vec<Square>
}

impl Game {
    fn push_piece_and_square(&mut self, pos: usize, color: Color, piece_type: PieceType, index: &mut usize){
        self.pieces.push(Piece {
            pos: (1 as u64) << pos,
            color: color,
            piece_type: piece_type
        });
        self.squares.push(Square::Occupied(*index));
        *index += 1;
    }

    fn push_empty_square(&mut self){
        self.squares.push(Square::Empty);
    }

    pub fn export_fen(&mut self){

    }

    pub fn load_fen (&mut self, fen: String) {
        self.pieces.clear();
        self.squares.clear();

        let vfen: Vec<&str> = fen.split(" ").collect();
        let vcodes: Vec<u8> = Vec::from(vfen[0]);

        let mut i: usize = 0;
        let mut piece_i: usize = 0;

        for s in vcodes {
            // Backslash
            if s == 47 {
                continue;
            }

            // Lowercase letters
            if s >= 97 && s <= 122 {
                self.push_piece_and_square(i, Color::Black, from_char(s as char), &mut piece_i);
            }

            // Uppercase letters
            if s >= 65 && s <= 90 {
                self.push_piece_and_square(i, Color::White, from_char(s as char), &mut piece_i);
            }

            // Numbers
            if s >= 48 && s <= 57 {
                // Convert char digit to integer
                let num: i32 = s as i32 - ('0') as i32;
                for _ in 0..num {
                    self.push_empty_square();
                }
                i += num as usize - 1;
            }
            
            i += 1;
        }
    }

    fn initialize (&mut self) {
        //
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();

        self.load_fen(fen);
    }

    fn print_board (&self) {
        //let mut temp = "".to_owned();

        for i in 0..8 {
            print!("{}  ", (8-i).to_string());

            for j in 0..8 {
                let x = i * 8 + j;
                
                let c = match self.squares[x] {
                    Square::Empty => ".",
                    Square::Occupied(idx) => &self.pieces[idx].make_str(),
                };
                
                print!("{} ",c);
            }

            print!("\n");
        }
        println!("\n  a b c d e f g h");
        println!("--------------------\n");
    }
}



pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn new_game() -> Game {
    let mut game = Game {
        pieces: vec![],
        squares: vec![]
    };
    game.initialize();

    game.print_board();

    game
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);

        let mut game: Game = new_game();
        
        game.load_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string());
        game.print_board();

        for i in 0..game.pieces.len() {
            println!("{} {} {}", game.pieces[i].make_str(), game.pieces[i].pos, bit_scan(game.pieces[i].pos));
        }

        println!("First test!");

        assert_eq!(result, 4);
    }
}
