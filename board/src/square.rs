use crate::BitBoard;
use derive_more::Display;
use std::ops::{BitAnd, BitOr, BitXor};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Display)]
#[repr(u8)]
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
    pub const ALL_SQUARES: [Self; 64] = [
        Self::A1,
        Self::B1,
        Self::C1,
        Self::D1,
        Self::E1,
        Self::F1,
        Self::G1,
        Self::H1,
        Self::A2,
        Self::B2,
        Self::C2,
        Self::D2,
        Self::E2,
        Self::F2,
        Self::G2,
        Self::H2,
        Self::A3,
        Self::B3,
        Self::C3,
        Self::D3,
        Self::E3,
        Self::F3,
        Self::G3,
        Self::H3,
        Self::A4,
        Self::B4,
        Self::C4,
        Self::D4,
        Self::E4,
        Self::F4,
        Self::G4,
        Self::H4,
        Self::A5,
        Self::B5,
        Self::C5,
        Self::D5,
        Self::E5,
        Self::F5,
        Self::G5,
        Self::H5,
        Self::A6,
        Self::B6,
        Self::C6,
        Self::D6,
        Self::E6,
        Self::F6,
        Self::G6,
        Self::H6,
        Self::A7,
        Self::B7,
        Self::C7,
        Self::D7,
        Self::E7,
        Self::F7,
        Self::G7,
        Self::H7,
        Self::A8,
        Self::B8,
        Self::C8,
        Self::D8,
        Self::E8,
        Self::F8,
        Self::G8,
        Self::H8,
    ];

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

    #[must_use]
    #[inline]
    pub const fn from_u64(value: u64) -> Option<Self> {
        match value {
            0 => Some(Self::A1),
            1 => Some(Self::B1),
            2 => Some(Self::C1),
            3 => Some(Self::D1),
            4 => Some(Self::E1),
            5 => Some(Self::F1),
            6 => Some(Self::G1),
            7 => Some(Self::H1),
            8 => Some(Self::A2),
            9 => Some(Self::B2),
            10 => Some(Self::C2),
            11 => Some(Self::D2),
            12 => Some(Self::E2),
            13 => Some(Self::F2),
            14 => Some(Self::G2),
            15 => Some(Self::H2),
            16 => Some(Self::A3),
            17 => Some(Self::B3),
            18 => Some(Self::C3),
            19 => Some(Self::D3),
            20 => Some(Self::E3),
            21 => Some(Self::F3),
            22 => Some(Self::G3),
            23 => Some(Self::H3),
            24 => Some(Self::A4),
            25 => Some(Self::B4),
            26 => Some(Self::C4),
            27 => Some(Self::D4),
            28 => Some(Self::E4),
            29 => Some(Self::F4),
            30 => Some(Self::G4),
            31 => Some(Self::H4),
            32 => Some(Self::A5),
            33 => Some(Self::B5),
            34 => Some(Self::C5),
            35 => Some(Self::D5),
            36 => Some(Self::E5),
            37 => Some(Self::F5),
            38 => Some(Self::G5),
            39 => Some(Self::H5),
            40 => Some(Self::A6),
            41 => Some(Self::B6),
            42 => Some(Self::C6),
            43 => Some(Self::D6),
            44 => Some(Self::E6),
            45 => Some(Self::F6),
            46 => Some(Self::G6),
            47 => Some(Self::H6),
            48 => Some(Self::A7),
            49 => Some(Self::B7),
            50 => Some(Self::C7),
            51 => Some(Self::D7),
            52 => Some(Self::E7),
            53 => Some(Self::F7),
            54 => Some(Self::G7),
            55 => Some(Self::H7),
            56 => Some(Self::A8),
            57 => Some(Self::B8),
            58 => Some(Self::C8),
            59 => Some(Self::D8),
            60 => Some(Self::E8),
            61 => Some(Self::F8),
            62 => Some(Self::G8),
            63 => Some(Self::H8),
            _ => None,
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

macro_rules! try_from_num {
    ($($t:ty)*) => {
        $(
            impl TryFrom<$t> for Square {
                type Error = ();

                #[inline]
                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    match value {
                        0 => Ok(Square::A1),
                        1 => Ok(Square::B1),
                        2 => Ok(Square::C1),
                        3 => Ok(Square::D1),
                        4 => Ok(Square::E1),
                        5 => Ok(Square::F1),
                        6 => Ok(Square::G1),
                        7 => Ok(Square::H1),
                        8 => Ok(Square::A2),
                        9 => Ok(Square::B2),
                        10 => Ok(Square::C2),
                        11 => Ok(Square::D2),
                        12 => Ok(Square::E2),
                        13 => Ok(Square::F2),
                        14 => Ok(Square::G2),
                        15 => Ok(Square::H2),
                        16 => Ok(Square::A3),
                        17 => Ok(Square::B3),
                        18 => Ok(Square::C3),
                        19 => Ok(Square::D3),
                        20 => Ok(Square::E3),
                        21 => Ok(Square::F3),
                        22 => Ok(Square::G3),
                        23 => Ok(Square::H3),
                        24 => Ok(Square::A4),
                        25 => Ok(Square::B4),
                        26 => Ok(Square::C4),
                        27 => Ok(Square::D4),
                        28 => Ok(Square::E4),
                        29 => Ok(Square::F4),
                        30 => Ok(Square::G4),
                        31 => Ok(Square::H4),
                        32 => Ok(Square::A5),
                        33 => Ok(Square::B5),
                        34 => Ok(Square::C5),
                        35 => Ok(Square::D5),
                        36 => Ok(Square::E5),
                        37 => Ok(Square::F5),
                        38 => Ok(Square::G5),
                        39 => Ok(Square::H5),
                        40 => Ok(Square::A6),
                        41 => Ok(Square::B6),
                        42 => Ok(Square::C6),
                        43 => Ok(Square::D6),
                        44 => Ok(Square::E6),
                        45 => Ok(Square::F6),
                        46 => Ok(Square::G6),
                        47 => Ok(Square::H6),
                        48 => Ok(Square::A7),
                        49 => Ok(Square::B7),
                        50 => Ok(Square::C7),
                        51 => Ok(Square::D7),
                        52 => Ok(Square::E7),
                        53 => Ok(Square::F7),
                        54 => Ok(Square::G7),
                        55 => Ok(Square::H7),
                        56 => Ok(Square::A8),
                        57 => Ok(Square::B8),
                        58 => Ok(Square::C8),
                        59 => Ok(Square::D8),
                        60 => Ok(Square::E8),
                        61 => Ok(Square::F8),
                        62 => Ok(Square::G8),
                        63 => Ok(Square::H8),
                        _ => Err(()),
                    }
                }
            }

            impl TryFrom<&$t> for Square {
                type Error = ();

                fn try_from(value: &$t) -> Result<Self, Self::Error> {
                    TryFrom::<$t>::try_from(*value)
                }
            }
        )*
    };
}

try_from_num!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 usize isize);
