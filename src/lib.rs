mod lookup;

use crate::lookup::tables::{MASK_RANK, CLEAR_RANK, MASK_FILE, CLEAR_FILE, PIECE, SQUARE};

/*

TODO:

Problem right now -> on capture -> two pieces on the same square !!!

Need a better way to do things differently for the different piece types, 12 if statements are ugly!

* Board representation
* -     Currently -> 12 piece-types x 64 bit mask AKA bitboards + 3 useful bitboards (white-pieces, black-pieces, all-pieces)
* -     Previously: Vector of squares (Empty or -> Vector of pieces)
*
* Game mechanics
* D     Bit Scan (position of a bit)
* -     Square_To_Piece (fn square -> piece on that square)
* -     Attack patterns (Completed apart from handling pins/checks)
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


type BitBoard = u64;



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

fn bit_scan (bit: BitBoard) -> usize {
    let remainder = (bit % 67) as usize;
    MOD67[remainder]
}

fn bit_count (bit: BitBoard) -> usize {
    bit.count_ones() as usize
}




fn print_8_bit (bit: BitBoard) {
    for i in (0..7).rev() {
        if bit & PIECE[i] != 0 {
            print!("0");
        }
        else {
            print!("1");
        }
    }

    print!("\n");
}


/*
Compute King Valid Moves (Incomplete)
Moving king to surrounding spots, masking with own pieces
Using Clear file to avoid clipping edge

Spots:
1 2 3
8 K 4
7 6 5

TODO:
Need check and checkmate validation later
*/
fn compute_king_attacks(king: BitBoard, own_pieces: BitBoard) -> BitBoard {
    let king_clip_h = king & CLEAR_FILE[7];
    let king_clip_a = king & CLEAR_FILE[0];

    let spot_1 = king_clip_h << 7;
    let spot_2 = king << 8;
    let spot_3 = king_clip_h << 9;
    let spot_4 = king_clip_h << 1;

    let spot_5 = king_clip_a >> 7;
    let spot_6 = king >> 8;
    let spot_7 = king_clip_a >> 9;
    let spot_8 = king_clip_a >> 1;

    let king_moves = spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8;

    /* Remove if own pieces block */
    let king_valid = king_moves & !own_pieces;

    /*
    Needs check testing
    */

    king_valid
}

/*
Compute Knights valid moves (Incomplete)
Moving knights to surrounding spots, masking with own pieces
Using Clear file to avoid clipping edge

Spots:
 2 3
1   4
  N 
8   5
 7 6

TODO:
Need check (pin) validation later
*/
fn compute_knight_attacks(knight: BitBoard, own_pieces: BitBoard) -> BitBoard {
    let clip_1 = knight & CLEAR_FILE[0] & CLEAR_FILE[1];
    let clip_2 = knight & CLEAR_FILE[0];
    
    let clip_3 = knight & CLEAR_FILE[7];
    let clip_4 = knight & CLEAR_FILE[7] & CLEAR_FILE[6];

    let clip_5 = knight & CLEAR_FILE[7] & CLEAR_FILE[6];
    let clip_6 = knight & CLEAR_FILE[7];
    let clip_7 = knight & CLEAR_FILE[0];
    let clip_8 = knight & CLEAR_FILE[0] & CLEAR_FILE[1];
    
    
    let spot_1 = clip_1 << 6;
    let spot_2 = clip_2 << 15;
    let spot_3 = clip_3 << 17;
    let spot_4 = clip_4 << 10;

    let spot_5 = clip_5 >> 6;
    let spot_6 = clip_6 >> 15;
    let spot_7 = clip_7 >> 17;
    let spot_8 = clip_8 >> 10;

    let knight_moves = spot_1 | spot_2 | spot_3 | spot_4 | spot_5 | spot_6 | spot_7 | spot_8;

    let knight_valid = knight_moves & !own_pieces;

    knight_valid
}


