pub mod file {
    use crate::square::square::{ Square, BOARD_SQUARES };

    // possibly redundant with Square struct
    #[derive(PartialEq)]
    pub enum File {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H
    }

    pub fn get_file(square: Square) -> File {
        let file: File;
        match square.id % 8 {
            0 => file = File::A,
            1 => file = File::B,
            2 => file = File::C,
            3 => file = File::D,
            4 => file = File::E,
            5 => file = File::F,
            6 => file = File::G,
            7 => file = File::H,
            _ => panic!("square identifier not recognizable!!")
        }
        file
    }

}

