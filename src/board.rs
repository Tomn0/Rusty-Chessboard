
pub mod board {

    pub use crate::structs::board_structure::BitBoard;

    enum Color {WHITE, BLACK, EMPTY}

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

    pub enum PiecesTypes {EMPTY, Pawn, Knight, Bishop, Rook, Queen, King}

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
                side_to_play: Color::EMPTY,
            }
        }

        pub fn start_game(self) -> Board{
            // TODO: wymyslić pętlę gry i poprawić zwracanie!!
            self.initial_setup()

        }

        fn initial_setup(mut self) -> Board{
            self.white_pawns = BitBoard::new(0b0000000011111111000000000000000000000000000000000000000000000000);
            self.white_knights = BitBoard::new(0b0100001000000000000000000000000000000000000000000000000000000000);
            self.white_bishops = BitBoard::new(0b0010010000000000000000000000000000000000000000000000000000000000);
            self.white_rooks = BitBoard::new(0b1000000100000000000000000000000000000000000000000000000000000000);
            self.white_queens = BitBoard::new(0b0001000000000000000000000000000000000000000000000000000000000000);
            self.white_king = BitBoard::new(0b0000100000000000000000000000000000000000000000000000000000000000);
        
            self.black_pawns = BitBoard::new(0b0000000000000000000000000000000000000000000000001111111100000000);
            self.black_knights = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000001000010);
            self.black_bishops = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000100100);
            self.black_rooks = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000010000001);
            self.black_queens = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000010000);
            self.black_king = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000001000);
            
            self.chessboard = BitBoard::new(0b111111111111111100000000000000000000000000000000111111111111111);

            self.side_to_play = Color::WHITE;

            self
        }

        fn generate_moves(&self, piece: PiecesTypes) {
            match piece {
                PiecesTypes::Pawn =>{
                    let pawn_targets = &( &self.white_pawns << 8i32 ) & &( !&self.chessboard );
                    // TODO: pseudo_legal and legal moves
                }
                PiecesTypes::Knight =>{}
                PiecesTypes::Bishop =>{}
                PiecesTypes::Rook =>{}
                PiecesTypes::Queen =>{}
                PiecesTypes::King =>{}
                PiecesTypes::EMPTY =>{}
            }
        }
    }

    
}