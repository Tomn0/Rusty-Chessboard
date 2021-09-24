// some BitBoard ideas taken from: https://docs.rs/chess/0.3.4/src/chess/.cargo/registry/src/github.com-1ecc6299db9ec823/chess-0.3.4/src/bitboard.rs.html#11

pub mod bitboard {
    use std::fmt;
    use crate::square::square::{BOARD_SQUARES, Square};

    #[derive(Copy, Clone)]
    pub struct BitBoard(u64);

/**********************************\
 ==================================
 
            Implementation
 
 ==================================
\**********************************/ 

    impl BitBoard {
        pub fn new (b: u64) -> BitBoard {
            return BitBoard(b);
        }

        pub fn get_u64(&self) -> &u64 {
            let BitBoard(v) = &self;    // destructuring let
            v
        }

        // use this function to update BitBoard value
        pub fn update(mut self, new_v: u64) {
            self.0 = new_v;
        }

        // function: get_bitboard_from_square
        pub fn get_bitboard_from_square(square: &Square) -> BitBoard {
            let new_val: u64 = 1 << square.id;
            let bitboard = BitBoard::new(new_val);
            return bitboard;
        }

        fn power_of_two(n:u64) -> bool {
            /// checks if the number n is a power of two
            /// if so n & (n-1) should give 0 (ex: 16 (10000) & 15 (01111) = 0)
            /// except 0 -> we add the condition that 
            let mut m = *&n as i32;
            return m != 0 &&  !(m & (m - 1)) != 0 ;
        }

        pub fn get_square_from_bitboard(bitboard: &BitBoard) -> Option<Square> {
            
            let BitBoard(mut v) = bitboard;
            if !BitBoard::power_of_two(v) {
                return None;
            }
            let mut count = 0;
            while v != 1 {
                v = v / 2;
                count += 1;
            }

            let square: Square = BOARD_SQUARES[count];

            return Some(square);
        }




    }

/**********************************\
 ==================================
 
              Traits
 
 ==================================
\**********************************/ 

    // TODO: na razie do cech używane są obiekty jako wypozyczone i zwracany jest zawsze nowy Bitboard - mieć na uwadze czy to nie spowoduje problemów w dalszej perspektywie
    // BitBoard traits
    impl fmt::Display for &BitBoard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut repr = String::new();
            let mut line = String::new();
            for x in (0..64).rev() {
                if self.0 & ( 1u64 << x ) == 1u64 << x {
                    line = format!("1{}", line);
                    // repr.push('1');
                }
                else {
                    // repr.push('-');
                    line = format!("-{}", line);
                }
                
                
                if x % 8 == 0 {
                    // repr.push('\n');
                    line = format!("\n{}", line);
                    repr = format!("{}{}", repr, line);
                    line = String::new();
                }
            }
            write!(f, "{}", repr)
        }
    }

    impl std::ops::Shl<i32> for &BitBoard {
        type Output = BitBoard;

        fn shl(self, other: i32) -> BitBoard {
            let val = self.get_u64();
            // let right = other.get_u64();
            
            let shifted = val << other;
            BitBoard::new(shifted)
        }
    }

    impl std::ops::Shr<i32> for &BitBoard {
        type Output = BitBoard;

        fn shr(self, other: i32) -> BitBoard {
            let val = self.get_u64();
            // let right = other.get_u64();
            
            let shifted = val >> other;
            BitBoard::new(shifted)
        }
    }

    impl std::ops::Not for &BitBoard {
        type Output = BitBoard;

        fn not(self) -> BitBoard {
            let val = self.get_u64();

            let neg = !val;
            BitBoard::new(neg)
        }
    }

    impl std::ops::BitAnd for &BitBoard {
        type Output = BitBoard;

        fn bitand(self, other: &BitBoard) -> BitBoard {
            let left = self.get_u64();
            let right = other.get_u64();

            BitBoard::new(left & right)
        }
    }

    impl std::ops::BitOr for &BitBoard {
        type Output = BitBoard;

        fn bitor(self, other: &BitBoard) -> BitBoard {
            let left = self.get_u64();
            let right = other.get_u64();

            BitBoard::new(left | right)
        }
    }

    impl std::ops::BitXor for &BitBoard {
        type Output = BitBoard;
        
        fn bitxor(self, other: &BitBoard) -> BitBoard {
            let left = self.get_u64();
            let right = other.get_u64();

            BitBoard::new(left ^ right)
        }
    }

    impl std::ops::Add for &BitBoard {
        /// Is used to add two bitboards together.
        /// For example: to count all possible piece moves we sum advance moves with capture moves
        type Output = BitBoard;
        
        // FIXME: ownership?? maybe borrow is enough
        fn add(self, other: &BitBoard) -> BitBoard {
            // FIXME: check the case when in both bitboard the same bits are set!!!!
            let sum = self.get_u64() + other.get_u64();
            // self.update(sum);

            return BitBoard::new(sum);
        }
    }

    impl std::cmp::PartialEq for BitBoard {
        /// Is used to compare two bitboards together.
        fn eq(&self, other: &BitBoard) -> bool {
            let left = &self.get_u64();
            let right = &other.get_u64();

            if left == right {
                true
            }
            else {
                false
            }
        }

        fn ne(&self, other: &BitBoard) -> bool {
            let left = &self.get_u64();
            let right = &other.get_u64();

            if left != right {
                true
            }
            else {
                false
            }
        }
    }
}


/**********************************\
 ==================================
 
                Tests
 
 ==================================
\**********************************/ 

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn bitboard_display() {
        let piece: u64 = 576460752303423488;
        let bitboard: BitBoard = BitBoard::new(piece);
        println!("{}", &bitboard);

    }
    
    #[test]
    fn square_to_bitboard() {
        let square = BOARD_SQUARES[3];
        println!("{:?}", square);
        let left = &BitBoard::get_bitboard_from_square(&square);
        let right = &BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000001000);

        println!("{}", left);
        println!("{}", right);

        assert_eq!(format!("{}", left), format!("{}", right));
    }

    #[test]
    fn bitboard_to_square() {
        let bitboard = &BitBoard::new(0b0000000000000000000000000000000000000000000000000000000000010000);
        let result = &BitBoard::get_square_from_bitboard(&bitboard);
        let left: Square;
        match result {
            Some(e) => left = *e,
            None => panic!("The function returned 'None'"),
        }
        
        let right = BOARD_SQUARES[4];

        assert_eq!(left.id, right.id);
    }

    #[test]
    fn chessboard_initial_state() {
        let mut board = Board::new();
        board = board.start_game();
        assert_eq!(format!("{}", &board.chessboard), "\n11111111\n11111111\n--------\n--------\n--------\n--------\n11111111\n11111111");
    }

}