
pub mod board {
    use crate::bitboard::bitboard::BitBoard;
    use crate::square::square::{BOARD_SQUARES, Square};
    use crate::rank::rank::{Rank, get_rank};
    use crate::file::file::{File, get_file};



/**********************************\
 ==================================
 
               Enums
 
 ==================================
\**********************************/    
    #[derive(Debug)]
    #[derive(Copy, Clone)]
    pub enum Color {White, Black}


    pub fn get_bit(bitboard: &BitBoard, square: &Square) -> u64 {
        let bitboard_u64 = bitboard.get_u64();
        bitboard_u64 & (1u64 << square.id)

    }


/**********************************\
 ==================================
 
              Structs
 
 ==================================
\**********************************/

    pub struct Board {
        // TODO make all values private and implement getters/setters
        chessboard: BitBoard,

        pub white_pawns: BitBoard,
        white_knights: BitBoard,
        white_bishops: BitBoard,
        white_rooks: BitBoard,
        white_queens: BitBoard,
        white_king: BitBoard,
        pub white_pieces: BitBoard,
    
        black_pawns: BitBoard,
        black_knights: BitBoard,
        black_bishops: BitBoard,
        black_rooks: BitBoard,
        black_queens: BitBoard,
        black_king: BitBoard,
        pub black_pieces: BitBoard,

        side_to_play: Color,


    }

    pub enum PieceType {Pawn, Knight, Bishop, Rook, Queen, King}

    impl Board {
        pub fn new() -> Board {
            Board {
                // // board squares are enumerated from A1 being the first square and H8 the last one
                // white_pawns: BitBoard::new(0b0000000011111111000000000000000000000000000000000000000000000000),
                // white_knights:  BitBoard::new(0b0100001000000000000000000000000000000000000000000000000000000000),
                // white_bishops:  BitBoard::new(0b0010010000000000000000000000000000000000000000000000000000000000),
                // white_rooks:  BitBoard::new(0b1000000100000000000000000000000000000000000000000000000000000000),
                // white_queens:  BitBoard::new(0b0001000000000000000000000000000000000000000000000000000000000000),
                // white_king:  BitBoard::new(0b0000100000000000000000000000000000000000000000000000000000000000),
                // // this is a bitboard to consider captures and friend pieces so the kings are not in there
                // // also while capturig/attacking/moving always check if not checking the king
                // white_pieces: BitBoard::new(0b0000000000000000000000000000000000000000000000000111111111111111),
            
                // black_pawns:  BitBoard::new(0b0000000000000000000000000000000000000000000000001111111100000000),
                // black_knights:  BitBoard::new(0b0000000000000000000000000000000000000000000000000000000001000010),
                // black_bishops:  BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000100100),
                // black_rooks:  BitBoard::new(0b0000000000000000000000000000000000000000000000000000000010000001),
                // black_queens:  BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000010000),
                // black_king:  BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000001000),
                // black_pieces: BitBoard::new(0b111111111111111100000000000000000000000000000000000000000000000),
                
                // chessboard:  BitBoard::new(0b111111111111111100000000000000000000000000000000111111111111111),
                // side_to_play: Color::White,

                white_pawns: BitBoard::new(0b0000000000000000000000000000000000000000000000001111111100000000),
                white_knights: BitBoard::new(0b0000000000000000000000000000000000000000000000000000000001000010),
                white_bishops: BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000100100),
                white_rooks: BitBoard::new(0b0000000000000000000000000000000000000000000000000000000010000001),
                white_queens: BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000001000),
                white_king: BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000010000),
                white_pieces: BitBoard::new(0b0000000000000000000000000000000000000000000000000111111111101111),
    
                black_pawns: BitBoard::new(0b0000000011111111000000000000000010000000000000000000000000000000),
                black_knights: BitBoard::new(0b0100001000000000000000000000000000000000000000000000000000000000),
                black_bishops: BitBoard::new(0b0010010000000000000000000000000000000000000000000000000000000000),
                black_rooks: BitBoard::new(0b1000000100000000000000000000000000000000000000000000000000000000),
                black_queens: BitBoard::new(0b0000100000000000000000000000000000000000000000000000000000000000),
                black_king: BitBoard::new(0b0001000000000000000000000000000000000000000000000000000000000000),
                black_pieces: BitBoard::new(0b111011111111111100000000000000000000000000000000000000000000000),
    
                chessboard: BitBoard::new(0b1111111111111111000000000000000000000000000000001111111111111111),
    
