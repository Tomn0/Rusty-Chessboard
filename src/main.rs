// use std::fmt;

mod structs;
pub use structs::board_structure;

mod board;
pub use board::board::Board;

mod square;
pub use square::square::Square;

fn main() {
    let mut board = Board::new();
    board = board.start_game();
    println!("{}", &board.white_pawns);

}
