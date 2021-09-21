
pub mod square {
    use crate::board::board::Color;

    pub struct Square<'a> {
        id: u8,
        coord: &'a str,
        color: Color,
    }



    pub const BOARD_SQUARES: [Square; 64] = [
    Square{id: 0, coord: "A1", color: Color::Black},
    Square{id: 1, coord: "B1", color: Color::White},
    Square{id: 2, coord: "C1", color: Color::Black},  
    Square{id: 3, coord: "D1", color: Color::White},
    Square{id: 4, coord: "E1", color: Color::Black},
    Square{id: 5, coord: "F1", color: Color::White},  
    Square{id: 6, coord: "G1", color: Color::Black}, 
    Square{id: 7, coord: "H1", color: Color::White},

    
    Square{id: 8, coord: "A2", color: Color::White},
    Square{id: 9, coord: "B2", color: Color::Black},  
    Square{id: 10, coord: "C2", color: Color::White},
    Square{id: 11, coord: "D2", color: Color::Black},
    Square{id: 12, coord: "E2", color: Color::White},  
    Square{id: 13, coord: "F2", color: Color::Black}, 
    Square{id: 14, coord: "G2", color: Color::White},
    Square{id: 15, coord: "H2", color: Color::Black},


    Square{id: 16, coord: "A3", color: Color::Black},
    Square{id: 17, coord: "B3", color: Color::White},
    Square{id: 18, coord: "C3", color: Color::Black},  
    Square{id: 19, coord: "D3", color: Color::White},
    Square{id: 20, coord: "E3", color: Color::Black},
    Square{id: 21, coord: "F3", color: Color::White},  
    Square{id: 22, coord: "G3", color: Color::Black}, 
    Square{id: 23, coord: "H3", color: Color::White},


    Square{id: 24, coord: "A4", color: Color::White},
    Square{id: 25, coord: "B4", color: Color::Black},  
    Square{id: 26, coord: "C4", color: Color::White},
    Square{id: 27, coord: "D4", color: Color::Black},
    Square{id: 28, coord: "E4", color: Color::White},  
    Square{id: 29, coord: "F4", color: Color::Black}, 
    Square{id: 30, coord: "G4", color: Color::White},
    Square{id: 31, coord: "H1", color: Color::Black},


    Square{id: 32, coord: "A5", color: Color::Black},
    Square{id: 33, coord: "B5", color: Color::White},
    Square{id: 34, coord: "C5", color: Color::Black},  
    Square{id: 35, coord: "D5", color: Color::White},
    Square{id: 36, coord: "E5", color: Color::Black},
    Square{id: 37, coord: "F5", color: Color::White},  
    Square{id: 38, coord: "G5", color: Color::Black}, 
    Square{id: 39, coord: "H5", color: Color::White},


    Square{id: 40, coord: "A6", color: Color::White},
    Square{id: 41, coord: "B6", color: Color::Black},  
    Square{id: 42, coord: "C6", color: Color::White},
    Square{id: 43, coord: "D6", color: Color::Black},
    Square{id: 44, coord: "E6", color: Color::White},  
    Square{id: 45, coord: "F6", color: Color::Black}, 
    Square{id: 46, coord: "G6", color: Color::White},
    Square{id: 47, coord: "H6", color: Color::Black},


    Square{id: 48, coord: "A7", color: Color::Black},
    Square{id: 49, coord: "B7", color: Color::White},
    Square{id: 50, coord: "C7", color: Color::Black},  
    Square{id: 51, coord: "D7", color: Color::White},
    Square{id: 52, coord: "E7", color: Color::Black},
    Square{id: 53, coord: "F7", color: Color::White},  
    Square{id: 54, coord: "G7", color: Color::Black}, 
    Square{id: 55, coord: "H7", color: Color::White},


    Square{id: 56, coord: "A8", color: Color::White},
    Square{id: 57, coord: "B8", color: Color::Black},  
    Square{id: 58, coord: "C8", color: Color::White},
    Square{id: 59, coord: "D8", color: Color::Black},
    Square{id: 60, coord: "E8", color: Color::White},  
    Square{id: 61, coord: "F8", color: Color::Black}, 
    Square{id: 62, coord: "G8", color: Color::White},
    Square{id: 63, coord: "H8", color: Color::Black}
    ];


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
}