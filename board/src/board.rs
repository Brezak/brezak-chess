use crate::{BitBoard, Square};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Board {
    white_pawns: BitBoard,
    white_knights: BitBoard,
    white_bishops: BitBoard,
    white_rooks: BitBoard,
    white_queens: BitBoard,
    white_kings: BitBoard,
    black_pawns: BitBoard,
    black_knights: BitBoard,
    black_bishops: BitBoard,
    black_rooks: BitBoard,
    black_queens: BitBoard,
    black_kings: BitBoard,
}

impl Board {
    #[must_use]
    pub const fn const_default() -> Self {
        // We cant do something like 'Square::A1 | Square::A2' here because const traits are unstable

        Self {
            white_pawns: BitBoard::FIRST_RANK,
            white_knights: BitBoard::EIGHT_RANK,
            white_bishops: BitBoard(0x0000_0000_0000_0021),
            white_rooks: BitBoard(0x0000_0000_0000_0081),
            white_queens: BitBoard(0x0000_0000_0000_0008),
            white_kings: BitBoard(0x0000_0000_0000_0010),
            black_pawns: BitBoard(0xFF00_0000_0000_0000),
            black_knights: BitBoard(0x4200_0000_0000_0000),
            black_bishops: BitBoard(0x2400_0000_0000_0000),
            black_rooks: BitBoard(0x8100_0000_0000_0000),
            black_queens: BitBoard(0x0800_0000_0000_0000),
            black_kings: BitBoard(0x1000_0000_0000_0000),
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::const_default()
    }
}
