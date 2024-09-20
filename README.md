# davbjor-chess
Chess library in Rust

## How to use
1. Add to your Cargo.toml
```rust
[dependencies]
davbjor-chess = { git = "https://github.com/INDA24PlusPlus/davbjor-chess.git" }

```

2. Use the crate in your program
Import the relevant crate.
```rust
extern crate davbjor_chess;
use davbjor_chess::{ChessBoard, PieceType, GameResult};
```

3. Create an instance of the game
```rust
let mut chess = ChessBoard::new();
```

4. Creating a chess game
There are many methods related to the ChessBoard struct, you can see how to use them by running the rustdoc of the library in the terminal by running:
```bash
cargo doc --open
```

## How it works
The library is based on bitboards (unsigned 64 bit integers), representing the board by 12 different bitboards - one for each piece-type (different for black and white).

The board is in many methods referenced by a usize (0-63 inclusive) where square A1 = 0, square H1 = 7, square A8 = 7x8 = 56, square H8 = 7x8+7 = 63.

You can load a fen-string position into the game by using the load method of the ChessBoard struct, or reset it to the standard setup of a game by calling the reset method. When using the ::new() constructor the game is setup as a default chess game.

Getting the state of the board - you can get the state of the board from the board field in the ChessBoard struct, there it is represented by a 64 sized vector containing the enum PieceType.

Getting the possible moves of a piece - you can get the legal moves of a piece (when it is that colors turn) by using the get_moves_list method, which will return a vector containg the possible squares (0-63 inclusive) that the piece can move to.

Making a move - you can make a move by using the move_piece method, which takes in a from square and a to square. The method returns a Result, which should be matched according to the rustdoc of the method - don't forget to handle promotions by calling the handle_promotion method.

The result of the game - The result of the game is stored in the game_result field of the ChessBoard struct, and is of the type GameResult enum. Either the game is still ongoing, or a player has won (black / white) or it is a draw.

## Good Luck
I hope everything goes well!