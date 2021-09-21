
pub mod square {
    use crate::board::board::Color;

    pub const SIZE: usize = 8;
    pub const NUM_OF_SQUARES: usize = 64;

    pub struct Square<'a> {
        pub id: u8,
        pub coord: &'a str,
        pub color: Color,
    }



    pub const BOARD_SQUARES: [Square; NUM_OF_SQUARES] = [
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
    
}