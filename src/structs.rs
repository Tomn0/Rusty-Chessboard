use std::fmt;

pub struct BitBoard(u64);

impl BitBoard {
    pub fn new (b: u64) -> BitBoard {
        return BitBoard(b);
    }

    pub fn display(&self) {
        let BitBoard(v) = &self;    // destructuring let
        println!("{:#025b}", v)
    }
}



impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut repr = String::new();
        for x in 0..64 {
            if self.0 & ( 1u64 << x ) == 1u64 << x {
                repr.push('1');
            }
            else {
                repr.push('-');
            }
            
            
            if x % 8 == 7 {
                repr.push('\n');
            }
        }
        write!(f, "{}", repr)
    }
}

pub enum PiecesTypes {EMPTY, Pawn, Knight, Bishop, Rook, Queen, King};

struct Board {
    whitePawns: BitBoard;
    whiteKnights: BitBoard;
    whiteBishops: BitBoard;
    whiteRooks: BitBoard;
    whiteQueens: BitBoard;
    whiteKing: BitBoard;
 
    blackPawns: BitBoard;
    blackKnights: BitBoard;
    blackBishops: BitBoard;
    blackRooks: BitBoard;
    blackQueens: BitBoard;
    blackKing: BitBoard;

    board: BitBoard;
}


impl Board {
    fn initial_setup(&self) {
        &self.whitePawns = BitBoard::new(0b0000000011111111000000000000000000000000000000000000000000000000);
        &self.whiteKnights = BitBoard::new(0b0100001000000000000000000000000000000000000000000000000000000000);
        &self.whiteBishops = BitBoard::new(0b0010010000000000000000000000000000000000000000000000000000000000);
        &self.whiteRooks = BitBoard::new(0b1000000100000000000000000000000000000000000000000000000000000000);
        &self.whiteQueens = BitBoard::new(0b0001000000000000000000000000000000000000000000000000000000000000);
        &self.whiteKing = BitBoard::new(0b0000100000000000000000000000000000000000000000000000000000000000);
     
        &self.blackPawns = BitBoard::new(0b0000000000000000000000000000000000000000000000001111111100000000);
        &self.blackKnights = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000001000010);
        &self.blackBishops = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000100100);
        &self.blackRooks = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000010000001);
        &self.blackQueens = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000010000);
        &self.blackKing = BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000001000);
        
        &self.board = BitBoard::new(0b111111111111111100000000000000000000000000000000111111111111111);

    }

    fn generate_moves(&self, piece: PiecesTypes) {
        match piece {
            PiecesTypes.Pawn =>{
                let pawn_targets = ( &self.whitePawns << 8 ) & !&self.board;
                // TODO: pseudo_legal and legal moves
            }
        
        }
    }
}
