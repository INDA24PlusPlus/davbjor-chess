
pub mod tables {

    type BitBoard = u64;
    
    /*
    Static Mask Rank 1 Precomputation to create arrays of precomutation
    */
    static MASK_RANK_1: BitBoard = ((1 as BitBoard) << (8 * 1)) - 1;
    
    /*
    Static Mask Rank Precomputation
    Returns a bitmask of 1's on the chosen rank, and 0's elsewhere
    
    Ex. Used to find all white pieces on rank 3 => white_pieces & MASK_RANK[2]
    
    (Could be rewritten as RANK_1 << 8*RANK, proably cheaper operations)
    */
    pub static MASK_RANK: [BitBoard; 8] = [
        MASK_RANK_1,
        MASK_RANK_1 << 8 * 1,
        MASK_RANK_1 << 8 * 2,
        MASK_RANK_1 << 8 * 3,
        MASK_RANK_1 << 8 * 4,
        MASK_RANK_1 << 8 * 5,
        MASK_RANK_1 << 8 * 6,
        MASK_RANK_1 << 8 * 7,
    ];
    
    
    /*
    Static Clear Rank Precomputation
    Returns a bitmask of 0's on the chosen rank, and 1's elsewhere
    
    Ex. Used to remove white pieces from row 6 => white_pieces & CLEAR_RANK[5]
    */
    #[allow(dead_code)]
    pub static CLEAR_RANK: [BitBoard; 8] = [
        !MASK_RANK_1,
        !(MASK_RANK_1 << 8 * 1),
        !(MASK_RANK_1 << 8 * 2),
        !(MASK_RANK_1 << 8 * 3),
        !(MASK_RANK_1 << 8 * 4),
        !(MASK_RANK_1 << 8 * 5),
        !(MASK_RANK_1 << 8 * 6),
        !(MASK_RANK_1 << 8 * 7)
    ];
    
    /*
    Static Mask File 1 Precomputation to create arrays of precomutation
    */
    static MASK_FILE_1: BitBoard = (1 as BitBoard) << (8 * 7) | (1 as BitBoard) << (8 * 6) | (1 as BitBoard) << (8 * 5) | (1 as BitBoard) << (8 * 4) | (1 as BitBoard) << (8 * 3) | (1 as BitBoard) << (8 * 2) | (1 as BitBoard) << (8 * 1) | (1 as BitBoard) << (8 * 0);
    
    /*
    Static Mask Rank Precomputation
    Returns a bitmask of 1's on the chosen file, and 0's elsewhere
    
    Ex. Used to find all white pieces on file 3 => white_pieces & MASK_FILE[2]
    */
    #[allow(dead_code)]
    pub static MASK_FILE: [BitBoard; 8] = [
        MASK_FILE_1,
        MASK_FILE_1 << 1,
        MASK_FILE_1 << 2,
        MASK_FILE_1 << 3,
        MASK_FILE_1 << 4,
        MASK_FILE_1 << 5,
        MASK_FILE_1 << 6,
        MASK_FILE_1 << 7
    ];
    
    
    /*
    Static Clear File Precomputation
    Returns a bitmask of 0's on the chosen file, and 1's elsewhere
    
    Ex. Used to remove white pieces from row 6 => white_pieces & CLEAR_FILE[5]
    */
    pub static CLEAR_FILE: [BitBoard; 8] = [
        !MASK_FILE_1,
        !(MASK_FILE_1 << 1),
        !(MASK_FILE_1 << 2),
        !(MASK_FILE_1 << 3),
        !(MASK_FILE_1 << 4),
        !(MASK_FILE_1 << 5),
        !(MASK_FILE_1 << 6),
        !(MASK_FILE_1 << 7)
    ];
    
