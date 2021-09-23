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



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pawn_moves() {
        let mut board = Board::new();
        board = board.start_game();
        // let test = &board.white_pawns;
        assert_eq!(format!("{}", &board.generate_moves(PieceType::Pawn, Color::White, get_square("H2"))), "\n--------\n--------\n--------\n--------\n-------1\n-------1\n--------\n--------");
    }
    #[test]
    fn chessboard_look() {
        let mut board = Board::new();
        board = board.start_game();
        assert_eq!(format!("{}", &board.chessboard), "\n11111111\n11111111\n--------\n--------\n--------\n--------\n11111111\n11111111");
    }

}

// useful binary <-> decimal convertion:
// 1000000000000000000000000000000000000000000000000000000000000000 <-> 9223372036854775808