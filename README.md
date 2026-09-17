# CHESSY

Welcome to Tina's Rust chess library: **chessy**! 

Hopefully it has no bugs and handles all the important logic (just tell me if something is wrong). Have fun using it :D

## Installation
Add the following code under dependencies in Cargo.toml
```toml
[dependencies]
chessy = {git = "https://github.com/INDA26PlusPlus/tingxuan-chess"}
```

## Quickstart
```rs
use chessy::{Chess, PieceType};

fn main() {
    // creates a new game with the standard starting board
    let mut game = Chess::new();

    // get all legal moves the piece at square 12 is able to make
    let moves = game.legal_moves(12);
    println!("{:?}", moves);

    // make a move: from, to, promotion piece (if relevant, else None)
    match game.move_piece(52, 36, None) {
        // if Ok then the move is valid
        Ok(()) => game.print(),
        // else illegal move, print error
        Err(typ) => println!("Illegal move: {:?}", typ),
    }
    // after making a move the turn switches to the other player
}

```

## Board representation
The board is an 1D array with 64 elements, `[Option<Piece>; 64]`. The black pieces start at index 0-15 and white pieces 48-63.

```
                           +-------------------------------+
 0  1  2  3  4  5  6  7    | r | n | b | q | k | b | n | r |
 8  9 10 11 12 13 14 15    | p | p | p | p | p | p | p | p |
16 17 18 19 20 21 22 23    | . | . | . | . | . | . | . | . |
24 25 26 27 28 29 30 31    | . | . | . | . | . | . | . | . |
32 33 34 35 36 37 38 39    | . | . | . | . | . | . | . | . |
40 41 42 43 44 45 46 47    | . | . | . | . | . | . | . | . |
48 49 50 51 52 53 54 55    | P | P | P | P | P | P | P | P |
56 57 58 59 60 61 62 63    | R | N | B | Q | K | B | N | R |
                           +-------------------------------+
```

## Gameflow
### -`Chess::new() -> Chess`-
create a new instance of the `Chess` struct

### -`instance.next_turn()`-
force turn to opponent

### -`instance.move_piece(from: usize, to: usize, promotion: Option<PieceType>) -> Result<(), Error>`-
move a piece from square `from` to `to` and then switches the turn to other player \
if this is a promotion: include the piecetype you want to promote to as `promotion` \
will return either nothing if successful move, else return an error

### -`instance.legal_moves(position: usize) -> Vec<usize>`-
generate all possible moves for piece as square `position` \
returns a vector containing square index of all possible moves

### -`instance.game_status() -> GameStatus`-
check the game status right now

### -`instance.print()`-
print an ascii representation of the board, the one to the right above \
good for debugging!

## Types
public enums
|||
|----|----|
| `PieceType` | `King`, `Queen`, `Rook`, `Bishop`, `Knight`, `Pawn` |
| `Color` | `White`, `Black` |
| `Error` | `IllegalMove`, `OutOfBound`, `IllegalPromotion` |
| `GameStatus` | `Ongoing`, `Check`, `Checkmate`, `Stalemate` |

\
`IllegalMove` means the move is not in legal_moves, you cannot move the piece like that \
`OutOfBound` is if the index is out of board, outsite 0-63 \
`IllegalPromotion` if user try to promote to an illegal piece, or this move is just not legal

&nbsp;
```rs
// the Chess struct:
pub struct Chess {
    pub board: [Option<Piece>;64],
    pub turn: Color,
    pub opposite: Color,
    pub r_white_castle: bool,
    pub l_white_castle: bool,
    pub r_black_castle: bool,
    pub l_black_castle: bool,
    pub en_passant: Option<usize>,
}
```
&nbsp;
```rs
// the Piece struct:
pub struct Piece { 
    pub piece_type: PieceType,
    pub color: Color,
}
```