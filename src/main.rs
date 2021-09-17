use std::fmt;

pub struct BitBoard(u64);

impl BitBoard {
    pub fn new (b: u64) -> BitBoard {
        return BitBoard(b);
    }

    pub fn display(&self) {
        let BitBoard(v) = &self;    // destructuring let
        println!("{:#025b}", v)
    }
}



impl fmt::Display for BitBoard {
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


struct Rooks(BitBoard);
struct Pawns(BitBoard);
struct Kings(BitBoard);
struct Queens(BitBoard);
struct Bishops(BitBoard);
struct Knights(BitBoard);

struct White(BitBoard);
struct Black(BitBoard);




fn main() {

    // let board = BitBoard::new(0b1000000100000000000000000000000000000000000000000000000010000001);
    // println!("{}", board);


    // initial state
    let pawns = Pawns(BitBoard::new(0b0000000011111111000000000000000000000000000000001111111100000000));
    let rooks = Rooks(BitBoard::new(0b1000000100000000000000000000000000000000000000000000000010000001));
    let knights = Knights(BitBoard::new(0b0100001000000000000000000000000000000000000000000000000001000010));
    let bishops = Bishops(BitBoard::new(0b0010010000000000000000000000000000000000000000000000000000100100));
    let kings = Kings(BitBoard::new(0b0000100000000000000000000000000000000000000000000000000000001000));
    let queens = Queens(BitBoard::new(0b0001000000000000000000000000000000000000000000000000000000010000));


    let whites = White(BitBoard::new(0b1111111111111111000000000000000000000000000000000000000000000000));
    let blacks = Black(BitBoard::new(0b000000000000000000000000000000000000000000000001111111111111111));

    println!("{}", pawns.0);
    println!("{}", rooks.0);
    println!("{}", knights.0);
    println!("{}", bishops.0);
    println!("{}", kings.0);
    println!("{}", queens.0);
    println!("{}", whites.0);
    println!("{}", blacks.0);
}
