mod lookup;
mod compute;

#[warn(missing_docs)]
#[allow(unused_imports)]
use crate::lookup::tables::{MASK_RANK, CLEAR_RANK, MASK_FILE, CLEAR_FILE, PIECE, SQUARE, string_to_square};
use crate::compute::patterns::{
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

Some parts of this code is really ugly :(

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
* D     Is position checkmate
* D     Is position stalemate
* D     Promoting (To N,B,R,Q)
* D     En passant
* D     Castling
*
* FEN data
* D     Import FEN
* -         Detect if FEN is allowed as a chess-game
* -     Export FEN
* D     Player turn,
* D     En passant target square (If a pawn moves 2 places -> store the square behind it)
* D     Castling rights (king-side, queen-side, black, white)
* D     Halfmove count
* 
* Special positions/rules
* D     Moves since last pawn move or capture (for 50 move rule)
* D     Store old positions (for 3-fold repetition)
* D         Data-Format (perhaps - 12(piece-types) * 64bit mask AKA bitboards)
* D         Store castling-rights - unique positions if castling rights differ
* D         Store whether the possibility of en passant exists
* -             (currently not accounting for pinned pawns)

*
* Unit Testing
* -     Testing special positions:
* D         En passant
* D         Checkmate
* D         Stalemate
* D         Three-move rule
* D         Importing series of FEN-positions of a game
* D         Comparing amount of possible moves, with stockfish calculation


*/


type BitBoard = u64;


/// Enum PieceType contains different types pieces in the game (and Empty)
/// 
/// Whites Pieces: PieceType::WhitePawn, PieceType::WhiteKnight, PieceType::WhiteBishop, PieceType::WhiteRook, PieceType::WhiteQueen, PieceType::WhiteKing
/// 
/// # Examples
/// 
/// ```
/// use davbjor_chess::{ChessBoard, PieceType};
/// 
/// // Create a new game
/// let mut chess = ChessBoard::new();
/// 
/// //Check the there is a white rook at square 7 (H1)
/// if chess.board[7] == PieceType::WhiteRook {
///     // Get the moves of the rook
///     let rook_moves: Vec<usize> = chess.get_moves_list(7);
/// }
/// 
/// ```
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
    Empty
}

impl PieceType {
    /// Checks if the piece is white
    /// 
    /// Returns true if piece is white
    /// 
    /// Returns false if piece is black
    /// 
    /// # Examples
    /// 
    /// ```
    /// use davbjor_chess::{ChessBoard, PieceType};
    /// 
    /// let mut chess = ChessBoard::new();
    /// 
    /// // Check if the piece at 7 (H1)
    /// if chess.board[7].is_white() {
    ///     // The piece is white
    /// }
    /// ```
    /// 
    pub fn is_white(&self) -> bool {
        return match self {
            PieceType::WhitePawn => true,
            PieceType::WhiteKnight => true,
            PieceType::WhiteBishop => true,
            PieceType::WhiteRook => true,
            PieceType::WhiteQueen => true,
            PieceType::WhiteKing => true,
            _ => false
        }
    }
    /// Checks if the piece is a king
    /// 
    /// Returns true if piece is a king
    /// 
    /// Returns false if piece is not a king
    /// 
    /// # Examples
    /// 
    /// ```
    /// use davbjor_chess::{ChessBoard, PieceType};
    /// 
    /// let mut chess = ChessBoard::new();
    /// 
    /// // Check if the piece at 7 (H1)
    /// if chess.board[4].is_king() {
    ///     // The piece is a king
    /// }
    /// ```
    /// 
    pub fn is_king(&self) -> bool {
        return match self {
            PieceType::WhiteKing => true,
            PieceType::BlackKing => true,
            _ => false
        }
    }
    /// Checks if the piece is a pawn
    /// 
    /// Returns true if piece is a pawn
    /// 
    /// Returns false if piece is not a pawn
    /// 
    /// # Examples
    /// 
    /// ```
    /// use davbjor_chess::{ChessBoard, PieceType};
    /// 
    /// let mut chess = ChessBoard::new();
    /// 
    /// // Check if the piece at 7 (H1)
    /// if chess.board[4].is_pawn() {
    ///     // The piece is a pawn
    /// }
    /// ```
    /// 
    pub fn is_pawn(&self) -> bool {
        return match self {
            PieceType::WhitePawn => true,
            PieceType::BlackPawn => true,
            _ => false
        }
    }
}

/// Enum GameResult contains types of state of the game
/// 
/// Either game is ongoing - GameResult::Ongoing
/// Or either side has won - GameResult::White / GameResult::Black
/// Or game is a draw - GameResult::Draw
/// 
/// # Examples
/// 
/// ```
/// use davbjor_chess::{ChessBoard, GameResult};
/// 
/// // Create a new game
/// let mut chess = ChessBoard::new();
/// 
/// //Check the state of the game
/// match chess.game_result {
///     GameResult::White => { println!("White has won!"); },
///     GameResult::Black => { println!("Black has won!"); },
///     GameResult::Draw => { println!("Game is a draw!"); },
///     _ => ()
/// } 
/// ```
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameResult {
    Ongoing,
    White,
    Draw,
    Black
}

/// Contains the chessgame and can be altered by it's methods
/// 
/// Stores a chessboard, indexed from down-left -> right -> up
/// A1 = 0, H1 = 8, A8 = 56, H8 = 63
/// 
/// Works by using bitboards (u64), for every piece type
/// 
/// 
/// 
#[derive(Debug, Clone)]
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
    /// Players turn (true if it is whites turn, false if it is blacks)
    pub whites_turn: bool,
    /// State of the game (stored as the enum GameResult)
    pub game_result: GameResult,
    /// Stores the castling_rights of both players (K Q k q) (whites-kingside, whites queenside, blacks kingside, blacks queenside)
    pub castling_rights: (bool, bool, bool, bool),
    /// Moves (counting every move) since last capture/pawn move (useful for calculating 50-move rule)
    pub halfmove_clock: i32,
    /// Moves (times both players played) since start (increments after blacks turn)
    pub fullmove: i32,
    /// Player whos turn it is, is in check
    pub player_in_check: bool,
    /// Board of pieces as 64 squares containing PieceType's 
    pub board: Vec<PieceType>,

    // 
    promotion_piece: PieceType,
    // Square of possible en passant
    en_passant_square: BitBoard,
    // Stores the previous positions
    positions: Vec<Vec<BitBoard>>,
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
            game_result: GameResult::Ongoing,
            castling_rights: (true, true, true, true),
            halfmove_clock: 0,
            fullmove: 1,
            player_in_check: false,
            board: vec![PieceType::Empty;64],
            
            promotion_piece: PieceType::Empty,
            en_passant_square: 0,
            positions: vec![],
        }
    }
}