/*
Compute the attacking squares masked by the opposing pieces
3   4
  P
*/
fn compute_white_pawn_attacks(white_pawn: BitBoard, black_pieces: BitBoard) -> BitBoard {
    // Attack spot 3, unless on file A and only if enemy piece is there
    let spot_3 = ((white_pawn & CLEAR_FILE[0]) << 7) & black_pieces;

    // Attack spot 4, unless on file H and only if enemy piece is there
    let spot_4 = ((white_pawn & CLEAR_FILE[7]) << 9) & black_pieces;

    return spot_3 | spot_4;
}

/*
Compute White Pawn valid moves (Incomplete) - Different for black
Moving pawns to surrounding spots, masking with own pieces
Using Clear file to avoid clipping edge

Spots:
  2
3 1 4
  P

TODO:
Need check (pin) validation later
*/
fn compute_white_pawn_moves(white_pawn: BitBoard, all_pieces: BitBoard, black_pieces: BitBoard) -> BitBoard {
    let spot_1 = (white_pawn << 8) & !all_pieces;
    
    // If pawn can move 1 step into rank 3 and move another step
    let spot_2 = ((spot_1 & MASK_RANK[2]) << 8) & !all_pieces;

    let pawn_attacks = compute_white_pawn_attacks(white_pawn, black_pieces);

    let white_pawn_moves = spot_1 | spot_2 | pawn_attacks;

    white_pawn_moves
}

/*
Compute the attacking squares masked by the opposing pieces
  p
3   4

*/
fn compute_black_pawn_attacks (black_pawn: BitBoard, white_pieces: BitBoard) -> BitBoard {
    // Attack spot 3, unless on file A and only if enemy piece is there
    let spot_3 = ((black_pawn & CLEAR_FILE[0]) >> 9) & white_pieces;

    // Attack spot 4, unless on file H and only if enemy piece is there
    let spot_4 = ((black_pawn & CLEAR_FILE[7]) >> 7) & white_pieces;

    return spot_3 | spot_4;
}


/*
Compute Black Pawn valid moves (Incomplete) - Different for white
Moving pawns to surrounding spots, masking with own pieces
Using Clear file to avoid clipping edge

Spots:
  p
3 1 4
  2

TODO:
Need check (pin) validation later
*/
fn compute_black_pawn_moves(black_pawn: BitBoard, all_pieces: BitBoard, white_pieces: BitBoard) -> BitBoard {
    let spot_1 = (black_pawn >> 8) & !all_pieces;
    
    // If pawn can move 1 step into rank 3 and move another step
    let spot_2 = ((spot_1 & MASK_RANK[5]) >> 8) & !all_pieces;

    let pawn_attacks = compute_black_pawn_attacks(black_pawn, white_pieces);

    let black_pawn_moves = spot_1 | spot_2 | pawn_attacks;

    black_pawn_moves
}

/*
Compute the targets the bishop could possibly have, (targets could be of own color)

*/
fn compute_bishop_attacks(bishop: BitBoard, all_pieces: BitBoard, enemy_pieces: BitBoard) -> BitBoard {
    let mut attacks: BitBoard = 0;

    let square = bit_scan(bishop);

    // initialize target rank and files
    let tr = square / 8;
    let tf = square % 8;

    // Up and right
    for (r, f) in ((tr+1)..8).zip((tf+1)..8) {
        let b = (1 as BitBoard) << (r * 8 + f);
        // Detect if piece is in the path of bishop
        if all_pieces & b == b { 
            if enemy_pieces & b == b { attacks |= b; }
            break;
        }
        attacks |= b;
    }

    // Up and left
    for (r, f) in ((tr+1)..8).zip((0..tf).rev()) {
        let b = (1 as BitBoard) << (r * 8 + f);
        // Detect if piece is in the path of bishop
        if all_pieces & b == b { 
            if enemy_pieces & b == b { attacks |= b; }
            break;
        }
        attacks |= b;
    }

    // Down and left
    for (r, f) in ((0..tr).rev()).zip((0..tf).rev()) {
        // Detect if piece is in the path of bishop
        let b = (1 as BitBoard) << (r * 8 + f);
        if all_pieces & b == b { 
            if enemy_pieces & b == b { attacks |= b; }
            break;
        }
        attacks |= b;
    }

    // Down and right
    for (r, f) in ((0..tr).rev()).zip((tf+1)..8) {
        // Detect if piece is in the path of bishop
        let b = (1 as BitBoard) << (r * 8 + f);
        if all_pieces & b == b { 
            if enemy_pieces & b == b { attacks |= b; }
            break;
        }
        attacks |= b;
    }

    attacks
}

