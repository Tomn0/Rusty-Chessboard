// use std::fmt;

mod bitboard;
pub use bitboard::bitboard::BitBoard;

mod board;
pub use board::board::{Board, get_bit, PieceType, Color};

mod square;
pub use square::square::{Square, BOARD_SQUARES, get_square};

mod rank;
mod file;


fn main() {
    let mut board = Board::new();
    board = board.start_game();
    let test = &board.white_pawns;
    // println!("{}", &board.white_pawns);
    
    // println!("{}", &BitBoard::new(get_bit(test, &BOARD_SQUARES[9])));
    
    let result = board.generate_moves(PieceType::Pawn, Color::White, get_square("H2"));

    println!("{}", &result);

}



// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn possible_first_pawn_moves() {
//         let mut board = Board::new();
//         board = board.start_game();
//         // let test = &board.white_pawns;
//         assert_eq!(format!("{}", &board.generate_moves(PieceType::Pawn, Color::White, get_square("H2"))), "\n--------\n--------\n--------\n--------\n-------1\n-------1\n--------\n--------");
//     }
//     #[test]
//     fn chessboard_initial_state() {
//         let mut board = Board::new();
//         board = board.start_game();
//         assert_eq!(format!("{}", &board.chessboard), "\n11111111\n11111111\n--------\n--------\n--------\n--------\n11111111\n11111111");
//     }

// }

// useful binary <-> decimal convertion:
// 1000000000000000000000000000000000000000000000000000000000000000 <-> 9223372036854775808

// initial piece states
// 0000000000000000000000000000000000000000000000001111111100000000 <-> 65280 white_pawns
// 0000000000000000000000000000000000000000000000000000000001000010 <-> 66 white_knights
// 0000000000000000000000000000000000000000000000000000000000100100 <-> 36 white_bishops
// 0000000000000000000000000000000000000000000000000000000010000001 <-> 129 white_rooks
// 0000000000000000000000000000000000000000000000000000000000001000 <-> 8 white_queens
// 0000000000000000000000000000000000000000000000000000000000010000 <-> 16 white_king


// 0000000011111111000000000000000000000000000000000000000000000000 <-> 71776119061217280 black_pawns
// 0100001000000000000000000000000000000000000000000000000000000000 <-> 4755801206503243776 black_knights
// 0010010000000000000000000000000000000000000000000000000000000000 <-> 2594073385365405696 black_bishops
// 1000000100000000000000000000000000000000000000000000000000000000 <-> 9295429630892703744 black_rooks
// 0000100000000000000000000000000000000000000000000000000000000000 <-> 576460752303423488 black_queens
// 0001000000000000000000000000000000000000000000000000000000000000 <-> 1152921504606846976 black_king

// 1111111111111111000000000000000000000000000000001111111111111111 <-> 18446462598732906495 initial state of the chessboard
