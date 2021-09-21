// some BitBoard ideas taken from: https://docs.rs/chess/0.3.4/src/chess/.cargo/registry/src/github.com-1ecc6299db9ec823/chess-0.3.4/src/bitboard.rs.html#11

pub mod board_structure {
    use std::fmt;

    pub struct BitBoard(u64);

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
    }


    // TODO: a razie do cech używane są obiekty jako wypozyczone i zwracany jest zawsze nowy Bitboard - mieć na uwadze czy to nie spowoduje problemów w dalszej perspektywie
    // BitBoard traits
    impl fmt::Display for &BitBoard {
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

    enum Color {WHITE, BLACK, BOTH}


}