/*
Compute the targets a rook could attack, (could be target of own color)

*/
fn compute_rook_attacks(rook: BitBoard, all_pieces: BitBoard, enemy_pieces: BitBoard) -> BitBoard {
    let mut attacks: BitBoard = 0;
    let square = bit_scan(rook);
    let tr = square / 8;
    let tf = square % 8;
    
    for r in (tr+1)..8 {
        let b = (1 as BitBoard) << (r * 8 + tf);
        // Detect if piece is in the path of rook
        if all_pieces & b == b { 
            if enemy_pieces & b == b { attacks |= b; }
            break;
        }
        attacks |= b;
    }
    for r in (0..tr).rev() {
        let b = (1 as BitBoard) << (r * 8 + tf);
        // Detect if piece is in the path of rook
        if all_pieces & b == b { 
            if enemy_pieces & b == b { attacks |= b; }
            break;
        }
        attacks |= b;
    }
    for f in (tf+1)..8 {
        let b = (1 as BitBoard) << (tr * 8 + f);
        // Detect if piece is in the path of rook
        if all_pieces & b == b { 
            if enemy_pieces & b == b { attacks |= b; }
            break;
        }
        attacks |= b;
    }
    for f in (0..tf).rev() {
        let b = (1 as BitBoard) << (tr * 8 + f);
        // Detect if piece is in the path of rook
        if all_pieces & b == b { 
            if enemy_pieces & b == b { attacks |= b; }
            break;
        }
        attacks |= b;
    }
    
    attacks
}


