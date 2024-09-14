mod lookup;
mod compute;

#[allow(unused_imports)]
use crate::lookup::tables::{MASK_RANK, CLEAR_RANK, MASK_FILE, CLEAR_FILE, PIECE, SQUARE, string_to_square};
use crate::compute::patterns::{
    bit_scan,
    bit_count,
    compute_king_attacks, 
    compute_knight_attacks, 
    compute_white_pawn_attacks,
    compute_white_pawn_moves,
    compute_black_pawn_attacks,
    compute_black_pawn_moves,
    compute_bishop_attacks,
    compute_rook_attacks
};


/*

TODO:

Need a better way to do things differently for the different piece types, 12 if statements are ugly!

* Board representation
* -     Currently -> 12 piece-types x 64 bit mask AKA bitboards + 3 useful bitboards (white-pieces, black-pieces, all-pieces)
*
* Game mechanics
* D     Bit Scan (position of a bit)
* D     Square_To_Piece (fn square -> piece on that square)
* D     Attack patterns (Completed apart from handling pins/checks)
* D         Pawn
* D         Knight
* D         Bishop
* D         Rook
* D         Queen
* D         King
* D     Is player in check
* D     Single piece possible moves (is in check-handling aswell)
* D     All pieces possible moves (Probably good for checkmate handling)
* D     Move piece (if possible)
* -     Is position checkmate
* -     Promoting (To N,B,R,Q)
* -     En passant
* -     Castling
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
* -         Comparing amount of possible moves, with stockfish calculation


*/