impl ChessBoard {
    pub fn new () -> Self {
        ChessBoard {
            /* All White Pieces */
            white_pawns: MASK_RANK[1],
            white_knights: PIECE[1] | PIECE[6],
            white_bishops: PIECE[2] | PIECE[5],
            white_rooks: PIECE[0] | PIECE[7],
            white_queens: PIECE[3],
            white_kings: PIECE[4],
            
            /* All Black Pieces */
            black_pawns: MASK_RANK[6],
            black_knights: PIECE[7*8+1] | PIECE[7*8+6],
            black_bishops: PIECE[7*8+2] | PIECE[7*8+5],
            black_rooks: PIECE[7*8+0] | PIECE[7*8+7],
            black_queens: PIECE[7*8+3],
            black_kings: PIECE[7*8+4],
        
            /* Derived Positions */
            white_pieces: MASK_RANK[0] | MASK_RANK[1],
            black_pieces: MASK_RANK[6] | MASK_RANK[7],
            all_pieces: MASK_RANK[0] | MASK_RANK[1] | MASK_RANK[6] | MASK_RANK[7],

            /* Game Info */
            whites_turn: true,
            game_result: GameResult::Ongoing,
            castling_rights: (true, true, true, true),
            halfmove_clock: 0,
            fullmove: 1,
            player_in_check: false,
            board: vec![
                PieceType::WhiteRook,
                PieceType::WhiteKnight,
                PieceType::WhiteBishop,
                PieceType::WhiteQueen,
                PieceType::WhiteKing,
                PieceType::WhiteBishop,
                PieceType::WhiteKnight,
                PieceType::WhiteRook,
                PieceType::WhitePawn,
                PieceType::WhitePawn,
                PieceType::WhitePawn,
                PieceType::WhitePawn,
                PieceType::WhitePawn,
                PieceType::WhitePawn,
                PieceType::WhitePawn,
                PieceType::WhitePawn,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::Empty,
                PieceType::BlackPawn,
                PieceType::BlackPawn,
                PieceType::BlackPawn,
                PieceType::BlackPawn,
                PieceType::BlackPawn,
                PieceType::BlackPawn,
                PieceType::BlackPawn,
                PieceType::BlackPawn,
                PieceType::BlackRook,
                PieceType::BlackKnight,
                PieceType::BlackBishop,
                PieceType::BlackQueen,
                PieceType::BlackKing,
                PieceType::BlackBishop,
                PieceType::BlackKnight,
                PieceType::BlackRook,
            ],
            
            promotion_piece: PieceType::Empty,
            en_passant_square: 0,
            positions: vec![],
        }
    }
    /// Reset entire board to a blank state
    /// 
    /// # Examples
    /// 
    /// ```
    /// use davbjor_chess::{ChessBoard};
    /// 
    /// // create a new game
    /// let mut chess = ChessBoard::new();
    /// 
    /// // remove every piece from the board
    /// chess.clear();
    /// ```
    /// 
    /// The default state of the chessboard is the initial position of a game
    /// 
    /// This can also be changed by importing a FEN-string position with chess.load(FEN)
    pub fn clear (&mut self) {
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
        self.game_result = GameResult::Ongoing;
        self.castling_rights = (true, true, true, true);
        self.halfmove_clock = 0;
        self.fullmove = 1;
        self.player_in_check = false;

        self.board = vec![PieceType::Empty;64];
        
        self.en_passant_square = 0;
        self.positions = Vec::new();
    }


