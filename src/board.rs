
pub mod board {
    use crate::structs::board_structure::BitBoard;
    use crate::square::square::{BOARD_SQUARES, Square};


/**********************************\
 ==================================
 
                Enums
 
 ==================================
\**********************************/    

    pub enum Color {White, Black, Empty}


    pub fn get_bit(bitboard: BitBoard, square: &Square) -> u64 {
        let bitboard_u64 = bitboard.get_u64();
        bitboard_u64 & (1u64 << square.id)

    }


/**********************************\
 ==================================
 
            Structs
 
 ==================================
\**********************************/

    pub struct Board {
        chessboard: BitBoard,

        pub white_pawns: BitBoard,
        white_knights: BitBoard,
        white_bishops: BitBoard,
        white_rooks: BitBoard,
        white_queens: BitBoard,
        white_king: BitBoard,
    
        black_pawns: BitBoard,
        black_knights: BitBoard,
        black_bishops: BitBoard,
        black_rooks: BitBoard,
        black_queens: BitBoard,
        black_king: BitBoard,

        side_to_play: Color,


    }

    pub enum PieceType {Pawn, Knight, Bishop, Rook, Queen, King}

    impl Board {
        pub fn new() -> Board {
            Board {
                white_pawns: BitBoard::new(0b0000000011111111000000000000000000000000000000000000000000000000),
                white_knights:  BitBoard::new(0b0100001000000000000000000000000000000000000000000000000000000000),
                white_bishops:  BitBoard::new(0b0010010000000000000000000000000000000000000000000000000000000000),
                white_rooks:  BitBoard::new(0b1000000100000000000000000000000000000000000000000000000000000000),
                white_queens:  BitBoard::new(0b0001000000000000000000000000000000000000000000000000000000000000),
                white_king:  BitBoard::new(0b0000100000000000000000000000000000000000000000000000000000000000),
            
                black_pawns:  BitBoard::new(0b0000000000000000000000000000000000000000000000001111111100000000),
                black_knights:  BitBoard::new(0b0000000000000000000000000000000000000000000000000000000001000010),
                black_bishops:  BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000100100),
                black_rooks:  BitBoard::new(0b0000000000000000000000000000000000000000000000000000000010000001),
                black_queens:  BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000010000),
                black_king:  BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000001000),
                
                chessboard:  BitBoard::new(0b111111111111111100000000000000000000000000000000111111111111111),
                side_to_play: Color::Empty,
            }
        }

        pub fn start_game(self) -> Board{
            // TODO: wymyslić pętlę gry i poprawić zwracanie!!
            self.initial_setup()
            

        }

        fn initial_setup(mut self) -> Board{
            // self.white_pawns = BitBoard::new(0b0000000011111111000000000000000000000000000000000000000000000000);
            // self.white_knights = BitBoard::new(0b0100001000000000000000000000000000000000000000000000000000000000);
            // self.white_bishops = BitBoard::new(0b0010010000000000000000000000000000000000000000000000000000000000);
            // self.white_rooks = BitBoard::new(0b1000000100000000000000000000000000000000000000000000000000000000);
            // self.white_queens = BitBoard::new(0b0001000000000000000000000000000000000000000000000000000000000000);
            // self.white_king = BitBoard::new(0b0000100000000000000000000000000000000000000000000000000000000000);
           

            self.white_pawns = BitBoard::new(0b0000000000000000000000000000000000000000000000001111111100000000);
            self.white_knights = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000001000010);
            self.white_bishops = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000100100);
            self.white_rooks = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000010000001);
            self.white_queens = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000010000);
            self.white_king = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000001000);


            // self.black_pawns = BitBoard::new(0b0000000000000000000000000000000000000000000000001111111100000000);
            // self.black_knights = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000001000010);
            // self.black_bishops = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000100100);
            // self.black_rooks = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000010000001);
            // self.black_queens = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000010000);
            // self.black_king = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000001000);
    
            self.black_pawns = BitBoard::new(0b0000000011111111000000000000000000000000000000000000000000000000);
            self.black_knights = BitBoard::new(0b0100001000000000000000000000000000000000000000000000000000000000);
            self.black_bishops = BitBoard::new(0b0010010000000000000000000000000000000000000000000000000000000000);
            self.black_rooks = BitBoard::new(0b1000000100000000000000000000000000000000000000000000000000000000);
            self.black_queens = BitBoard::new(0b0001000000000000000000000000000000000000000000000000000000000000);
            self.black_king = BitBoard::new(0b0000100000000000000000000000000000000000000000000000000000000000);

            self.chessboard = BitBoard::new(0b111111111111111100000000000000000000000000000000111111111111111);

            self.side_to_play = Color::White;

            self
        }

        // jak generować ruchy w bitboardach? Też wszystkie razem czy dla każdej figury osobno?
        // a jak osobno to w jaki sposób te ruchy przechowywać
        fn generate_moves(&self, piece: PieceType, side: Color, square: Square, board: Board) -> BitBoard{
            match piece {
                PieceType::Pawn =>{
                    let pawn_targets = &( &self.white_pawns << 8i32 ) & &( !&self.chessboard );
                    // TODO: pseudo_legal and legal moves
                    // pseudo_legal - select empty squares
                    // advance move - TODO -> first move by two squares
                    let pawns: BitBoard;
                    if let Color::White = side {
                        pawns = board.white_pawns;
                    }
                    else {
                        pawns = board.black_pawns;
                    }

                    let mut moves = BitBoard::new((pawns.get_u64() & square.id as u64) << 8u8);
                    moves
                    // FIXME

                }
                PieceType::Knight =>{
                    // TODO
                    let moves: BitBoard = BitBoard::new(0);
                    let knights: BitBoard;
                    if let Color::White = side {
                        knights = board.white_knights;
                    }
                    else {
                        knights = board.black_knights;
                    }

                    moves 
                }
                PieceType::Bishop =>{
                    // TODO
                    let moves: BitBoard = BitBoard::new(0);
                    let bishops: BitBoard;
                    if let Color::White = side {
                        bishops = board.white_bishops;
                    }
                    else {
                        bishops = board.black_bishops;
                    }

                    moves 
                }
                PieceType::Rook =>{
                    // TODO
                    let moves: BitBoard = BitBoard::new(0);
                    let rooks: BitBoard;
                    if let Color::White = side {
                        rooks = board.white_rooks;
                    }
                    else {
                        rooks = board.black_rooks;
                    }

                    moves 
                }
                PieceType::Queen =>{
                    // TODO
                    let moves: BitBoard = BitBoard::new(0);
                    let queens: BitBoard;
                    if let Color::White = side {
                        queens = board.white_queens;
                    }
                    else {
                        queens = board.black_queens;
                    }

                    moves 
                }
                PieceType::King =>{
                    // TODO
                    let moves: BitBoard = BitBoard::new(0);
                    let king: BitBoard;
                    if let Color::White = side {
                        king = board.white_king;
                    }
                    else {
                        king = board.black_king;
                    }

                    moves 
                }
            }
        }
        fn pawn_attacks(side: Color) {

            // result attack bitboard
            let attacks = 0u64;

            // set piece on board

            // white pawns

            // black pawns
        }

    }    
}
