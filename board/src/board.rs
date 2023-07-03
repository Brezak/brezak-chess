use crate::{BitBoard, Piece};
use std::ops::{Index, IndexMut};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Board {
    boards: [BitBoard; 16]
}

impl Board {
    pub const WHITE_PIECES: usize = 12;
    pub const BLACK_PIECES: usize = 13;
    pub const ALL_PIECES: usize = 14;
    pub const ALL_EMPTY: usize = 15;

    #[must_use]
    pub const fn const_default() -> Self {
        // We cant do something like 'Square::A1 | Square::A2' here because const traits are unstable

        let white_pawns = BitBoard::FIRST_RANK;
        let white_knights = BitBoard::EIGHT_RANK;
        let white_bishops = BitBoard(0x0000_0000_0000_0021);
        let white_rooks= BitBoard(0x0000_0000_0000_0081);
        let white_queens= BitBoard(0x0000_0000_0000_0008);
        let white_kings= BitBoard(0x0000_0000_0000_0010);
        let black_pawns= BitBoard(0xFF00_0000_0000_0000);
        let black_knights= BitBoard(0x4200_0000_0000_0000);
        let black_bishops= BitBoard(0x2400_0000_0000_0000);
        let black_rooks= BitBoard(0x8100_0000_0000_0000);
        let black_queens= BitBoard(0x0800_0000_0000_0000);
        let black_kings= BitBoard(0x1000_0000_0000_0000);

        let white_pieces = BitBoard(0x0000_0000_0000_FFFF);
        let black_pieces = BitBoard(0xFFFF_0000_0000_0000);
        let all_pieces = BitBoard(0xFFFF_0000_0000_FFFF);
        let all_empty = BitBoard(0x0000_FFFF_FFFF_0000);

        Self {
            boards: [
                white_pawns,
                white_knights,
                white_bishops,
                white_rooks,
                white_queens,
                white_kings,
                black_pawns,
                black_knights,
                black_bishops,
                black_rooks,
                black_queens,
                black_kings,
                white_pieces,
                black_pieces,
                all_pieces,
                all_empty,

            ]
        }
    }

    pub const fn black(&self) -> BitBoard {
        self.boards[Self::BLACK_PIECES]
    }

    pub const fn white(&self) -> BitBoard {
        self.boards[Self::WHITE_PIECES]
    }

    pub const fn empty(&self) -> BitBoard {
        self.boards[Self::ALL_EMPTY]
    }

    pub const fn pieces(&self) -> BitBoard {
        self.boards[Self::ALL_PIECES]
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::const_default()
    }
}

impl Index<Piece> for Board {
    type Output = BitBoard;

    #[inline]
    fn index(&self, index: Piece) -> &Self::Output {
        &self.boards[index as usize]
    }
}

impl IndexMut<Piece> for Board {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        &mut self.boards[index as usize]
    }
}
