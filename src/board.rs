
pub mod board {

    pub use crate::structs::board_structure::BitBoard;

    enum Color {WHITE, BLACK, EMPTY}

    pub struct Board {
        chessboard: BitBoard,

        whitePawns: BitBoard,
        whiteKnights: BitBoard,
        whiteBishops: BitBoard,
        whiteRooks: BitBoard,
        whiteQueens: BitBoard,
        whiteKing: BitBoard,
    
        blackPawns: BitBoard,
        blackKnights: BitBoard,
        blackBishops: BitBoard,
        blackRooks: BitBoard,
        blackQueens: BitBoard,
        blackKing: BitBoard,

        side_to_play: Color,


    }

    pub enum PiecesTypes {EMPTY, Pawn, Knight, Bishop, Rook, Queen, King}

    impl Board {
        pub fn new() -> Board {
            Board {
                whitePawns: BitBoard::new(0b0000000011111111000000000000000000000000000000000000000000000000),
                whiteKnights:  BitBoard::new(0b0100001000000000000000000000000000000000000000000000000000000000),
                whiteBishops:  BitBoard::new(0b0010010000000000000000000000000000000000000000000000000000000000),
                whiteRooks:  BitBoard::new(0b1000000100000000000000000000000000000000000000000000000000000000),
                whiteQueens:  BitBoard::new(0b0001000000000000000000000000000000000000000000000000000000000000),
                whiteKing:  BitBoard::new(0b0000100000000000000000000000000000000000000000000000000000000000),
            
                blackPawns:  BitBoard::new(0b0000000000000000000000000000000000000000000000001111111100000000),
                blackKnights:  BitBoard::new(0b0000000000000000000000000000000000000000000000000000000001000010),
                blackBishops:  BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000100100),
                blackRooks:  BitBoard::new(0b0000000000000000000000000000000000000000000000000000000010000001),
                blackQueens:  BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000010000),
                blackKing:  BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000001000),
                
                chessboard:  BitBoard::new(0b111111111111111100000000000000000000000000000000111111111111111),
                side_to_play: Color::EMPTY,
            }
        }

        fn initial_setup(mut self) {
            self.whitePawns = BitBoard::new(0b0000000011111111000000000000000000000000000000000000000000000000);
            self.whiteKnights = BitBoard::new(0b0100001000000000000000000000000000000000000000000000000000000000);
            self.whiteBishops = BitBoard::new(0b0010010000000000000000000000000000000000000000000000000000000000);
            self.whiteRooks = BitBoard::new(0b1000000100000000000000000000000000000000000000000000000000000000);
            self.whiteQueens = BitBoard::new(0b0001000000000000000000000000000000000000000000000000000000000000);
            self.whiteKing = BitBoard::new(0b0000100000000000000000000000000000000000000000000000000000000000);
        
            self.blackPawns = BitBoard::new(0b0000000000000000000000000000000000000000000000001111111100000000);
            self.blackKnights = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000001000010);
            self.blackBishops = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000100100);
            self.blackRooks = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000010000001);
            self.blackQueens = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000010000);
            self.blackKing = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000001000);
            
            self.chessboard = BitBoard::new(0b111111111111111100000000000000000000000000000000111111111111111);

        }

        fn generate_moves(&self, piece: PiecesTypes) {
            match piece {
                PiecesTypes::Pawn =>{
                    let pawn_targets = &( &self.whitePawns << 8i32 ) & &( !&self.chessboard );
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