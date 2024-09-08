
pub mod tables {

type BitBoard = u64;
/*
Static Mask Rank Precomputation
Returns a bitmask of 1's on the chosen rank, and 0's elsewhere

Ex. Used to find all white pieces on rank 3 => white_pieces & MASK_RANK[2]

(Could be rewritten as RANK_1 << 8*RANK, proably cheaper operations)
*/
pub static MASK_RANK: [BitBoard; 8] = [
    ((1 as BitBoard) << (8 * 1)) - 1,
    ((1 as BitBoard) << (8 * 2)) - 1 ^ ((1 as BitBoard) << (8 * 1)) - 1,
    ((1 as BitBoard) << (8 * 3)) - 1 ^ ((1 as BitBoard) << (8 * 2)) - 1,
    ((1 as BitBoard) << (8 * 4)) - 1 ^ ((1 as BitBoard) << (8 * 3)) - 1,
    ((1 as BitBoard) << (8 * 5)) - 1 ^ ((1 as BitBoard) << (8 * 4)) - 1,
    ((1 as BitBoard) << (8 * 6)) - 1 ^ ((1 as BitBoard) << (8 * 5)) - 1,
    ((1 as BitBoard) << (8 * 7)) - 1 ^ ((1 as BitBoard) << (8 * 6)) - 1,
    !(0 as BitBoard) ^ (((1 as BitBoard) << (8 * 7)) - 1)
];


/*
Static Clear Rank Precomputation
Returns a bitmask of 0's on the chosen rank, and 1's elsewhere

Ex. Used to remove white pieces from row 6 => white_pieces & CLEAR_RANK[5]
*/
pub static CLEAR_RANK: [BitBoard; 8] = [
    !(((1 as BitBoard) << (8 * 1)) - 1),
    !(((1 as BitBoard) << (8 * 2)) - 1 ^ ((1 as BitBoard) << (8 * 1)) - 1),
    !(((1 as BitBoard) << (8 * 3)) - 1 ^ ((1 as BitBoard) << (8 * 2)) - 1),
    !(((1 as BitBoard) << (8 * 4)) - 1 ^ ((1 as BitBoard) << (8 * 3)) - 1),
    !(((1 as BitBoard) << (8 * 5)) - 1 ^ ((1 as BitBoard) << (8 * 4)) - 1),
    !(((1 as BitBoard) << (8 * 6)) - 1 ^ ((1 as BitBoard) << (8 * 5)) - 1),
    !(((1 as BitBoard) << (8 * 7)) - 1 ^ ((1 as BitBoard) << (8 * 6)) - 1),
    !(!(0 as BitBoard) ^ ((1 as BitBoard) << (8 * 1)) - 1)
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


}