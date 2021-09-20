// use std::fmt;

mod structs;
pub use structs::board_structure;

mod board;
pub use board::board::Board;

fn main() {
    let board = Board::new();
}