    /*
    Precomputed values of BitBoard for every square on the board
    */
    pub static PIECE: [BitBoard; 64] = [
        (1 as BitBoard),
        (1 as BitBoard) << 1,
        (1 as BitBoard) << 2,
        (1 as BitBoard) << 3,
        (1 as BitBoard) << 4,
        (1 as BitBoard) << 5,
        (1 as BitBoard) << 6,
        (1 as BitBoard) << 7,
        (1 as BitBoard) << 8,
        (1 as BitBoard) << 9,
        (1 as BitBoard) << 10,
        (1 as BitBoard) << 11,
        (1 as BitBoard) << 12,
        (1 as BitBoard) << 13,
        (1 as BitBoard) << 14,
        (1 as BitBoard) << 15,
        (1 as BitBoard) << 16,
        (1 as BitBoard) << 17,
        (1 as BitBoard) << 18,
        (1 as BitBoard) << 19,
        (1 as BitBoard) << 20,
        (1 as BitBoard) << 21,
        (1 as BitBoard) << 22,
        (1 as BitBoard) << 23,
        (1 as BitBoard) << 24,
        (1 as BitBoard) << 25,
        (1 as BitBoard) << 26,
        (1 as BitBoard) << 27,
        (1 as BitBoard) << 28,
        (1 as BitBoard) << 29,
        (1 as BitBoard) << 30,
        (1 as BitBoard) << 31,
        (1 as BitBoard) << 32,
        (1 as BitBoard) << 33,
        (1 as BitBoard) << 34,
        (1 as BitBoard) << 35,
        (1 as BitBoard) << 36,
        (1 as BitBoard) << 37,
        (1 as BitBoard) << 38,
        (1 as BitBoard) << 39,
        (1 as BitBoard) << 40,
        (1 as BitBoard) << 41,
        (1 as BitBoard) << 42,
        (1 as BitBoard) << 43,
        (1 as BitBoard) << 44,
        (1 as BitBoard) << 45,
        (1 as BitBoard) << 46,
        (1 as BitBoard) << 47,
        (1 as BitBoard) << 48,
        (1 as BitBoard) << 49,
        (1 as BitBoard) << 50,
        (1 as BitBoard) << 51,
        (1 as BitBoard) << 52,
        (1 as BitBoard) << 53,
        (1 as BitBoard) << 54,
        (1 as BitBoard) << 55,
        (1 as BitBoard) << 56,
        (1 as BitBoard) << 57,
        (1 as BitBoard) << 58,
        (1 as BitBoard) << 59,
        (1 as BitBoard) << 60,
        (1 as BitBoard) << 61,
        (1 as BitBoard) << 62,
        (1 as BitBoard) << 63
    ];
    
    pub struct SQUARE();

    #[allow(dead_code)]
    impl SQUARE {
        pub const A1: usize = 0;
        pub const B1: usize = 1;
        pub const C1: usize = 2;
        pub const D1: usize = 3;
        pub const E1: usize = 4;
        pub const F1: usize = 5;
        pub const G1: usize = 6;
        pub const H1: usize = 7;
        pub const A2: usize = 8;
        pub const B2: usize = 9;
        pub const C2: usize = 10;
        pub const D2: usize = 11;
        pub const E2: usize = 12;
        pub const F2: usize = 13;
        pub const G2: usize = 14;
        pub const H2: usize = 15;
        pub const A3: usize = 16;
        pub const B3: usize = 17;
        pub const C3: usize = 18;
        pub const D3: usize = 19;
        pub const E3: usize = 20;
        pub const F3: usize = 21;
        pub const G3: usize = 22;
        pub const H3: usize = 23;
        pub const A4: usize = 24;
        pub const B4: usize = 25;
        pub const C4: usize = 26;
        pub const D4: usize = 27;
        pub const E4: usize = 28;
        pub const F4: usize = 29;
        pub const G4: usize = 30;
        pub const H4: usize = 31;
        pub const A5: usize = 32;
        pub const B5: usize = 33;
        pub const C5: usize = 34;
        pub const D5: usize = 35;
        pub const E5: usize = 36;
        pub const F5: usize = 37;
        pub const G5: usize = 38;
        pub const H5: usize = 39;
        pub const A6: usize = 40;
        pub const B6: usize = 41;
        pub const C6: usize = 42;
        pub const D6: usize = 43;
        pub const E6: usize = 44;
        pub const F6: usize = 45;
        pub const G6: usize = 46;
        pub const H6: usize = 47;
        pub const A7: usize = 48;
        pub const B7: usize = 49;
        pub const C7: usize = 50;
        pub const D7: usize = 51;
        pub const E7: usize = 52;
        pub const F7: usize = 53;
        pub const G7: usize = 54;
        pub const H7: usize = 55;
        pub const A8: usize = 56;
        pub const B8: usize = 57;
        pub const C8: usize = 58;
        pub const D8: usize = 59;
        pub const E8: usize = 60;
        pub const F8: usize = 61;
        pub const G8: usize = 62;
        pub const H8: usize = 63;
    
    }

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
    

}
