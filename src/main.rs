// use std::fmt;

mod structs;
pub use structs::board_structure::BitBoard;

mod board;
pub use board::board::{Board, get_bit, PieceType, Color};

mod square;
pub use square::square::{Square, BOARD_SQUARES, get_square};


fn main() {
    let mut board = Board::new();
    board = board.start_game();
    let test = &board.white_pawns;
    // println!("{}", &board.white_pawns);
    
    // println!("{}", &BitBoard::new(get_bit(test, &BOARD_SQUARES[9])));
    
    let result = board.generate_moves(PieceType::Pawn, Color::Black, get_square("C7"));

    println!("{}", &result);

}



// useful binary <-> decimal convertion:
// 1000000000000000000000000000000000000000000000000000000000000000 <-> 9223372036854775808