                side_to_play: Color::White,
            }
        }

        pub fn start_game(self) -> Board{
            // TODO: wymyslić pętlę gry i poprawić zwracanie!!
            self.initial_setup()
            

        }

        fn initial_setup(mut self) -> Board {           

            self.white_pawns = BitBoard::new(0b0000000000000000000000000000000000000000000000001111111100000000);
            self.white_knights = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000001000010);
            self.white_bishops = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000100100);
            self.white_rooks = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000010000001);
            self.white_queens = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000001000);
            self.white_king = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000010000);
            self.white_pieces = BitBoard::new(0b0000000000000000000000000000000000000000000000000111111111101111);

            self.black_pawns = BitBoard::new(0b0000000011111111000000000000000000000000000000000000000000000000);
            self.black_knights = BitBoard::new(0b0100001000000000000000000000000000000000000000000000000000000000);
            self.black_bishops = BitBoard::new(0b0010010000000000000000000000000000000000000000000000000000000000);
            self.black_rooks = BitBoard::new(0b1000000100000000000000000000000000000000000000000000000000000000);
            self.black_queens = BitBoard::new(0b0000100000000000000000000000000000000000000000000000000000000000);
            self.black_king = BitBoard::new(0b0001000000000000000000000000000000000000000000000000000000000000);
            self.black_pieces = BitBoard::new(0b111011111111111100000000000000000000000000000000000000000000000);

            self.chessboard = BitBoard::new(0b1111111111111111000000000000000000000000000000001111111111111111);

            self.side_to_play = Color::White;

            self
        }
        // getters/setters

        // immutable access
        pub fn chessboard(&self) -> &BitBoard {
            &self.chessboard
        }

        // mutable access
        pub fn chessboard_mut(self) -> BitBoard {
            self.chessboard
        }

/**********************************\
 ==================================
 
                Moves
 
 ==================================
\**********************************/


        // jak generować ruchy w bitboardach? Też wszystkie razem czy dla każdej figury osobno?
        // a jak osobno to w jaki sposób te ruchy przechowywać
        // FIXME - funkcja nie sprawdza w pełni czy dana figura rzeczywiście znajduje się na podanym polu
        pub fn generate_moves(&self, piece: PieceType, side: Color, square: Square) -> BitBoard{
            match piece {
                PieceType::Pawn =>{
                    // UNUSED
                    let pawn_targets = &( &self.white_pawns << 8i32 ) & &( !&self.chessboard );

                    // TODO: pseudo_legal and legal moves
                    // pseudo_legal - select empty squares - not yet implemented
                    // advance move
                    let pawns: BitBoard;
                    let mut moves: BitBoard;
                    match side {
                        Color::White => {
                            pawns = self.white_pawns;
                            moves = BitBoard::new((pawns.get_u64() & (1u64 << square.id as u64)) << 8u8);
                            // check if first move
                            if Rank::Second == get_rank(&square)  {
                                // HACK - dodać do bitboarda ^= (XOR_assign)
                                moves = &moves ^ &BitBoard::new(1u64 << square.id + 16);
                            }
                        }
                        // FIXME works only for second rank
                        Color::Black => {
                            pawns = self.black_pawns;
                            moves = BitBoard::new((pawns.get_u64() & (9223372036854775808u64 >> (63 - square.id as u64))) >> 8u8);
                            if Rank::Seventh == get_rank(&square)  {
                                // HACK - dodać do bitboarda ^= (XOR_assign)
                                moves = &moves ^ &BitBoard::new((9223372036854775808u64 >> 63 - square.id as u64) >> 16u8);
                            }
                        }
                    }
                    // check if the squares are empty
                    moves = &moves & &!&self.chessboard;
                    // moves = moves + pawn_attacks(side: Color, square: Square);
                    moves = &moves + &Board::pawn_attacks(&self, side, square);
                    moves
                    // FIXME

                }
                PieceType::Knight =>{
                    // TODO
                    let moves: BitBoard = BitBoard::new(0);
                    let knights: BitBoard;
                    if let Color::White = side {
                        knights = self.white_knights;
                    }
                    else {
                        knights = self.black_knights;
                    }

                    moves 
                }
                PieceType::Bishop =>{
                    // TODO
                    let moves: BitBoard = BitBoard::new(0);
                    let bishops: BitBoard;
                    if let Color::White = side {
                        bishops = self.white_bishops;
                    }
                    else {
                        bishops = self.black_bishops;
                    }

                    moves 
                }
                PieceType::Rook =>{
                    // TODO
                    let moves: BitBoard = BitBoard::new(0);
                    let rooks: BitBoard;
                    if let Color::White = side {
                        rooks = self.white_rooks;
                    }
                    else {
                        rooks = self.black_rooks;
                    }

                    moves 
                }
                PieceType::Queen =>{
                    // TODO
                    let moves: BitBoard = BitBoard::new(0);
                    let queens: BitBoard;
                    if let Color::White = side {
                        queens = self.white_queens;
                    }
                    else {
                        queens = self.black_queens;
                    }

                    moves 
                }
                PieceType::King =>{
                    // TODO
                    let moves: BitBoard = BitBoard::new(0);
                    let king: BitBoard;
                    if let Color::White = side {
                        king = self.white_king;
                    }
                    else {
                        king = self.black_king;
                    }

                    moves 
                }
            }

        }

        // FIXME !!!! too much overhead
        fn pawn_attacks(&self, side: Color, square: Square) -> BitBoard {
            let mut attacks: BitBoard = BitBoard::new(0);
            // identify possible attacks squares (keep in mind "square wrapping")
            match side {
                Color::White => {
                    let initial_file = get_file(&square);
                    let mut no_left_capture = false;
                    let mut no_right_capture = false;
                    if initial_file == File::A {
                        // no left capture
                        no_left_capture = true;

                    }
                    else if initial_file == File::H {
                        // no right capture
                        no_right_capture = true;

                    }
                    
                    let left_up = BitBoard::get_bitboard_from_square( &BOARD_SQUARES[(square.id + 7) as usize] );
                    let right_up = BitBoard::get_bitboard_from_square( &BOARD_SQUARES[(square.id + 9) as usize] );

                    if !no_left_capture && left_up.get_u64() & *self.black_pieces.get_u64() != 0 {
                        attacks = &attacks + &left_up;

                    }

                    if !no_right_capture && right_up.get_u64() & *self.black_pieces.get_u64() != 0 {
                        attacks = &attacks + &right_up;
                    }

                    return attacks;

                }

                Color::Black => {
                    let initial_file = get_file(&square);
                    let mut no_left_capture = false;
                    let mut no_right_capture = false;
                    if initial_file == File::A {
                        // no left capture
                        no_right_capture = true;

                    }
                    else if initial_file == File::H {
                        // no right capture
                        no_left_capture = true;

                    }

                    let left_down = BitBoard::get_bitboard_from_square( &BOARD_SQUARES[(square.id - 7) as usize] );
                    let right_down = BitBoard::get_bitboard_from_square( &BOARD_SQUARES[(square.id - 9) as usize] );
                    
                    // look for pieces to capture
                    if !no_left_capture && left_down.get_u64() & *self.white_pieces.get_u64() != 0 {
                        attacks = &attacks + &left_down;
                    }

                    if !no_right_capture && right_down.get_u64() & *self.white_pieces.get_u64() != 0 {

                        attacks = &attacks + &right_down;
                    }

                    return attacks;

                }
            }

            // TODO en passant
        }


    }    
}



