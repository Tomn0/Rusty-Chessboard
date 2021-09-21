    // data structures

    // chess board 8x8 + protective squares (two rows on top and bottom and one on every board) = 120 squares
    
    // pieces
    pub enum PiecesTypes {EMPTY, Pawn, Knight, Bishop, Rook, Queen, King};
    enum Colors {WHITE, BLACK, BOTH};
    enum Rows {FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, FILE_NONE};
    enum Columns {RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8, RANK_NONE};

    enum Squares {
        A1 = 21, B1, C1, D1, E1, F1, G1, H1,
        A2 = 31, B2, C2, D2, E2, F2, G2, H2,
        A3 = 41, B3, C3, D3, E3, F3, G3, H3,
        A4 = 51, B4, C4, D4, E4, F4, G4, H4,
        A5 = 61, B5, C5, D5, E5, F5, G5, H5,
        A6 = 71, B6, C6, D6, E6, F6, G6, H6,
        A7 = 81, B7, C7, D7, E7, F7, G7, H7,
        A8 = 91, B8, C8, D8, E8, F8, G8, H8, 
        NO_SQ
    };

    enum Bools {FALSE, TRUE};

    struct Game {
        event: 
        [Event "Chess.com Staff Tournament #2"]
        [Site "Chess.com"]
        [Date "2010.10.26"]
        [White "ACEChess"]
        [Black "piotr"]
        [Result "1-0"]
        [WhiteElo "2037"]
        [BlackElo "2125"]
        [TimeControl "1 in 3 days"]
        [Termination "ACEChess won by resignation"]
    }

    enum Outcome {
        White_Won = String.from("1:0"),
        Black_Won = String::from("0:1"),
        Draw = String::from("0.5:0.5")
    }

        // pub const A1: u8 = 1;
    // pub const B1 = 2;
    // pub const C1 = 3; 
    // pub const D1 = 4; 
    // pub const E1 = 5; 
    // pub const F1; 
    // pub const G1; 
    // pub const H1;
    // pub const A2 = 9; 
    // pub const B2;
    // pub const C2; 
    // pub const D2; 
    // pub const E2; 
    // pub const F2; 
    // pub const G2; 
    // pub const H2;
    // pub const A3 = 18; 
    // pub const B3;
    // pub const C3; 
    // pub const D3; 
    // pub const E3; 
    // pub const F3; 
    // pub const G3; 
    // pub const H3;
    // pub const A4 = 27; 
    // pub const B4;
    // pub const C4; 
    // pub const D4; 
    // pub const E4; 
    // pub const F4; 
    // pub const G4; 
    // pub const H4;
    // pub const A5 = 36; 
    // pub const B5;
    // pub const C5; 
    // pub const D5; 
    // pub const E5; 
    // pub const F5; 
    // pub const G5; 
    // pub const H5;
    // pub const A6 = 45; 
    // pub const B6;
    // pub const C6; 
    // pub const D6; 
    // pub const E6; 
    // pub const F6; 
    // pub const G6; 
    // pub const H6;
    // pub const A7 = 54; 
    // pub const B7;
    // pub const C7;
    // pub const D7; 
    // pub const E7; 
    // pub const F7; 
    // pub const G7; 
    // pub const H7;
    // pub const A8 = 63;
    // pub const B8;
    // pub const C8; 
    // pub const D8; 
    // pub const E8; 
    // pub const F8; 
    // pub const G8; 
    // pub const H8; 
    // NoSQ

// left: https://www.youtube.com/watch?v=3uBCUF_qHcg