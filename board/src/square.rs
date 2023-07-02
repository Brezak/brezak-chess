use crate::BitBoard;
use derive_more::Display;
use std::ops::{BitAnd, BitOr, BitXor};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Display)]
pub enum Square {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Display)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Display)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Square {
    #[must_use]
    pub const fn new(file: File, rank: Rank) -> Self {
        #[allow(clippy::enum_glob_use)]
        use {File::*, Rank::*, Square::*};

        match (file, rank) {
            (A, One) => A1,
            (A, Two) => A2,
            (A, Three) => A3,
            (A, Four) => A4,
            (A, Five) => A5,
            (A, Six) => A6,
            (A, Seven) => A7,
            (A, Eight) => A8,
            (B, One) => B1,
            (B, Two) => B2,
            (B, Three) => B3,
            (B, Four) => B4,
            (B, Five) => B5,
            (B, Six) => B6,
            (B, Seven) => B7,
            (B, Eight) => B8,
            (C, One) => C1,
            (C, Two) => C2,
            (C, Three) => C3,
            (C, Four) => C4,
            (C, Five) => C5,
            (C, Six) => C6,
            (C, Seven) => C7,
            (C, Eight) => C8,
            (D, One) => D1,
            (D, Two) => D2,
            (D, Three) => D3,
            (D, Four) => D4,
            (D, Five) => D5,
            (D, Six) => D6,
            (D, Seven) => D7,
            (D, Eight) => D8,
            (E, One) => E1,
            (E, Two) => E2,
            (E, Three) => E3,
            (E, Four) => E4,
            (E, Five) => E5,
            (E, Six) => E6,
            (E, Seven) => E7,
            (E, Eight) => E8,
            (F, One) => F1,
            (F, Two) => F2,
            (F, Three) => F3,
            (F, Four) => F4,
            (F, Five) => F5,
            (F, Six) => F6,
            (F, Seven) => F7,
            (F, Eight) => F8,
            (G, One) => G1,
            (G, Two) => G2,
            (G, Three) => G3,
            (G, Four) => G4,
            (G, Five) => G5,
            (G, Six) => G6,
            (G, Seven) => G7,
            (G, Eight) => G8,
            (H, One) => H1,
            (H, Two) => H2,
            (H, Three) => H3,
            (H, Four) => H4,
            (H, Five) => H5,
            (H, Six) => H6,
            (H, Seven) => H7,
            (H, Eight) => H8,
        }
    }
}

impl<T: Into<BitBoard>> BitOr<T> for Square {
    type Output = BitBoard;

    fn bitor(self, rhs: T) -> Self::Output {
        BitBoard::from(self) | rhs
    }
}

impl<T: Into<BitBoard>> BitXor<T> for Square {
    type Output = BitBoard;

    fn bitxor(self, rhs: T) -> Self::Output {
        BitBoard::from(self) ^ rhs
    }
}

impl<T: Into<BitBoard>> BitAnd<T> for Square {
    type Output = BitBoard;

    fn bitand(self, rhs: T) -> Self::Output {
        BitBoard::from(self) & rhs
    }
}
