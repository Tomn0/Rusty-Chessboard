pub mod file {
    use crate::square::square::{ Square, BOARD_SQUARES };

    // possibly redundant with Square struct
    pub enum File {
        First,
        Second,
        Third,
        Forth,
        Fifth,
        Sixth,
        Seventh,
        Eighth

    }

    pub fn get_file(square: Square) -> File {
        let file: File;
        match square.id {
            0..=7 => file = File::First,
            8..=15 => file = File::Second,
            16..=23 => file = File::Third,
            24..=31 => file = File::Forth,
            32..=39 => file = File::Fifth,
            40..=47 => file = File::Sixth,
            48..=55 => file = File::Seventh,
            56..=63 => file = File::Eighth,
            _ => panic!("square identifier not recognizable!!")
        }
        file
    }

}

