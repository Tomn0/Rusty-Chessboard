
pub mod board {
    use crate::structs::board_structure::BitBoard;
    use crate::square::square::{BOARD_SQUARES, Square};


/**********************************\
 ==================================
 
                Enums
 
 ==================================
\**********************************/    

    #[derive(Copy, Clone)]
    pub enum Color {White, Black, Empty}


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
/**********************************\
 ==================================
 
            Moves
 
 ==================================
\**********************************/


        // jak generować ruchy w bitboardach? Też wszystkie razem czy dla każdej figury osobno?
        // a jak osobno to w jaki sposób te ruchy przechowywać
        pub fn generate_moves(&self, piece: PieceType, side: Color, square: Square) -> BitBoard{
            match piece {
                PieceType::Pawn =>{
                    // UNUSED
                    let pawn_targets = &( &self.white_pawns << 8i32 ) & &( !&self.chessboard );
                    // println!("First pawn targets: ");
                    // println!("{}", &pawn_targets);
                    // TODO: pseudo_legal and legal moves
                    // pseudo_legal - select empty squares
                    // advance move - TODO -> first move by two squares
                    let pawns: BitBoard;
                    let moves: BitBoard;
                    if let Color::White = side {
                        pawns = self.white_pawns;
                        moves = BitBoard::new((pawns.get_u64() & (1u64 << square.id as u64)) << 8u8);
                    }
                    else {
                        // println!("{}", )
                        pawns = self.black_pawns;
                        moves = BitBoard::new((pawns.get_u64() & (9223372036854775808u64 >> 63 - square.id as u64)) >> 8u8);
                        // println!("{}", (9223372036854775808u64 >> square.id));
                        // println!("{}", 9223372036854775808u64 >> (63 - square.id) as u64);
                        // println!("{}", pawns.get_u64());
                        
                    }
                    // println!("pawns.get_u64 = {}", pawns.get_u64());
                    // println!("pawns.get_u64 & square.id as u64 {}", pawns.get_u64() & square.id as u64);


                    
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
        fn pawn_attacks(side: Color) {

            // result attack bitboard
            let attacks = 0u64;

            // set piece on board

            // white pawns

            // black pawns
        }

    }    
}
