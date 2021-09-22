pub mod rank {
    use crate::square::square::{ Square, BOARD_SQUARES };

    // possibly redundant with Square struct
    #[derive(PartialEq)]
    pub enum Rank {
        First,
        Second,
        Third,
        Forth,
        Fifth,
        Sixth,
        Seventh,
        Eighth
    }

    pub fn get_rank(square: Square) -> Rank {
        let rank: Rank;
        match square.id {
            0..=7 => rank = Rank::First,
            8..=15 => rank = Rank::Second,
            16..=23 => rank = Rank::Third,
            24..=31 => rank = Rank::Forth,
            32..=39 => rank = Rank::Fifth,
            40..=47 => rank = Rank::Sixth,
            48..=55 => rank = Rank::Seventh,
            56..=63 => rank = Rank::Eighth,
            _ => panic!("square identifier not recognizable!!")
        }
        rank
    }

}

