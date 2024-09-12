pub mod patterns {


type BitBoard = u64;
use crate::lookup::tables::{MASK_RANK, CLEAR_RANK, MASK_FILE, CLEAR_FILE, PIECE, SQUARE};


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

pub fn bit_scan (bit: BitBoard) -> usize {
    let remainder = (bit % 67) as usize;
    MOD67[remainder]
}

pub fn bit_count (bit: BitBoard) -> usize {
    bit.count_ones() as usize
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
pub fn compute_king_attacks(king: BitBoard, own_pieces: BitBoard) -> BitBoard {
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
pub fn compute_knight_attacks(knight: BitBoard, own_pieces: BitBoard) -> BitBoard {
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
pub fn compute_white_pawn_attacks(white_pawn: BitBoard, black_pieces: BitBoard) -> BitBoard {
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
pub fn compute_white_pawn_moves(white_pawn: BitBoard, all_pieces: BitBoard, black_pieces: BitBoard) -> BitBoard {
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
pub fn compute_black_pawn_attacks (black_pawn: BitBoard, white_pieces: BitBoard) -> BitBoard {
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
pub fn compute_black_pawn_moves(black_pawn: BitBoard, all_pieces: BitBoard, white_pieces: BitBoard) -> BitBoard {
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
pub fn compute_bishop_attacks(bishop: BitBoard, all_pieces: BitBoard, enemy_pieces: BitBoard) -> BitBoard {
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
pub fn compute_rook_attacks(rook: BitBoard, all_pieces: BitBoard, enemy_pieces: BitBoard) -> BitBoard {
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

}