type BitBoard = u64;

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

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameResult {
    Ongoing,
    White,
    Draw,
    Black
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

    /* Game Info */
    // Players turn
    pub whites_turn: bool,
    // Square of possible en passant
    en_passant_square: BitBoard,
    // State of the game
    pub game_result: GameResult,
    // K Q k q
    pub castling_rights: (bool, bool, bool, bool),
    // moves since last capture/pawn move
    pub halfmove_clock: i32,
    // moves since start (increment after blacks turn)
    pub fullmove: i32,
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

            /* Game Info */
            whites_turn: true,
            en_passant_square: 0,
            game_result: GameResult::Ongoing,
            castling_rights: (true, true, true, true),
            halfmove_clock: 0,
            fullmove: 1
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
        
        /* Game Info */
        self.whites_turn = true;
        self.en_passant_square = 0;
        self.game_result = GameResult::Ongoing;
        self.castling_rights = (true, true, true, true);
        self.halfmove_clock = 0;
        self.fullmove = 1;
    }


    /*
    Combine all attack patterns into one attack function, that returns a bitboard of every square currently attacked by one side.
    Note - pieces may be pinned yet can still attack a square - 
        ex. Bishop could be pinned down on A2, By a rook on A3 when king is on A1 -> Still attacks B3,C4... and could check the other king
    */
    pub fn compute_white_attacks (&self, black_pieces_option: Option<BitBoard>, white_pieces_option: Option<BitBoard>) -> BitBoard {
        // Option to use a modified version of pieces
        let black_pieces: BitBoard = black_pieces_option.unwrap_or(self.black_pieces);
        let white_pieces: BitBoard = white_pieces_option.unwrap_or(self.white_pieces);
        
        let all_pieces = black_pieces | white_pieces;
        
        let mut attacks: BitBoard = compute_white_pawn_attacks(self.white_pawns, black_pieces)
                | compute_knight_attacks(self.white_knights, self.white_pieces)
                | compute_king_attacks(self.white_kings, white_pieces);

        for i in 0..64 {
            if self.white_bishops & PIECE[i] != 0{
                attacks |= compute_bishop_attacks(PIECE[i], all_pieces, black_pieces);
            }
            if self.white_rooks & PIECE[i] != 0 {
                attacks |= compute_rook_attacks(PIECE[i], all_pieces, black_pieces);
            }
            if self.white_queens & PIECE[i] != 0{
                attacks |= compute_bishop_attacks(PIECE[i], all_pieces, black_pieces);
                attacks |= compute_rook_attacks(PIECE[i], all_pieces, black_pieces);
            }
        }

        attacks
    }

    /*
    Combine all attack patterns into one attack function, that returns a bitboard of every square currently attacked by one side.
    Note - pieces may be pinned yet can still attack a square - 
        ex. Bishop could be pinned down on A2, By a rook on A3 when king is on A1 -> Still attacks B3,C4... and could check the other king
    */
    pub fn compute_black_attacks (&self, black_pieces_option: Option<BitBoard>, white_pieces_option: Option<BitBoard>) -> BitBoard {
        // Option to use a modified version of pieces
        let black_pieces: BitBoard = black_pieces_option.unwrap_or(self.black_pieces);
        let white_pieces: BitBoard = white_pieces_option.unwrap_or(self.white_pieces);

        let all_pieces = black_pieces | white_pieces;

        let mut attacks: BitBoard = compute_black_pawn_attacks(self.black_pawns, white_pieces)
                | compute_knight_attacks(self.black_knights, black_pieces)
                | compute_king_attacks(self.black_kings, black_pieces);

        for i in 0..64 {
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
    Option to use altered state of blacks attacking pieces, and/or a moved white king
    */
    fn white_in_check (&self, black_attacks_option: Option<BitBoard>, white_kings_option: Option<BitBoard>) -> bool {
        let white_kings: BitBoard = white_kings_option.unwrap_or(self.white_kings);
        let black_attacks: BitBoard = black_attacks_option.unwrap_or(
            self.compute_black_attacks(None, None)
        );

        return white_kings & black_attacks != 0;
    }

    /*
    Function test whether black in check
    Option to use altered state of whites attacking pieces, and/or a moved black king
    */
    fn black_in_check (&self, white_attacks_option: Option<BitBoard>, black_kings_option: Option<BitBoard>) -> bool {
        let black_kings: BitBoard = black_kings_option.unwrap_or(0);
        let white_attacks: BitBoard = white_attacks_option.unwrap_or(0);
        
        // Test for when white king has moved,
        if black_kings_option.is_some() {
            return PIECE[bit_scan(black_kings)] & white_attacks != 0;
        }

        // Test for when some black piece has moved
        if white_attacks_option.is_some() {
            return PIECE[bit_scan(self.black_kings)] & white_attacks != 0;
        }
        
        // Test for current position
        let white_attacks: BitBoard = self.compute_white_attacks(None, None);

        return PIECE[bit_scan(self.black_kings)] & white_attacks != 0;
    }

    fn white_in_checkmate(&self) -> bool {
        // White has to be in check
        if self.white_in_check(None, None) == false { return false; }

        // Test if white has any possible moves
        for i in 0..64 {
            if self.white_pieces & PIECE[i] == 0 { continue; }
            if self.get_moves(i) != 0 { return false; }
        }
        true
    }

    fn black_in_checkmate(&self) -> bool {
        // Black has to be in check
        if self.black_in_check(None, None) == false { return false; }

        // Test if black has any possible moves
        for i in 0..64 {
            if self.black_pieces & PIECE[i] == 0 { continue; }
            if self.get_moves(i) != 0 { return false; }
        }
        true
    }

    fn white_in_stalemate(&self) -> Result<bool, String> {
        // If it's not whites turn, white cant be in stalemate
        if !self.whites_turn {
            return Err("Not whites turn -> not stalemate".to_string());
        }
        // If white is in check, no stalemate is possible
        if self.white_in_check(None, None) {
            return Err("White is in check -> not stalemate".to_string());
        }

        // Test if white has any possible moves no stalemate is possible
        for i in 0..64 {
            if self.white_pieces & PIECE[i] == 0 { continue; }
            if self.get_moves(i) != 0 { return Err(format!("White piece at {} -> not stalemate", i)); }
        }
        
        Ok(true)
    }

    fn black_in_stalemate(&self) -> Result<bool, String> {
        // If it's not whites turn, white cant be in stalemate
        if self.whites_turn {
            return Err("Not blacks turn -> not stalemate".to_string());
        }
        // If black is in check, no stalemate is possible
        if self.black_in_check(None, None) {
            return Err("Black is in check -> not stalemate".to_string());
        }

        // Test if black has any possible moves no stalemate is possible
        for i in 0..64 {
            if self.black_pieces & PIECE[i] == 0 { continue; }
            if self.get_moves(i) != 0 { return Err(format!("White piece at {} -> not stalemate", i)); }
        }        
        
        Ok(true)
    }


    /*
    Get BitBoard of possible moves a piece
    */
    pub fn get_moves (&self, position: usize) -> BitBoard {
        let square: BitBoard = PIECE[position];
        let mut moves = 0;
        let piece_type = self.piece_at(position);

        // Can't move piece if it's not that sides turn
        //if self.whites_turn && !is_white(piece_type) { return 0; }
        //if !self.whites_turn && is_white(piece_type) { return 0; }

        match piece_type {
            PieceType::WhiteKing =>     moves |= compute_king_attacks(square, self.white_pieces),
            PieceType::WhiteQueen =>    moves |= compute_rook_attacks(square, self.all_pieces, self.black_pieces)
                                              | compute_bishop_attacks(square, self.all_pieces, self.black_pieces),
            PieceType::WhiteRook =>     moves |= compute_rook_attacks(square, self.all_pieces, self.black_pieces),
            PieceType::WhiteBishop =>   moves |= compute_bishop_attacks(square, self.all_pieces, self.black_pieces),
            PieceType::WhiteKnight =>   moves |= compute_knight_attacks(square, self.white_pieces),
            PieceType::WhitePawn =>     moves |= compute_white_pawn_moves(square, self.all_pieces, self.black_pieces | self.en_passant_square),
            PieceType::BlackKing =>     moves |= compute_king_attacks(square, self.black_pieces),
            PieceType::BlackQueen =>    moves |= compute_rook_attacks(square, self.all_pieces, self.white_pieces)
                                              | compute_bishop_attacks(square, self.all_pieces, self.white_pieces),
            PieceType::BlackRook =>     moves |= compute_rook_attacks(square, self.all_pieces, self.white_pieces),
            PieceType::BlackBishop =>   moves |= compute_bishop_attacks(square, self.all_pieces, self.white_pieces),
            PieceType::BlackKnight =>   moves |= compute_knight_attacks(square, self.black_pieces),
            PieceType::BlackPawn =>     moves |= compute_black_pawn_moves(square, self.all_pieces, self.white_pieces | self.en_passant_square),
            _ => moves = 0
        }

        let is_white: bool = is_white(piece_type);

        // Check if this move places own side in check
        if piece_type != PieceType::WhiteKing && piece_type != PieceType::BlackKing {
            for i in 0..64 {
                if moves & PIECE[i] == 0 { continue; }
                // If white moved a piece (not a king)
                if is_white && self.white_in_check(
                        Some(self.compute_black_attacks(
                            Some(self.black_pieces),
                            Some(self.white_pieces & !square | PIECE[i])
                        )), 
                        None
                    ) {
                    moves &= !PIECE[i];
                } 
                // If black moved a piece (not a king)
                else if self.black_in_check(Some(
                    self.compute_white_attacks(
                        Some(self.black_pieces & !square | PIECE[i]),
                        Some(self.white_pieces)
                    )
                ), None ) {
                    moves &= !PIECE[i];
                }
            }
        }
        else {
            // Piece is a king
            for i in 0..64 {
                if moves & PIECE[i] == 0 { continue; }
                // If white moved the king
                if is_white && self.white_in_check(
                        Some(self.compute_black_attacks(
                            Some(self.black_pieces & !PIECE[i]), 
                            Some(self.white_pieces & !square | PIECE[i]))
                        ),
                        Some(PIECE[i])
                    ) {
                    moves &= !PIECE[i];
                } 
                // If black moved the king
                else if !is_white && self.black_in_check(Some(
                    self.compute_white_attacks(
                        Some(self.black_pieces & !square | PIECE[i]),
                         Some(self.white_pieces)
                    )), Some(PIECE[i])) {
                    moves &= !PIECE[i];

                }
            }
            
            // Add castling moves - Need another implementation for Fischer Random etc.
            // Whites Kingside
            if is_white && self.castling_rights.0 &&
                self.all_pieces & PIECE[5] == 0 &&
                self.all_pieces & PIECE[6] == 0 &&
                !self.white_in_check(None, None) &&
                !self.white_in_check(Some(self.compute_black_attacks(
                    Some(self.black_pieces), 
                    Some(self.white_pieces & !square | PIECE[5]))
                    ), Some(PIECE[5])) &&
                !self.white_in_check(Some(self.compute_black_attacks(
                    Some(self.black_pieces), 
                    Some(self.white_pieces & !square | PIECE[6]))
                    ), Some(PIECE[6])) {
                    moves |= PIECE[6];
            }

            // Whites Queenside
            if is_white && self.castling_rights.1 &&
                self.all_pieces & PIECE[3] == 0 &&
                self.all_pieces & PIECE[2] == 0 &&
                self.all_pieces & PIECE[1] == 0 &&
                !self.white_in_check(None, None) &&
                !self.white_in_check(Some(self.compute_black_attacks(
                    Some(self.black_pieces), 
                    Some(self.white_pieces & !square | PIECE[3]))
                    ), Some(PIECE[3])) &&
                !self.white_in_check(Some(self.compute_black_attacks(
                    Some(self.black_pieces), 
                    Some(self.white_pieces & !square | PIECE[2]))
                    ), Some(PIECE[2])) {
                    moves |= PIECE[2];
            }
            // Blacks Kingside
            if !is_white && self.castling_rights.2 &&
                self.all_pieces & PIECE[8*7+5] == 0 &&
                self.all_pieces & PIECE[8*7+6] == 0 &&
                !self.black_in_check(None, None) &&
                !self.black_in_check(Some(
                    self.compute_white_attacks(
                        Some(self.black_pieces & !square | PIECE[8*7+5]),
                         Some(self.white_pieces)
                    )), Some(PIECE[8*7+5])) &&
                !self.black_in_check(Some(
                    self.compute_white_attacks(
                        Some(self.black_pieces & !square | PIECE[8*7+6]),
                         Some(self.white_pieces)
                    )), Some(PIECE[8*7+6])) {
                    moves |= PIECE[8*7+6];
            }
            // Blacks Queenside
            if !is_white && self.castling_rights.3 &&
                self.all_pieces & PIECE[8*7+3] == 0 &&
                self.all_pieces & PIECE[8*7+2] == 0 &&
                self.all_pieces & PIECE[8*7+1] == 0 &&
                !self.black_in_check(None, None) &&
                !self.black_in_check(Some(
                    self.compute_white_attacks(
                        Some(self.black_pieces & !square | PIECE[8*7+3]),
                         Some(self.white_pieces)
                    )), Some(PIECE[8*7+3])) &&
                !self.black_in_check(Some(
                    self.compute_white_attacks(
                        Some(self.black_pieces & !square | PIECE[8*7+2]),
                         Some(self.white_pieces)
                    )), Some(PIECE[8*7+2])) {
                    moves |= PIECE[8*7+2];
            }
        }

        moves
    }

    pub fn piece_at (&self, position: usize) -> PieceType {
        let square: BitBoard = PIECE[position];
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

    pub fn move_piece (&mut self, from: usize, to: usize) -> Result<bool, String> {
        if self.game_result != GameResult::Ongoing {
            return Err("Game is finished".to_string());
        }

        // if piece doesn't exist
        if self.all_pieces & PIECE[from] == 0 { return Err("Piece doesn't exist".to_string()); }

        let moves = self.get_moves(from);

        // break if piece cant move to desired position
        if moves == 0 { return Err("Piece can't move at all".to_string()); }
        if moves & PIECE[to] == 0 { return Err("Piece can't move to desired square".to_string()); }

        let piece_type: PieceType = self.piece_at(from);
        
        // Can't move piece if it's not that sides turn
        if self.whites_turn && !is_white(piece_type) { return Err("Not black's turn".to_string()); }
        if !self.whites_turn && is_white(piece_type) { return Err("Not white's turn".to_string()); }

        // Store if piece was captured (for halfmove clock)
        let mut capture: bool = false;
        if self.all_pieces & PIECE[to] != 0 { capture = true; }

        // Move piece in bitboards
        self.update_board_after_move(piece_type, from, to);

        // Handle castling
        if piece_type == PieceType::WhiteKing {
            // White Kingside
            if self.castling_rights.0 && to == 6 {
                self.update_board_after_move(PieceType::WhiteRook, 7, 5);
            }
            // White Queenside
            if self.castling_rights.1 && to == 2 {
                self.update_board_after_move(PieceType::WhiteRook, 0, 3);
            }
        }
        if piece_type == PieceType::BlackKing {
            // Black Kingside
            if self.castling_rights.2 && to == 8*7+6 {
                self.update_board_after_move(PieceType::BlackRook, 8*7+7, 8*7+5);
            }
            // White Kingside
            if self.castling_rights.3 && to == 8*7+2 {
                self.update_board_after_move(PieceType::BlackRook, 8*7+0, 8*7+3);
            }
        }

        // Handle en passant moves
        if piece_type == PieceType::WhitePawn && PIECE[to] == self.en_passant_square {
            self.update_board_after_move(PieceType::BlackPawn, to-8, 64)
        }
        if piece_type == PieceType::BlackPawn && PIECE[to] == self.en_passant_square {
            self.update_board_after_move(PieceType::WhitePawn, to+8, 64)
        }

        // Detect possible en passant square
        self.en_passant_square = 0;
        if piece_type == PieceType::WhitePawn && from / 8 == 1 && to / 8 == 3 {
            self.en_passant_square = PIECE[from + 8 as usize];
        }
        if piece_type == PieceType::BlackPawn && from / 8 == 6 && to / 8 == 4 {
            self.en_passant_square = PIECE[from - 8 as usize];
        }

        // Halfmove clock
        self.halfmove_clock += 1;
        if piece_type == PieceType::WhitePawn || piece_type == PieceType::BlackPawn || capture {
            self.halfmove_clock = 0;
        }

        // Add fullmove if black just moved
        if self.whites_turn == false {
            self.fullmove += 1;
        }

        // Handle castling-rights
        if piece_type == PieceType::WhiteKing {
            self.castling_rights.0 = false;
            self.castling_rights.1 = false;
        }
        if piece_type == PieceType::BlackKing {
            self.castling_rights.2 = false;
            self.castling_rights.3 = false;
        }
        if from == SQUARE::H1 { self.castling_rights.0 = false; }
        if from == SQUARE::A1 { self.castling_rights.1 = false; }
        if from == SQUARE::H8 { self.castling_rights.0 = false; }
        if from == SQUARE::A8 { self.castling_rights.1 = false; }


        self.update_board();

        // Change player turn
        self.whites_turn = !self.whites_turn;

        return Ok(true);
    }

    fn update_board_after_move (&mut self, piece_type: PieceType, from: usize, to: usize) {
        self.white_pawns &= !PIECE[to] & !PIECE[from];
        self.white_knights &= !PIECE[to] & !PIECE[from];
        self.white_bishops &= !PIECE[to] & !PIECE[from];
        self.white_rooks &= !PIECE[to] & !PIECE[from];
        self.white_queens &= !PIECE[to] & !PIECE[from];
        self.white_kings &= !PIECE[to] & !PIECE[from];
        self.black_pawns &= !PIECE[to] & !PIECE[from];
        self.black_knights &= !PIECE[to] & !PIECE[from];
        self.black_bishops &= !PIECE[to] & !PIECE[from];
        self.black_rooks &= !PIECE[to] & !PIECE[from];
        self.black_queens &= !PIECE[to] & !PIECE[from];
        self.black_kings &= !PIECE[to] & !PIECE[from];

        match piece_type {
            PieceType::WhitePawn => {  self.white_pawns |= PIECE[to]; },
            PieceType::WhiteKnight => {self.white_knights |= PIECE[to]; },
            PieceType::WhiteBishop => {self.white_bishops |= PIECE[to]; },
            PieceType::WhiteRook => {  self.white_rooks |= PIECE[to]; },
            PieceType::WhiteQueen => { self.white_queens |= PIECE[to]; },
            PieceType::WhiteKing => {  self.white_kings |= PIECE[to]; },

            PieceType::BlackPawn => {  self.black_pawns |= PIECE[to]; },
            PieceType::BlackKnight => {self.black_knights |= PIECE[to]; },
            PieceType::BlackBishop => {self.black_bishops |= PIECE[to]; },
            PieceType::BlackRook => {  self.black_rooks |= PIECE[to]; },
            PieceType::BlackQueen => { self.black_queens |= PIECE[to]; },
            PieceType::BlackKing => {  self.black_kings |= PIECE[to]; },
            _ => panic!("No Piece Type")
        }
    }
    
    pub fn count_moves(&self) -> usize {
        let own_pieces = if self.whites_turn { self.white_pieces } else { self.black_pieces };
        let mut count = 0;

        for i in 0..64 {
            if own_pieces & PIECE[i] != 0 {
                let moves = self.get_moves(i);
                
                count += bit_count(moves);
            }
        }

        count
    }

    
    pub fn load (&mut self, fen: String) {
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

        // Read player turn
        if fen_vec.len() >= 2 &&  fen_vec[1] == "w" {
            self.whites_turn = true;
        }
        if fen_vec.len() >= 2 &&  fen_vec[1] == "b" {
            self.whites_turn = false;
        }

        // Read castling rights
        self.castling_rights = (false, false, false, false);
        if fen_vec.len() >= 3 {
            if fen_vec[2].chars().nth(0).unwrap() == 'K' && self.white_kings & PIECE[4] != 0 && self.white_rooks & PIECE[7] != 0 { 
                self.castling_rights.0 = true; 
            }
            if fen_vec[2].chars().nth(1).unwrap() == 'Q' && self.white_kings & PIECE[4] != 0 && self.white_rooks & PIECE[0] != 0 { 
                self.castling_rights.1 = true; 
            }
            if fen_vec[2].chars().nth(2).unwrap() == 'k' && self.black_kings & PIECE[8*7+4] != 0 && self.black_rooks & PIECE[8*7+7] != 0 { 
                self.castling_rights.2 = true; 
            }
            if fen_vec[2].chars().nth(3).unwrap() == 'q' && self.black_kings & PIECE[8*7+4] != 0 && self.black_rooks & PIECE[8*7+0] != 0 { 
                self.castling_rights.3 = true; 
            }
        }

        // Read en passant square
        if fen_vec.len() >= 4 {
            let sq = string_to_square(fen_vec[3].to_string());
            if sq != 64 {
                self.en_passant_square = PIECE[sq];
            }
        }

        // Read halfmove clock
        if fen_vec.len() >= 5 {
            self.halfmove_clock = fen_vec[4].parse().unwrap_or(0);
        }
        
        // Read fullmove count
        if fen_vec.len() >= 6 {
            self.fullmove = fen_vec[5].parse().unwrap_or(0);
        }

        self.game_result = GameResult::Ongoing;

        // Update the derived boards
        self.update_board();
    }

    // Updates the derived boards
    fn update_board (&mut self) {
        self.white_pieces = self.white_pawns | self.white_knights | self.white_bishops | self.white_rooks | self.white_queens | self.white_kings;
        self.black_pieces = self.black_pawns | self.black_knights | self.black_bishops | self.black_rooks | self.black_queens | self.black_kings;
        self.all_pieces = self.white_pieces | self.black_pieces;

        // Check if board is in checkmate / stalemate
        if self.black_in_checkmate() {
            self.game_result = GameResult::White;
        }
        if self.white_in_checkmate() {
            self.game_result = GameResult::Black;
        }
        if self.black_in_stalemate().is_ok() {
            self.game_result = GameResult::Draw;
        }
        
        if self.white_in_stalemate().is_ok() {
            self.game_result = GameResult::Draw;
        }
    }


    pub fn reset (&mut self) {
        self.load("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
    }

    pub fn print_board(&self, b: BitBoard){
        print!("\n");
        for y in (0..8).rev() {
            print!("{}   ", y+1);
            for x in 0..8 {
                let bitval: BitBoard = (1 as BitBoard) << (y*8 + x);
                if b & bitval != 0 { print!("1 ");  }
                else if self.all_pieces & bitval != 0 {
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
                    print!(". ");
                }
            }
            print!("\n");
        }
        print!("\n");
        print!("    A B C D E F G H\n");
        print!("-------------------\n");
    }
}

pub fn game () -> ChessBoard {
    let mut chess = ChessBoard { ..Default::default() };
    chess.reset();

    chess
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let mut chess: ChessBoard = game(); 
        println!("Game Loaded!");

        chess.print_board(0);

        let _ = chess.move_piece(SQUARE::E2,SQUARE::E4);
        chess.print_board(0);
        println!("Possible moves -> {}", chess.count_moves());

        let _ = chess.move_piece(SQUARE::E7,SQUARE::E5);
        chess.print_board(0);
        println!("Possible moves -> {}", chess.count_moves());

        let _ = chess.move_piece(SQUARE::F1,SQUARE::C4);
        chess.print_board(0);
        println!("Possible moves -> {}", chess.count_moves());
        

        let _ = chess.move_piece(SQUARE::A7,SQUARE::A6);
        chess.print_board(0);
        println!("Possible moves -> {}", chess.count_moves());

        let _ = chess.move_piece(SQUARE::C4,SQUARE::F7);
        chess.print_board(0);
        println!("Possible moves -> {}", chess.count_moves());

        assert_eq!(2,2);
    }

    #[test]
    fn test_possible_moves() {
        // POS where white is in check
        let mut chess: ChessBoard = game(); 
        chess.load("2k5/8/4q3/8/6b1/1n6/1PPP4/3KR3".to_string());
        chess.print_board(chess.compute_black_attacks(None, None));
        chess.print_board(chess.compute_white_attacks(None, None));
        println!("Possible moves -> {}", chess.count_moves());

    }

    #[test]
    fn play_game() {
        let mut chess: ChessBoard = game(); 
        chess.load("k7/8/8/8/8/8/8/7K".to_string());

        //chess.print_board(compute_king_attacks(PIECE[7], PIECE[7]));
        //chess.print_board(compute_king_attacks(PIECE[7*8], PIECE[7*8]));
        //let     stdin = std::io::stdin();
        //let mut input = stdin.lock().lines();
        
        
    }

    #[test]
    fn castling() {
        let mut chess: ChessBoard = game();
        chess.load("r3k2r/pppp1ppp/4p2b/8/8/B2P4/PPP1PPPP/R3K2R w KQkq - 0 1".to_string());
        
        //chess.print_board(0);

        // Cant castle into check
        //chess.print_board(chess.get_moves(SQUARE::E1));
        assert!(chess.move_piece(SQUARE::E1, SQUARE::C1).is_err());
        assert!(chess.move_piece(SQUARE::E1, SQUARE::G1).is_ok());
        
        // Cant castle through check
        //chess.print_board(chess.get_moves(SQUARE::E8));
        assert!(chess.move_piece(SQUARE::E8, SQUARE::G8).is_err());
        assert!(chess.move_piece(SQUARE::E8, SQUARE::C8).is_ok());

        chess.print_board(0);
    }

    #[test]
    fn white_in_stalemate() {
        let mut chess: ChessBoard = game();
        chess.load("k5rr/8/8/8/8/8/7p/7K w ---- - 0 1".to_string());
        /*
        chess.print_board(0);
        chess.print_board(chess.get_moves(7));
        */

        assert_eq!(chess.game_result, GameResult::Draw);
    }

    #[test]
    fn black_in_stalemate() {
        let mut chess: ChessBoard = game();
        chess.load("k7/8/8/8/8/8/5B2/1R5K b ---- - 0 1".to_string());
        /*
        chess.print_board(0);
        chess.print_board(chess.get_moves(7*8));
        */
    

        assert_eq!(chess.game_result, GameResult::Draw);
    }


    #[test]
    fn white_in_check() {
        let mut chess: ChessBoard = game();
        chess.load("2k5/8/4q3/8/6b1/1n6/1PPP4/3KR3".to_string());
        assert_eq!(chess.white_in_check(None, None), true);

        chess.load("k6q/8/8/8/8/8/8/7K".to_string());
        assert_eq!(chess.white_in_check(None, None), true);
    }

    #[test]
    fn en_passant() {
        let mut chess: ChessBoard = game();
        chess.load("rnbqkbnr/1p3p1p/8/P1PpP1P1/p1p1p1pP/8/1P1P1P2/RNBQKBNR w KQkq d6 0 1".to_string());
        
        // White can do en passant at d6 (due to fen string recording d6)
            //chess.print_board(chess.get_moves(SQUARE::E5));
        assert!(chess.move_piece(SQUARE::E5, SQUARE::D6).is_ok());
        assert_eq!(chess.piece_at(SQUARE::D5), PieceType::Other);
        
        // Black cant do en passant at h3 (due to that move not being made last turn)
            //chess.print_board(chess.get_moves(SQUARE::G4));
        assert!(chess.move_piece(SQUARE::G4, SQUARE::H3).is_err());
        assert_eq!(chess.piece_at(SQUARE::H4), PieceType::WhitePawn);

        // Move white to allow en passant at b3
        assert!(chess.move_piece(SQUARE::G4, SQUARE::G3).is_ok());
        assert!(chess.move_piece(SQUARE::B2, SQUARE::B4).is_ok());

        // Both pawn A4, and pawn C4 should be able to do en passant at B#
            //chess.print_board(chess.get_moves(SQUARE::A4));
            //chess.print_board(chess.get_moves(SQUARE::C4));
        assert!(chess.move_piece(SQUARE::A4, SQUARE::B3).is_ok());
        assert_eq!(chess.piece_at(SQUARE::B4), PieceType::Other);

        //chess.print_board(0);
    }

}
