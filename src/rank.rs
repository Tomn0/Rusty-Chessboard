pub mod rank {
    use crate::square::square::{ Square, BOARD_SQUARES };

    pub enum Rank {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H
    }

    pub fn get_rank(square: Square) -> Rank {
        let rank: Rank;
        match square.id % 8 {
            0 => rank = Rank::A,
            1 => rank = Rank::B,
            2 => rank = Rank::C,
            3 => rank = Rank::D,
            4 => rank = Rank::E,
            5 => rank = Rank::F,
            6 => rank = Rank::G,
            7 => rank = Rank::H,
            _ => panic!("square identifier not recognizable!!")
        }
        rank
    }

}

