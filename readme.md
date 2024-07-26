## Crabablanca: command-line chess in Rust

So named after legendary chess player José Raúl Capablanca and Ferris the Rust mascot, who is a crab.

### How to play

After compilation, launching the executable in a terminal window will start the game. Default behaviour is that the player must input moves for both sides using standard [algebraic chess notation](https://en.wikipedia.org/wiki/Algebraic_notation_(chess)) (with some exceptions, noted below). A number of commands allow for different behaviour:
* `white` hands control of the black pieces to the engine, leaving the player as white
* `black` hands control of the white pieces to the engine, leaving the player as black
* `play` hands control of both sides to the engine, which will then play against itself
* `showme` and `!showme` toggle display of every possible move after each move is made
* `preview` briefly shows the top engine move in the current position
* `next` plays the top engine move
* `quit` and `exit` terminate the program

Exceptions to standard chess notation are as follows:
* Castling is performed by inputting a king move to the square it will end up on having castled (ie. Kc1/Kg1 for white, Kc8/Kg8 for black), instead of standard O-O/O-O-O.
* Pawns automatically promote to queens, no additional notation for promotion is currently supported.
* En passant is not implemented, so notation to attempt it will not be accepted.

### The Crabablanca engine
Crabablanca is a from-scratch engine.
#### Board representation
Crabablanca is built on "bitboards", whereby each set of pieces of a certain type (eg. white rooks, black pawns, etc.) is represented by a 64-bit unsigned integer. Each bit represents a square of the chessboard, and is 1 if a piece is present in that square, and 0 otherwise. This allows for highly compact representation and efficient manipulation of the state of the board.
#### Evaluation
Crabablanca considers material balance, central pawns, piece mobility, king safety, doubled pawns, and possible checks when assessing static evaluation. The evaluation function has not been thoroughly tuned, and is a long way from complete.
#### Search
Crabablanca uses an alpha/beta depth-first search to a fixed depth. Currently that depth is set to 4 ply (ie. 2 moves from each side), which is not a lot - this is limited by the (currently!) rudimentary nature of the engine. Plans to improve search performance include implementing a transposition table and experimenting with multi-threaded search.