    /*
    Combine all attack patterns into one attack function, that returns a bitboard of every square currently attacked by one side.
    Note - pieces may be pinned yet can still attack a square - 
        ex. Bishop could be pinned down on A2, By a rook on A3 when king is on A1 -> Still attacks B3,C4... and could check the Empty king
    */
    fn compute_white_attacks (&self, black_pieces_option: Option<BitBoard>, white_pieces_option: Option<BitBoard>) -> BitBoard {
        // Option to use a modified version of pieces
        let black_pieces: BitBoard = black_pieces_option.unwrap_or(self.black_pieces);
        let white_pieces: BitBoard = white_pieces_option.unwrap_or(self.white_pieces);
        
        let all_pieces = black_pieces | white_pieces;
        
        let mut attacks: BitBoard = compute_white_pawn_attacks(white_pieces & self.white_pawns, black_pieces | self.en_passant_square)
                | compute_knight_attacks(white_pieces & self.white_knights, self.white_pieces)
                | compute_king_attacks(white_pieces & self.white_kings, white_pieces);

        for i in 0..64 {
            if white_pieces & self.white_bishops & PIECE[i] != 0{
                attacks |= compute_bishop_attacks(PIECE[i], all_pieces, black_pieces);
            }
            if white_pieces & self.white_rooks & PIECE[i] != 0 {
                attacks |= compute_rook_attacks(PIECE[i], all_pieces, black_pieces);
            }
            if white_pieces & self.white_queens & PIECE[i] != 0{
                attacks |= compute_bishop_attacks(PIECE[i], all_pieces, black_pieces);
                attacks |= compute_rook_attacks(PIECE[i], all_pieces, black_pieces);
            }
        }

        attacks
    }

