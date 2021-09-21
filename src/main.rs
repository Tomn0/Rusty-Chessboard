// use std::fmt;

mod structs;
pub use structs::board_structure::BitBoard;

mod board;
pub use board::board::{Board, get_bit};

mod square;
pub use square::square::{Square, BOARD_SQUARES};


fn main() {
    let mut board = Board::new();
    board = board.start_game();
    let test = board.white_pawns;
    // println!("{}", &board.white_pawns);
    
    println!("{}", &BitBoard::new(get_bit(test, &BOARD_SQUARES[9])));
}