// Prints the board
fn print_board (board: BitBoard) {
    print!("\n");
    for y in (0..8).rev() {
        for x in 0..8 {
            let bitval: BitBoard = (1 as BitBoard) << (y*8 + x);
            if board & bitval == bitval {
                print!("1 ");
            }
            else {
                print!("0 ");
            }
        }
        print!("\n");
    }
    print!("---------------\n");
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceType {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
    Other
}

fn is_white(piece: PieceType) -> bool {
    return match piece {
        PieceType::WhitePawn => true,
        PieceType::WhiteKnight => true,
        PieceType::WhiteBishop => true,
        PieceType::WhiteRook => true,
        PieceType::WhiteQueen => true,
        PieceType::WhiteKing => true,
        _ => false
    }
}

pub struct ChessBoard {
    /* All White Pieces */
    white_pawns: BitBoard,
    white_knights: BitBoard,
    white_bishops: BitBoard,
    white_rooks: BitBoard,
    white_queens: BitBoard,
    white_kings: BitBoard,
    
    /* All Black Pieces */
    black_pawns: BitBoard,
    black_knights: BitBoard,
    black_bishops: BitBoard,
    black_rooks: BitBoard,
    black_queens: BitBoard,
    black_kings: BitBoard,

    /* Derived Positions */
    white_pieces: BitBoard,
    black_pieces: BitBoard,
    all_pieces: BitBoard,

}

impl Default for ChessBoard {
    fn default() -> ChessBoard {
        ChessBoard {
            /* All White Pieces */
            white_pawns: 0,
            white_knights: 0,
            white_bishops: 0,
            white_rooks: 0,
            white_queens: 0,
            white_kings: 0,
            
            /* All Black Pieces */
            black_pawns: 0,
            black_knights: 0,
            black_bishops: 0,
            black_rooks: 0,
            black_queens: 0,
            black_kings: 0,
        
            /* Derived Positions */
            white_pieces: 0,
            black_pieces: 0,
            all_pieces: 0,
        }
    }
}

impl ChessBoard {
    /* Reset entire board to empty */ 
    fn clear (&mut self) {
        /* All White Pieces */
        self.white_pawns = 0;
        self.white_knights = 0;
        self.white_bishops = 0;
        self.white_rooks = 0;
        self.white_queens = 0;
        self.white_kings = 0;
        
        /* All Black Pieces */
        self.black_pawns = 0;
        self.black_knights = 0;
        self.black_bishops = 0;
        self.black_rooks = 0;
        self.black_queens = 0;
        self.black_kings = 0;
    
        /* Derived Positions */
        self.white_pieces = 0;
        self.black_pieces = 0;
        self.all_pieces = 0;
        
    }


    /*
    Combine all attack patterns into one attack function, that returns a bitboard of every square currently attacked by one side.
    Note - pieces may be pinned yet can still attack a square - ex. Bishop could be pinned down on A2, By a rook on A3 when king is on A1 -> Still attacks B3,C4... and could check the other king
    */
    fn compute_white_attacks (&self, black_pieces_option: Option<BitBoard>, white_pieces_option: Option<BitBoard>) -> BitBoard {
        let black_pieces;
        let white_pieces;
        match black_pieces_option {
            Some(bb) => black_pieces = bb,
            None => black_pieces = self.black_pieces
        }
        match white_pieces_option {
            Some(bb) => white_pieces = bb,
            None => white_pieces = self.white_pieces
        }
        let all_pieces = black_pieces | white_pieces;
        
        let mut attacks: BitBoard = 0;
        attacks |= compute_white_pawn_attacks(self.white_pawns, black_pieces);
        attacks |= compute_knight_attacks(self.white_knights, self.white_pieces);
        attacks |= compute_king_attacks(self.white_kings, white_pieces);

        for i in 0..63 {
            if self.white_bishops & PIECE[i] != 0{
                attacks |= compute_bishop_attacks(PIECE[i], white_pieces, black_pieces);
            }
            if self.white_rooks & PIECE[i] != 0 {
                attacks |= compute_rook_attacks(PIECE[i], white_pieces, black_pieces);
            }
            if self.white_queens & PIECE[i] != 0{
                attacks |= compute_bishop_attacks(PIECE[i], white_pieces, black_pieces);
                attacks |= compute_rook_attacks(PIECE[i], white_pieces, black_pieces);
            }
        }

        attacks
    }

    /*
    Combine all attack patterns into one attack function, that returns a bitboard of every square currently attacked by one side.
    Note - pieces may be pinned yet can still attack a square - ex. Bishop could be pinned down on A2, By a rook on A3 when king is on A1 -> Still attacks B3,C4... and could check the other king
    */
    fn compute_black_attacks (&self, black_pieces_option: Option<BitBoard>, white_pieces_option: Option<BitBoard>) -> BitBoard {
        let black_pieces;
        let white_pieces;
        match black_pieces_option {
            Some(bb) => black_pieces = bb,
            None => black_pieces = self.black_pieces
        }
        match white_pieces_option {
            Some(bb) => white_pieces = bb,
            None => white_pieces = self.white_pieces
        }
        let all_pieces = black_pieces | white_pieces;

        let mut attacks: BitBoard = 0;
        attacks |= compute_black_pawn_attacks(self.black_pawns, white_pieces);
        attacks |= compute_knight_attacks(self.black_knights, black_pieces);
        attacks |= compute_king_attacks(self.black_kings, black_pieces);

        for i in 0..63 {
            if self.black_bishops & PIECE[i] != 0{
                attacks |= compute_bishop_attacks(PIECE[i], all_pieces, white_pieces);
            }
            if self.black_rooks & PIECE[i] != 0 {
                attacks |= compute_rook_attacks(PIECE[i], all_pieces, white_pieces);
            }
            if self.black_queens & PIECE[i] != 0{
                attacks |= compute_bishop_attacks(PIECE[i], all_pieces, white_pieces);
                attacks |= compute_rook_attacks(PIECE[i], all_pieces, white_pieces);
            }
        }

        attacks
    }

    /*
    Function test whether white in check
    */
    fn white_in_check (&self) -> bool {
        let black_attacks: BitBoard = self.compute_black_attacks(None, None);

        PIECE[bit_scan(self.white_kings)] & black_attacks != 0
    }
    /*
    Function test whether white in check
    */
    fn white_in_check_mut (&self, black_attacks: BitBoard) -> bool {
        PIECE[bit_scan(self.white_kings)] & black_attacks != 0
    }
    /*
    Function test whether white in check
    */
    fn white_in_check_king (&self, black_attacks: BitBoard, white_kings: BitBoard) -> bool {
        PIECE[bit_scan(white_kings)] & black_attacks != 0
    }

    /*
    Function test whether black in check
    */
    fn black_in_check (&self) -> bool {
        let white_attacks: BitBoard = self.compute_white_attacks(None, None);

        PIECE[bit_scan(self.black_kings)] & white_attacks != 0
    }
    
    /*
    Function test whether black in check
    */
    fn black_in_check_mut (&self, white_attacks: BitBoard) -> bool {
        PIECE[bit_scan(self.black_kings)] & white_attacks != 0
    }
    /*
    Function test whether black in check
    */
    fn black_in_check_king (&self, white_attacks: BitBoard, black_kings: BitBoard) -> bool {
        PIECE[bit_scan(black_kings)] & white_attacks != 0
    }


    /*
    Get BitBoard of possible moves a piece
    */
    fn get_moves (&self, square: BitBoard) -> BitBoard {
        let mut moves = 0;
        let piece_type = self.piece_at(square);

        // NEEDS CHECK SAFEGUARD FROM CHECK IMPLEMENTATION (pinned pieces)

        match piece_type {
            PieceType::WhiteKing =>     moves |= compute_king_attacks(square, self.white_pieces),
            PieceType::WhiteQueen =>    moves |= compute_rook_attacks(square, self.all_pieces, self.black_pieces)
                                            | compute_bishop_attacks(square, self.all_pieces, self.black_pieces),
            PieceType::WhiteRook =>     moves |= compute_rook_attacks(square, self.all_pieces, self.black_pieces),
            PieceType::WhiteBishop =>   moves |= compute_bishop_attacks(square, self.all_pieces, self.black_pieces),
            PieceType::WhiteKnight =>   moves |= compute_knight_attacks(square, self.white_pieces),
            PieceType::WhitePawn =>     moves |= compute_white_pawn_moves(square, self.all_pieces, self.black_pieces),
            PieceType::BlackKing =>     moves |= compute_king_attacks(square, self.black_pieces),
            PieceType::BlackQueen =>    moves |= compute_rook_attacks(square, self.all_pieces, self.white_pieces)
                                            | compute_bishop_attacks(square, self.all_pieces, self.white_pieces),
            PieceType::BlackRook =>     moves |= compute_rook_attacks(square, self.all_pieces, self.white_pieces),
            PieceType::BlackBishop =>   moves |= compute_bishop_attacks(square, self.all_pieces, self.white_pieces),
            PieceType::BlackKnight =>   moves |= compute_knight_attacks(square, self.black_pieces),
            PieceType::BlackPawn =>     moves |= compute_black_pawn_moves(square, self.all_pieces, self.white_pieces),
            _ => moves = 0
        }

        let is_white: bool = is_white(piece_type);

        if piece_type != PieceType::WhiteKing && piece_type != PieceType::BlackKing {
            for i in 0..63 {
                if moves & PIECE[i] != 0 {
                    if is_white {
                        if self.white_in_check_mut(self.compute_black_attacks(Some(self.black_pieces), Some(self.white_pieces & !square | PIECE[i]))) {
                            moves &= !PIECE[i];
                        }
                    } else {
                        if self.black_in_check_mut(self.compute_white_attacks(Some(self.black_pieces & !square | PIECE[i]), Some(self.white_pieces))) {
                            moves &= !PIECE[i];
                        }
                    }
                }
            }
        }
        else {
            for i in 0..63 {
                if moves & PIECE[i] != 0 {
                    if is_white {
                        if self.white_in_check_king(self.compute_black_attacks(Some(self.black_pieces), Some(self.white_pieces & !square | PIECE[i])), PIECE[i]) {
                            moves &= !PIECE[i];
                        }
                    } else {
                        if self.black_in_check_king(self.compute_white_attacks(Some(self.black_pieces & !square | PIECE[i]), Some(self.white_pieces)), PIECE[i]) {
                            moves &= !PIECE[i];
                        }
                    }
                }
            }
        }

        
        
        moves
    }

    fn piece_at (&self, square: BitBoard) -> PieceType {
        if self.white_pawns & square != 0 { return PieceType::WhitePawn; }
        if self.black_pawns & square != 0 { return PieceType::BlackPawn; }
        if self.white_knights & square != 0 { return PieceType::WhiteKnight; }
        if self.black_knights & square != 0 { return PieceType::BlackKnight; }
        if self.white_bishops & square != 0 { return PieceType::WhiteBishop; }
        if self.black_bishops & square != 0 { return PieceType::BlackBishop; }
        if self.white_rooks & square != 0 { return PieceType::WhiteRook; }
        if self.black_rooks & square != 0 { return PieceType::BlackRook; }
        if self.white_queens & square != 0 { return PieceType::WhiteQueen; }
        if self.black_queens & square != 0 { return PieceType::BlackQueen; }
        if self.white_kings & square != 0 { return PieceType::WhiteKing; }
        if self.black_kings & square != 0 { return PieceType::BlackKing; }

        PieceType::Other
    }

    fn move_piece (&mut self, from: usize, to: usize) -> bool {
        // if piece doesn't exist
        if self.all_pieces & PIECE[from] == 0 { return false; }

        let mut moves = self.get_moves(PIECE[from]);

        // break if piece cant move to desired position
        if moves == 0 { return false; }
        if moves & PIECE[to] == 0 { return false; }

        let piece_type: PieceType = self.piece_at(PIECE[from]);
        
        match piece_type {
            PieceType::WhiteKing => { self.white_kings &= !PIECE[from]; self.white_kings |= PIECE[to]; },
            PieceType::WhiteQueen => { self.white_queens &= !PIECE[from]; self.white_queens |= PIECE[to]; },
            PieceType::WhiteRook => { self.white_rooks &= !PIECE[from]; self.white_rooks |= PIECE[to]; },
            PieceType::WhiteBishop => { self.white_bishops &= !PIECE[from]; self.white_bishops |= PIECE[to]; },
            PieceType::WhiteKnight => { self.white_knights &= !PIECE[from]; self.white_knights |= PIECE[to]; },
            PieceType::WhitePawn => { self.white_pawns &= !PIECE[from]; self.white_pawns |= PIECE[to]; },
            PieceType::BlackKing => { self.black_kings &= !PIECE[from]; self.black_kings |= PIECE[to]; },
            PieceType::BlackQueen => { self.black_queens &= !PIECE[from]; self.black_queens |= PIECE[to]; },
            PieceType::BlackRook => { self.black_rooks &= !PIECE[from]; self.black_rooks |= PIECE[to]; },
            PieceType::BlackBishop => { self.black_bishops &= !PIECE[from]; self.black_bishops |= PIECE[to]; },
            PieceType::BlackKnight => { self.black_knights &= !PIECE[from]; self.black_knights |= PIECE[to]; },
            PieceType::BlackPawn => { self.black_pawns &= !PIECE[from]; self.black_pawns |= PIECE[to]; },
            _ => panic!("No Piece Type")
        }

        self.update_board();

        true
    }
    
    
    fn load (&mut self, fen: String) {
        // Clear the entire board
        self.clear();

        // Split FEN into different parts
        let fen_vec: Vec<&str> = fen.split(" ").collect::<Vec<&str>>();

        // Split FEN-position into a vec from bottom to top
        let mut fen_rows: Vec<String> = fen_vec[0].split("/").map(|x| x.to_string()).collect();
        fen_rows.reverse();


        // Iterate through the FEN position, keeping track of position
        let mut y: usize = 0;
        for row in fen_rows.iter() {
            let mut x: usize = 0;
            let row_char: Vec<char> = row.chars().collect();

            for s in row_char.iter() {
                let pos = y*8+x;
                
                /*
                TODO!!
                Implement safe-guard system for bad FEN strings, (check chars and such)
                 */ 

                match s {   
                    /* Add Black Piece from FEN */
                    'p' => self.black_pawns |= PIECE[pos],
                    'n' => self.black_knights |= PIECE[pos],
                    'b' => self.black_bishops |= PIECE[pos],
                    'r' => self.black_rooks |= PIECE[pos],
                    'q' => self.black_queens |= PIECE[pos],
                    'k' => self.black_kings |= PIECE[pos],
                    /* Add Whtie Piece from FEN */
                    'P' => self.white_pawns |= PIECE[pos],
                    'N' => self.white_knights |= PIECE[pos],
                    'B' => self.white_bishops |= PIECE[pos],
                    'R' => self.white_rooks |= PIECE[pos],
                    'Q' => self.white_queens |= PIECE[pos],
                    'K' => self.white_kings |= PIECE[pos],
                    /* Read amount of empty space from FEN */
                    _ => x += (*s as usize) - ('0') as usize - 1
                }
                x += 1;
            }
            y += 1;
        }

        // Update the derived boards
        self.update_board();
    }

    // Updates the derived boards
    fn update_board (&mut self) {
        self.white_pieces = self.white_pawns | self.white_knights | self.white_bishops | self.white_rooks | self.white_queens | self.white_kings;
        self.black_pieces = self.black_pawns | self.black_knights | self.black_bishops | self.black_rooks | self.black_queens | self.black_kings;
        self.all_pieces = self.white_pieces | self.black_pieces;
    }


    pub fn reset (&mut self) {
        self.load("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
    }

    fn print_board(&self, b: BitBoard){
        print!("\n");
        for y in (0..8).rev() {
            for x in 0..8 {
                let bitval: BitBoard = (1 as BitBoard) << (y*8 + x);
                if self.all_pieces & bitval != 0 {
                    if self.black_pawns & bitval != 0 { print!("p "); }
                    if self.black_knights & bitval != 0 { print!("n "); }
                    if self.black_bishops & bitval != 0 { print!("b "); }
                    if self.black_rooks & bitval != 0 { print!("r "); }
                    if self.black_queens & bitval != 0 { print!("q "); }
                    if self.black_kings & bitval != 0 { print!("k "); }

                    if self.white_pawns & bitval != 0 { print!("P "); }
                    if self.white_knights & bitval != 0 { print!("N "); }
                    if self.white_bishops & bitval != 0 { print!("B "); }
                    if self.white_rooks & bitval != 0 { print!("R "); }
                    if self.white_queens & bitval != 0 { print!("Q "); }
                    if self.white_kings & bitval != 0 { print!("K "); }
                }
                else {
                    if b & bitval != 0 { print!("1 ");  }
                    else { print!(". "); }
                }
            }
            print!("\n");
        }
        print!("---------------\n");
    }
}

pub fn game () -> ChessBoard {
    let mut chessboard = ChessBoard { ..Default::default() };
    chessboard.reset();

    chessboard
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let mut chess: ChessBoard = game(); 
        println!("Game Loaded!");

        chess.print_board(0);

        chess.move_piece(SQUARE::E2 as usize,SQUARE::E4 as usize);
        chess.print_board(0);

        chess.move_piece(SQUARE::E7 as usize,SQUARE::E5 as usize);
        chess.print_board(0);

        chess.move_piece(SQUARE::F1 as usize,SQUARE::C4 as usize);
        chess.print_board(0);
        
        chess.move_piece(SQUARE::A7 as usize,SQUARE::A6 as usize);
        chess.print_board(0);

        chess.move_piece(SQUARE::C4 as usize,SQUARE::F7 as usize);
        chess.print_board(0);
        
        assert_eq!(bit_count(chess.all_pieces),32);
    }

    #[test]
    fn test_possible_moves() {
        // POS where white is in check
        let mut chess: ChessBoard = game(); 
        chess.load("2k5/8/4q3/8/6b1/1n6/1PPP4/3KR3".to_string());

        //chess.print_board(chess.compute_black_attacks(None, None));

        //println!("White in check = {}", chess.white_in_check());
        //println!("Black in check = {}", chess.black_in_check());
        /*
        for i in 0..63 {
            if chess.white_pieces & PIECE[i] != 0 {
                let moves = chess.get_moves(PIECE[i]);
                if moves != 0 {
                    println!("{} {}", i / 8, i % 8);
                    chess.print_board(moves);
                } 
            }
        }*/

        //chess.print_board(0);
    }
}