    /*
    Combine all attack patterns into one attack function, that returns a bitboard of every square currently attacked by one side.
    Note - pieces may be pinned yet can still attack a square - 
        ex. Bishop could be pinned down on A2, By a rook on A3 when king is on A1 -> Still attacks B3,C4... and could check the Empty king
    */
    fn compute_black_attacks (&self, black_pieces_option: Option<BitBoard>, white_pieces_option: Option<BitBoard>) -> BitBoard {
        // Option to use a modified version of pieces
        let black_pieces: BitBoard = black_pieces_option.unwrap_or(self.black_pieces);
        let white_pieces: BitBoard = white_pieces_option.unwrap_or(self.white_pieces);

        let all_pieces = black_pieces | white_pieces;

        let mut attacks: BitBoard = compute_black_pawn_attacks(black_pieces & self.black_pawns, white_pieces)
                | compute_knight_attacks(black_pieces & self.black_knights, black_pieces)
                | compute_king_attacks(black_pieces & self.black_kings, black_pieces);

        for i in 0..64 {
            if black_pieces & self.black_bishops & PIECE[i] != 0{
                attacks |= compute_bishop_attacks(PIECE[i], all_pieces, white_pieces);
            }
            if black_pieces & self.black_rooks & PIECE[i] != 0 {
                attacks |= compute_rook_attacks(PIECE[i], all_pieces, white_pieces);
            }
            if black_pieces & self.black_queens & PIECE[i] != 0{
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
        let black_kings: BitBoard = black_kings_option.unwrap_or(self.black_kings);
        let white_attacks: BitBoard = white_attacks_option.unwrap_or(
            self.compute_white_attacks(None, None)
        );
        
        return black_kings & white_attacks != 0;
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

    fn is_three_fold_repetition(&self) -> bool {
        let mut repetitions = 1;
        let current = &self.positions[self.positions.len() - 1];
        
        // Loop through every stored position and compare it to the last stored position
        for i in 0..&self.positions.len()-1 {
            let vb = &self.positions[i];
            let mut identical = true;

            for i in 0..vb.len() {
                if vb[i] != current[i] { identical = false; break; }
            }

            if !identical { continue; }
            
            repetitions += 1;
        }

        if repetitions >= 3 { return true; }
        return false;
    }

    fn store_position(&mut self) {
        // Store the castling rights as a bitboard
        let mut castling: BitBoard = 0;
        if self.castling_rights.0 { castling |= (1 as BitBoard) << 1; }
        if self.castling_rights.1 { castling |= (1 as BitBoard) << 2; }
        if self.castling_rights.2 { castling |= (1 as BitBoard) << 3; }
        if self.castling_rights.3 { castling |= (1 as BitBoard) << 4; }

        // Store the whether the possibility of en passant exists as bitboard (not accounting for pinned pawns)
        let mut en_passant_possible: BitBoard = 0;
        if self.en_passant_square & 
            (compute_white_pawn_attacks(self.white_pawns, self.en_passant_square)
            | compute_black_pawn_attacks(self.black_pawns, self.en_passant_square)
            ) != 0 {
            en_passant_possible = 1;
        }
        
        let vb: Vec<BitBoard> = vec![
            self.white_pawns,
            self.white_knights,
            self.white_bishops,
            self.white_rooks,
            self.white_queens,
            self.white_kings,
            self.black_pawns,
            self.black_knights,
            self.black_bishops,
            self.black_rooks,
            self.black_queens,
            self.black_kings,
            en_passant_possible,
            castling            
        ];

        self.positions.push(vb);
    }

    /// Get BitBoard of possible moves a piece
    /// 
    fn get_moves (&self, position: usize) -> BitBoard {
        let square: BitBoard = PIECE[position];
        let mut moves = 0;
        let piece_type = self.piece_at(position);
                
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

        let is_white: bool = piece_type.is_white();

        // Check if this move places own side in check
        if !piece_type.is_king() {
            for i in 0..64 {
                if moves & PIECE[i] == 0 { continue; }
                // If white moved a piece (not a king)
                if is_white {
                    let mut black_attacks = self.compute_black_attacks(
                        Some(self.black_pieces & !PIECE[i]),
                        Some(self.white_pieces & !square | PIECE[i])
                    );
                    // Remove pawn if move is en-passant
                    if piece_type == PieceType::WhitePawn && PIECE[i] == self.en_passant_square {
                        black_attacks = self.compute_black_attacks(
                            Some(self.black_pieces & !PIECE[i] & !PIECE[i-8]),
                            Some(self.white_pieces & !square | PIECE[i])
                        );
                    }
                    if self.white_in_check(Some(black_attacks), None) {
                        moves &= !PIECE[i];
                    }
                }
                // If black moved a piece (not a king)
                else {
                    let mut white_attacks = self.compute_white_attacks(
                        Some(self.black_pieces & !square | PIECE[i]),
                        Some(self.white_pieces & !PIECE[i])
                    );
                    // Remove enemy pawn if en-passanted
                    if piece_type == PieceType::WhitePawn && PIECE[i] == self.en_passant_square {
                        white_attacks = self.compute_white_attacks(
                            Some(self.black_pieces & !square | PIECE[i]),
                            Some(self.white_pieces & !PIECE[i] & !PIECE[i+8])
                        );
                    }
                    if self.black_in_check(Some(white_attacks), None ) {
                        moves &= !PIECE[i];
                    }
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
            
            // Add castling moves - Need anEmpty implementation for Fischer Random etc.
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

    /// Returns a list of all squares the piece at a certain position can move to
    /// 
    /// Will only show legal moves of the current players turns own pieces (cant move enemies pieces)
    /// 
    /// 
    pub fn get_moves_list(&self, position: usize) -> Vec<usize> {
        let mut moves: Vec<usize> = vec![];
        if position > 63 { return moves; }

        let piece_type = self.piece_at(position);

        // Can't move piece if it's not that sides turn
        if self.whites_turn && !piece_type.is_white() { return moves; }
        if !self.whites_turn && piece_type.is_white() { return moves; }

        let bb = self.get_moves(position);
        for i in 0..64 {
            if bb & ((1 as BitBoard) << i) != 0 {
                moves.push(i);
            }
        }

        return moves;
    }

    /// Gives the PieceType at a certain square (0-63 inclusive)
    /// 
    /// This method returns the PieceType of the piece at a square
    /// 
    /// Returns PieceType::Empty if no piece is at the square
    /// 
    /// ```
    /// use davbjor_chess::{ChessBoard, PieceType};
    /// let mut chess = ChessBoard::new();
    /// let square: usize = 52;
    /// if chess.piece_at(square) == PieceType::WhiteKing {
    ///     // The piece at square 52 (E7) is the White King
    /// }
    /// ```
    /// 
    pub fn piece_at (&self, position: usize) -> PieceType {
        let square: BitBoard = PIECE[position];
        if self.all_pieces & square == 0 { return PieceType::Empty; }

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

        PieceType::Empty
    }

    /// Handle promotion by moving piece from -> to and promoting to piecetype
    /// 
    /// Should be used in combination with a match to parse if the move was actually made or if it was illegal
    /// 
    /// Returns Ok(true) and moves the piece if it is a legal move
    /// 
    /// Returns Ok(false) without the move is an unhandled promotion   
    /// 
    /// Returns Err(m) without moving the piece if for any reason the piece could not move and gives a message m (String) for the reason why
    /// 
    /// # Examples
    /// 
    /// ```
    /// use davbjor_chess::{ChessBoard, PieceType};
    /// let mut chess = ChessBoard::new();
    /// // From square H2
    /// let fromSquare: usize = 15;
    /// // To square H4
    /// let toSquare: usize = 31;
    /// // Promote to a queen
    /// let new_piece = PieceType::WhiteQueen;
    /// match chess.handle_promotion(fromSquare, toSquare, new_piece) {
    ///     Ok(true) => {
    ///         // Move was made
    ///         ()
    ///     },
    ///     Ok(false) => {
    ///         // Move was not made due to being an unhandled promotion
    ///         ()
    ///     },
    ///     Err(s) => {
    ///         // Move was not made due to error
    ///         println!("Error: {s}")
    ///     }
    /// }
    /// ```
    /// 
    pub fn handle_promotion (&mut self, from: usize, to: usize, piece_type: PieceType) -> Result<bool, String> {
        self.promotion_piece = piece_type;

        if piece_type.is_king() || piece_type.is_pawn() {
            return Err("Can't promote to a king or a pawn".to_string());
        }
        if (self.whites_turn && !piece_type.is_white()) || (!self.whites_turn && piece_type.is_white()) {
            return Err("Wrong color promotion piece".to_string());
        }

        self.move_piece(from, to)
    }

    /// Method to move piece from one square to another square
    /// 
    /// Should be used in combination with a match to parse if the move was actually made or if it was illegal or a promotion
    /// 
    /// Returns Ok(true) and moves the piece if it is a legal move
    /// 
    /// Returns Ok(false) without moving the piece if the move is a promotion (use chess.handle_promotion(from, to, piece_type) instead)  
    /// 
    /// Returns Err(m) without moving the piece if for any reason the piece could not move and gives a message m (String) for the reason why
    /// 
    /// # Examples
    /// 
    /// ```
    /// use davbjor_chess::{ChessBoard};
    /// let mut chess = ChessBoard::new();
    /// // From square H2
    /// let fromSquare: usize = 15;
    /// // To square H4
    /// let toSquare: usize = 31;
    /// match chess.move_piece(fromSquare, toSquare) {
    ///     Ok(true) => {
    ///         // Move was made
    ///         ()
    ///     },
    ///     Ok(false) => {
    ///         // Move was not made due to move being promotion
    ///         // Should do a promotion using do_promotion into the desired PieceType
    ///         // example: chess.do_promotion(fromSquare, toSquare, PieceType::WhiteQueen)
    ///         ()
    ///     },
    ///     Err(s) => {
    ///         // Move was not made due to error
    ///         println!("Error: {s}")
    ///     }
    /// }
    /// ```
    /// 
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
        if self.whites_turn && !piece_type.is_white() { return Err("Not black's turn".to_string()); }
        if !self.whites_turn && piece_type.is_white() { return Err("Not white's turn".to_string()); }

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
        if piece_type.is_pawn() || capture {
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

        // Promotion handling
        if (piece_type == PieceType::WhitePawn && to / 8 == 7) || 
            (piece_type == PieceType::BlackPawn && to / 8 == 0) {
            // Same players turn to specify what piece type to promote to
            //handlePromotion(from, to);
            if self.promotion_piece == PieceType::Empty {
                return Ok(false);
            }
            self.update_board_after_move(self.promotion_piece, to, to);
        }
        
        // Update derived bitboards, check for checkmate, stalemate...
        self.update_board();

        self.store_position();

        if self.is_three_fold_repetition() {
            self.game_result = GameResult::Draw;
        }

        if self.game_result == GameResult::Ongoing && self.halfmove_clock >= 100 {
            self.game_result = GameResult::Draw;
        }

        // Change player turn
        self.whites_turn = !self.whites_turn;

        self.promotion_piece = PieceType::Empty;

        // Detect if player is in check
        self.player_in_check = false;
        if self.whites_turn && self.white_in_check(None, None) {
            self.player_in_check = true;
        }
        if !self.whites_turn && self.black_in_check(None, None) {
            self.player_in_check = true;
        }

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
    
    /// Get the number of possible moves for the current player in a position
    /// 
    /// # Examples
    /// 
    /// ```
    /// use davbjor_chess::{ChessBoard};
    /// // Create a new game
    /// let mut chess = ChessBoard::new();
    /// 
    /// let amount_of_moves = chess.count_moves();
    /// 
    /// ```
    /// 
    pub fn count_moves(&self) -> usize {
        let own_pieces = if self.whites_turn { self.white_pieces } else { self.black_pieces };
        let mut count = 0;

        for i in 0..64 {
            if own_pieces & PIECE[i] != 0 {
                let moves = self.get_moves(i);
                
                // Count promotion extra times
                if self.piece_at(i) == PieceType::WhitePawn && i / 8 == 6 {
                    count += bit_count(moves & MASK_RANK[7]) * 3;
                }
                if self.piece_at(i) == PieceType::BlackPawn && i / 8 == 1 {
                    count += bit_count(moves & MASK_RANK[0]) * 3;
                }
                count += bit_count(moves);
            }
        }

        count
    }

    
    /// Imports a position by a FEN-string into the game
    /// 
    /// # Examples
    /// 
    /// ```
    /// use davbjor_chess::{ChessBoard};
    /// 
    /// // create a new game
    /// let mut chess = ChessBoard::new();
    /// 
    /// let fen = "rnbqkbnr/1p3p1p/8/P1PpP1P1/p1p1p1pP/8/1P1P1P2/RNBQKBNR w KQkq d6 0 1".to_string();
    /// 
    /// // Change the position of the game into the FEN-string
    /// chess.load(fen);
    /// ```
    /// 
    /// Loading a FEN-string resets the games state (chess.game_result, ...)
    /// 
    /// If a bad FEN-string is passed the game in the best case be cleared, and in the worst case crash
    /// 
    
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
            if fen_vec[2].chars().nth(0).unwrap_or('-') == 'K' && self.white_kings & PIECE[4] != 0 && self.white_rooks & PIECE[7] != 0 { 
                self.castling_rights.0 = true; 
            }
            if fen_vec[2].chars().nth(1).unwrap_or('-') == 'Q' && self.white_kings & PIECE[4] != 0 && self.white_rooks & PIECE[0] != 0 { 
                self.castling_rights.1 = true; 
            }
            if fen_vec[2].chars().nth(2).unwrap_or('-') == 'k' && self.black_kings & PIECE[8*7+4] != 0 && self.black_rooks & PIECE[8*7+7] != 0 { 
                self.castling_rights.2 = true; 
            }
            if fen_vec[2].chars().nth(3).unwrap_or('-') == 'q' && self.black_kings & PIECE[8*7+4] != 0 && self.black_rooks & PIECE[8*7+0] != 0 { 
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

        // Store position
        self.store_position();
    }

    
    /// Ends the game by white surrendering
    /// 
    /// # Examples
    /// 
    /// ```
    /// use davbjor_chess::{ChessBoard, GameResult};
    /// 
    /// // create a new game
    /// let mut chess = ChessBoard::new();
    /// 
    /// // Surrender the game for white
    /// chess.white_surrender();
    /// 
    /// if chess.game_result != GameResult::Black {
    ///     println!("White did not surrender, something wierd is going on!");
    /// }
    /// ```
    /// 
    pub fn white_surrender(&mut self){
        self.game_result = GameResult::Black;
    }

    /// Ends the game by black surrendering
    /// 
    /// # Examples
    /// 
    /// ```
    /// use davbjor_chess::{ChessBoard, GameResult};
    /// 
    /// // create a new game
    /// let mut chess = ChessBoard::new();
    /// 
    /// // Surrender the game for black
    /// chess.black_surrender();
    /// 
    /// if chess.game_result != GameResult::White {
    ///     println!("Black did not surrender, something wierd is going on!");
    /// }
    /// ```
    /// 
    pub fn black_surrender(&mut self){
        self.game_result = GameResult::White;
    }

    /// Ends the game by draw (if both players want it)
    /// 
    /// # Examples
    /// 
    /// ```
    /// use davbjor_chess::{ChessBoard, GameResult};
    /// 
    /// // create a new game
    /// let mut chess = ChessBoard::new();
    /// 
    /// // End game by draw
    /// chess.mutual_draw();
    /// 
    /// if chess.game_result != GameResult::Draw {
    ///     println!("The game is not a tie, something wierd is going on!");
    /// }
    /// ```
    /// 
    pub fn mutual_draw(&mut self){
        self.game_result = GameResult::Draw;
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

        for i in 0..64 {
            self.board[i] = PieceType::Empty;
            if self.all_pieces & PIECE[i] != 0 {
                self.board[i] = self.piece_at(i);
            }
        }
    }


    /// Reset the game to the starting position
    /// 
    /// # Examples
    /// 
    /// ```
    /// use davbjor_chess::{ChessBoard, GameResult};
    /// 
    /// // create a new game
    /// let mut chess = ChessBoard::new();
    /// 
    /// // Unnecessary right now, but could be useful sometimes in games
    /// chess.reset();
    /// 
    /// ```
    /// 
    /// 
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn castling() {
        let mut chess = ChessBoard::new();
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

        //chess.print_board(0);
    }

    #[test]
    fn white_in_stalemate() {
        let mut chess = ChessBoard::new();
        chess.load("k5rr/8/8/8/8/8/7p/7K w ---- - 0 1".to_string());
        /*
        chess.print_board(0);
        chess.print_board(chess.get_moves(7));
        */

        assert_eq!(chess.game_result, GameResult::Draw);
    }

    #[test]
    fn black_in_stalemate() {
        let mut chess = ChessBoard::new();
        chess.load("k7/8/8/8/8/8/5B2/1R5K b ---- - 0 1".to_string());
        /*
        chess.print_board(0);
        chess.print_board(chess.get_moves(7*8));
        */
    

        assert_eq!(chess.game_result, GameResult::Draw);
    }


    #[test]
    fn white_in_check() {
        let mut chess = ChessBoard::new();
        chess.load("2k5/8/4q3/8/6b1/1n6/1PPP4/3KR3".to_string());
        assert_eq!(chess.white_in_check(None, None), true);
        assert_eq!(chess.game_result, GameResult::Ongoing);

        chess.load("k6q/8/8/8/8/8/8/7K".to_string());
        assert_eq!(chess.white_in_check(None, None), true);
        assert_eq!(chess.game_result, GameResult::Ongoing);
    }

    /// Bug detected where game ended by checkmate when it was a sacrificial check
    /// 
    /// Solved by removal of the requirement for the own side to be the player in turn
    /// Moved that requirement to the get_moves_list function
    /// 
    #[test]
    fn checkmate_bug() {
        let mut chess = ChessBoard::new();

        assert!(chess.move_piece(SQUARE::E2, SQUARE::E4).is_ok());
        assert!(chess.move_piece(SQUARE::E7, SQUARE::E5).is_ok());
        assert!(chess.move_piece(SQUARE::F1, SQUARE::C4).is_ok());
        assert!(chess.move_piece(SQUARE::A7, SQUARE::A5).is_ok());
        assert!(chess.move_piece(SQUARE::C4, SQUARE::F7).is_ok());
        //chess.print_board(chess.get_moves(SQUARE::E8));
        assert_eq!(chess.game_result, GameResult::Ongoing);

    }

    #[test]
    fn en_passant() {
        let mut chess = ChessBoard::new();
        chess.load("rnbqkbnr/1p3p1p/8/P1PpP1P1/p1p1p1pP/8/1P1P1P2/RNBQKBNR w KQkq d6 0 1".to_string());
        
        // White can do en passant at d6 (due to fen string recording d6)
            //chess.print_board(chess.get_moves(SQUARE::E5));
        assert!(chess.move_piece(SQUARE::E5, SQUARE::D6).is_ok());
        assert_eq!(chess.piece_at(SQUARE::D5), PieceType::Empty);
        
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
        assert_eq!(chess.piece_at(SQUARE::B4), PieceType::Empty);

        //chess.print_board(0);
    }

    /// Count moves in positions with possible promotions, and then do a promotion
    /// 
    #[test]
    fn promotion() {
        let mut chess = ChessBoard::new();

        chess.load("3r3k/1p2P1pp/8/p7/8/5NK1/1qp3PP/8 w - - 0 39".to_string());
        assert_eq!(chess.count_moves(), 22);
        assert!(chess.handle_promotion(SQUARE::E7, SQUARE::D8, PieceType::WhiteQueen).is_ok());


        chess.load("8/pp3P1k/1npNp3/4P3/2PP1PR1/4K3/P1r5/7q w - - 1 38".to_string());
        assert_eq!(chess.count_moves(), 24);
        assert!(chess.handle_promotion(SQUARE::F7, SQUARE::F8, PieceType::WhiteKnight).is_ok());


        chess.load("8/pPr4k/6p1/8/1P5p/8/5PK1/8 w - - 0 37".to_string());
        assert_eq!(chess.count_moves(), 13);
        assert!(chess.handle_promotion(SQUARE::B7, SQUARE::B8, PieceType::WhiteQueen).is_ok());


        chess.load("r1bqr3/pp1n1Pkp/4p2b/3pP3/3N4/2NPBR2/PP4PP/R5K1 w - - 1 18".to_string());
        assert_eq!(chess.count_moves(), 51);
        assert!(chess.handle_promotion(SQUARE::F7, SQUARE::E8, PieceType::WhiteKnight).is_ok());


        chess.load("8/5QP1/2qp3k/4p3/8/6K1/4N3/1q6 w - - 0 60".to_string());
        //assert_eq!(chess.count_moves(), 35);
        println!("{}", chess.count_moves());
        assert!(chess.handle_promotion(SQUARE::G7, SQUARE::G8, PieceType::WhiteKnight).is_ok());
    }

    #[test]
    fn three_fold_repetition() {
        let mut chess = ChessBoard::new();

        // Move pawns
        assert!(chess.move_piece(SQUARE::E2, SQUARE::E4).is_ok());
        assert!(chess.move_piece(SQUARE::E7, SQUARE::E5).is_ok());

        // Begin to shuffle kings
        assert!(chess.move_piece(SQUARE::E1, SQUARE::E2).is_ok());
        assert!(chess.move_piece(SQUARE::E8, SQUARE::E7).is_ok());
        // Above is the first time in repeatable position due to castling-rights now being gone


        assert!(chess.move_piece(SQUARE::E2, SQUARE::E1).is_ok());
        assert!(chess.move_piece(SQUARE::E7, SQUARE::E8).is_ok());
        assert!(chess.move_piece(SQUARE::E1, SQUARE::E2).is_ok());
        assert!(chess.move_piece(SQUARE::E8, SQUARE::E7).is_ok());
        
        // Now twice repeated
        assert!(chess.move_piece(SQUARE::E2, SQUARE::E1).is_ok());
        assert!(chess.move_piece(SQUARE::E7, SQUARE::E8).is_ok());
        assert!(chess.move_piece(SQUARE::E1, SQUARE::E2).is_ok());
        
        // Last one
        assert!(chess.move_piece(SQUARE::E8, SQUARE::E7).is_ok());

        // Cant move anymore
        assert!(chess.move_piece(SQUARE::E8, SQUARE::E7).is_err());

        // Check if game is draw
        assert_eq!(chess.game_result,GameResult::Draw);
    }

    #[test]    
    fn fifty_move_rule() {
        let mut chess = ChessBoard::new();
        chess.load("k7/8/8/8/8/8/8/7K w ---- - 96 70".to_string());

        // Walk kings
        assert!(chess.move_piece(SQUARE::H1, SQUARE::H2).is_ok());
        assert!(chess.move_piece(SQUARE::A8, SQUARE::A7).is_ok());
        assert!(chess.move_piece(SQUARE::H2, SQUARE::H3).is_ok());
        assert!(chess.move_piece(SQUARE::A7, SQUARE::A6).is_ok());
        // Reached 100 moves

        // Cant move again
        assert!(chess.move_piece(SQUARE::H3, SQUARE::H4).is_err());

        // Check if game is draw
        assert_eq!(chess.game_result,GameResult::Draw);
    }

    /// Testing the amount of legal moves, compared to a chess engines result
    /// 
    /// Games gathered mainly from puzzles on lichess.org
    /// 
    /// Compared to the results of github.com/bhlangonijr/chesslib
    #[test]
    fn count_legal_moves () {
        let mut chess = ChessBoard::new();
        
        chess.load("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/5Q2/PPPBBPpP/RN2K2R w KQkq - 0 2".to_string());
        assert_eq!(chess.count_moves(), 47);
        
        chess.load("1r6/3k2p1/7p/Ppp2r1P/K1N1B1p1/2P2NP1/b7/4b3 w - - 0 56".to_string());
        assert_eq!(chess.count_moves(), 1);
        
        chess.load("2r3r3/4n3/p1kp3p/1p3pP1/1p1bPPKP/1PPP4/BR1R4/8 w - - 0 73".to_string());
        assert_eq!(chess.count_moves(), 5);
        
        chess.load("7k/8/R5Q1/1BpP4/3K4/8/8/8 w - c6 0 0".to_string());
        assert_eq!(chess.count_moves(), 8);

        chess.load("3n4/2k5/p5pr/2pBP2P/PpN1KP2/1P6/8/6b1 w - - 0 32".to_string());
        assert_eq!(chess.count_moves(), 19);

        chess.load("8/6kp/1r2rR1B/4P3/p1p5/1bN2P2/1Pn2K2/8 b - - 1 39".to_string());
        assert_eq!(chess.count_moves(), 2);

        chess.load("5kr1/1r2p1b1/p2p1R2/3q1Q1p/5P2/4R2P/P5PK/8 b - - 0 41".to_string());
        assert_eq!(chess.count_moves(), 4);

        chess.load("5Qk1/1p2r1bp/3pN1p1/3pq3/2P1p3/1P5P/P5P1/5RK1 b - - 1 27".to_string());
        assert_eq!(chess.count_moves(), 1);

        chess.load("2r2rk1/6pp/p4nbN/1p1pq1Q1/4p3/7P/PPP1NPP1/R4RK1 b - - 8 25".to_string());
        assert_eq!(chess.count_moves(), 2);

        chess.load("2r3k1/4q3/p3prpp/1p1Q4/2pP3P/8/PP3PP1/1B2RRK1 b - - 0 24".to_string());
        assert_eq!(chess.count_moves(), 39);

        chess.load("3r2k1/pb3pp1/1p6/8/8/P4P2/3R1QPP/3q2K1 w - - 0 34".to_string());
        assert_eq!(chess.count_moves(), 3);
        
    }

}