/**********************************\
 ==================================
 
                Tests
 
 ==================================
\**********************************/ 


#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    fn possible_first_pawn_moves() {
        let mut board = Board::new();
        board = board.start_game();
        // let test = &board.white_pawns;
        assert_eq!(format!("{}", &board.generate_moves(PieceType::Pawn, Color::White, get_square("H2"))), "\n--------\n--------\n--------\n--------\n-------1\n-------1\n--------\n--------");
    }
    #[test]
    fn chessboard_initial_state() {
        let mut board = Board::new();
        board = board.start_game();
        assert_eq!(format!("{}", &board.chessboard()), "\n11111111\n11111111\n--------\n--------\n--------\n--------\n11111111\n11111111");
    }

    #[test]
    fn check_moves_generation() {
        let mut board = Board::new();
        board = board.start_game();
        {
            let moves = board.generate_moves(PieceType::Pawn, Color::White, BOARD_SQUARES[9]);
            assert_eq!(format!("{}", &moves), "\n--------\n--------\n--------\n--------\n-1------\n-1------\n--------\n--------");
        }

        {
            let moves = board.generate_moves(PieceType::Pawn, Color::Black, BOARD_SQUARES[49]);
            assert_eq!(format!("{}", &moves), "\n--------\n--------\n-1------\n-1------\n--------\n--------\n--------\n--------");
        }

        // {   
        //     // test pawns advance moves when it is not their first move

        //     // change board state
        //     let mut fake_board = Board::new();
        //     fake_board.white_pawns = BitBoard::new(0b0000000000000000000000000000000000000000000000001111111100000000);
        //     fake_board.chessboard = BitBoard::new(0b1111111111111111000000000000000000000000000000001111111111111111);

        //     let board = 
        // }
    }

}
