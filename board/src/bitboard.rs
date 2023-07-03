use crate::Square;
use derive_more::{Not, Shl, ShlAssign, Shr, ShrAssign};
use std::fmt::{Display, Formatter};
use std::iter::FusedIterator;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug, Shl, ShlAssign, Shr, ShrAssign, Not)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub const A_FILE: Self = Self(0x0101_0101_0101_0101);
    pub const H_FILE: Self = Self(0x8080_8080_8080_8080);
    pub const FIRST_RANK: Self = Self(0x0000_0000_0000_00FF);
    pub const EIGHT_RANK: Self = Self(0xFF00_0000_0000_0000);
    pub const A1_H8_DIAGONAL: Self = Self(0x8040_2010_0804_0201);
    pub const H1_A8_ANTI_DIAGONAL: Self = Self(0x0102_0408_1020_4080);
    pub const LIGHT_SQUARES: Self = Self(0x55AA_55AA_55AA_55AA);
    pub const DARK_SQUARES: Self = Self(0xAA55_AA55_AA55_AA55);
    pub const EMPTY: Self = Self(0);
    pub const FULL: Self = Self(0xFFFF_FFFF_FFFF_FFFF);

    // We can't use the From trait since that's not const
    #[must_use]
    #[inline]
    pub const fn from_square(square: Square) -> Self {
        Self(1 << square as u64)
    }

    /// Checks if the board isn't occupied by any pieces
    #[must_use]
    #[inline]
    pub const fn is_empty(self) -> bool {
        self.0 == Self::EMPTY.0
    }

    /// Returns first occupied square
    #[must_use]
    #[inline]
    pub const fn first(self) -> Option<Square> {
        if self.is_empty() {
            None
        } else {
            Square::from_u64(self.0.trailing_zeros() as u64)
        }
    }

    /// Returns last occupied square
    #[must_use]
    #[inline]
    pub const fn last(self) -> Option<Square> {
        if self.is_empty() {
            None
        } else {
            Square::from_u64(63 - self.0.leading_zeros() as u64)
        }
    }

    /// Returns the bitboard with the first occupied square removed
    #[must_use]
    #[inline]
    pub const fn without_first(self) -> Self {
        let board = self.0;
        Self(board & board.wrapping_sub(1))
    }

    /// Returns the bitboard with the last occupied square removed
    #[must_use]
    #[inline]
    pub const fn without_last(self) -> Self {
        if self.is_empty() {
            return Self::EMPTY;
        }

        let mask = 1 << (63 - self.0.leading_zeros() as u64);

        Self(self.0 ^ mask)
    }

    #[inline]
    pub fn discard_first(&mut self) {
        if self.is_empty() {
            return;
        }

        let mask = 1 << self.0.trailing_zeros();

        *self ^= mask;
    }

    #[inline]
    pub fn discard_last(&mut self) {
        if self.is_empty() {
            return;
        }

        let mask = 1 << (63 - self.0.leading_zeros());

        *self ^= mask;
    }

    #[inline]
    pub fn pop_first(&mut self) -> Option<Square> {
        let piece = self.first();
        self.discard_first();
        piece
    }

    #[inline]
    pub fn pop_last(&mut self) -> Option<Square> {
        let piece = self.last();
        self.discard_last();
        piece
    }

    #[must_use]
    #[inline]
    pub const fn flip_vertical(self) -> Self {
        // see https://www0chessprogramming0org/Flipping_Mirroring_and_Rotating#FlipVertically

        Self(self.0.swap_bytes())
    }

    #[must_use]
    pub const fn flip_horizontal(self) -> Self {
        // see https://www.chessprogramming.org/Flipping_Mirroring_and_Rotating#MirrorHorizontally
        const K1: u64 = 0x5555_5555_5555_5555;
        const K2: u64 = 0x3333_3333_3333_3333;
        const K4: u64 = 0x0f0f_0f0f_0f0f_0f0f;

        let x = self.0;

        let x = ((x >> 1) & K1) + 2 * (x & K1);
        let x = ((x >> 2) & K2) + 4 * (x & K2);
        let x = ((x >> 4) & K4) + 16 * (x & K4);

        Self(x)
    }

    #[must_use]
    pub const fn flip_diagonal(self) -> Self {
        // see https://www.chessprogramming.org/Flipping_Mirroring_and_Rotating#FlipabouttheDiagonal
        const K1: u64 = 0x5500_5500_5500_5500;
        const K2: u64 = 0x3333_0000_3333_0000;
        const K4: u64 = 0x0f0f_0f0f_0000_0000;

        let x = self.0;

        let t = K4 & (x ^ (x << 28));
        let x = x ^ (t ^ (t >> 28));
        let t = K2 & (x ^ (x << 14));
        let x = x ^ (t ^ (t >> 14));
        let t = K1 & (x ^ (x << 7));
        let x = x ^ (t ^ (t >> 7));

        Self(x)
    }

    #[must_use]
    pub const fn flip_anti_diagonal(self) -> Self {
        // see https://www.chessprogramming.org/Flipping_Mirroring_and_Rotating#FlipabouttheAntidiagonal

        const K1: u64 = 0xaa00_aa00_aa00_aa00;
        const K2: u64 = 0xcccc_0000_cccc_0000;
        const K4: u64 = 0xf0f0_f0f0_0f0f_0f0f;
        let x = self.0;
        let t = x ^ (x << 36);
        let x = x ^ (K4 & (t ^ (x >> 36)));
        let t = K2 & (x ^ (x << 18));
        let x = x ^ (t ^ (t >> 18));
        let t = K1 & (x ^ (x << 9));
        let x = x ^ (t ^ (t >> 9));

        Self(x)
    }

    #[must_use]
    pub const fn rotate_90_clockwise(self) -> Self {
        self.flip_diagonal().flip_vertical()
    }

    #[must_use]
    pub const fn rotate_90_counterclockwise(self) -> Self {
        self.flip_vertical().flip_diagonal()
    }

    #[must_use]
    pub const fn rotate_180(self) -> Self {
        self.flip_horizontal().flip_vertical()
    }

    #[must_use]
    #[inline]
    pub const fn north(self) -> Self {
        Self(self.0 << 8)
    }

    #[must_use]
    #[inline]
    pub const fn south(self) -> Self {
        Self(self.0 >> 8)
    }

    #[must_use]
    #[inline]
    pub const fn east(self) -> Self {
        Self((self.0 << 1) & !Self::A_FILE.0)
    }

    #[must_use]
    #[inline]
    pub const fn west(self) -> Self {
        Self((self.0 >> 1) & !Self::H_FILE.0)
    }

    #[must_use]
    #[inline]
    pub const fn north_east(self) -> Self {
        Self((self.0 << 9) & !Self::A_FILE.0)
    }

    #[must_use]
    #[inline]
    pub const fn south_east(self) -> Self {
        Self((self.0 >> 7) & !Self::A_FILE.0)
    }

    #[must_use]
    #[inline]
    pub const fn north_west(self) -> Self {
        Self((self.0 << 7) & !Self::H_FILE.0)
    }

    #[must_use]
    #[inline]
    pub const fn south_west(self) -> Self {
        Self((self.0 >> 9) & !Self::H_FILE.0)
    }

    #[must_use]
    pub fn to_nice_display(self) -> String {
        let mut result = String::with_capacity(128);

        for byte in self.0.to_be_bytes() {
            for index in 0..u8::BITS {
                let separator = if index == 7 { '\n' } else { ' ' };

                let is_bit_one = (byte >> index) & 0x1 == 1;

                if is_bit_one {
                    result.push('1');
                } else {
                    result.push('.');
                }

                result.push(separator);
            }

            result.push(' ');
        }

        result
    }
}

impl From<u64> for BitBoard {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Square> for BitBoard {
    fn from(value: Square) -> Self {
        match value {
            Square::A1 => Self(1 << 0),
            Square::B1 => Self(1 << 1),
            Square::C1 => Self(1 << 2),
            Square::D1 => Self(1 << 3),
            Square::E1 => Self(1 << 4),
            Square::F1 => Self(1 << 5),
            Square::G1 => Self(1 << 6),
            Square::H1 => Self(1 << 7),
            Square::A2 => Self(1 << 8),
            Square::B2 => Self(1 << 9),
            Square::C2 => Self(1 << 10),
            Square::D2 => Self(1 << 11),
            Square::E2 => Self(1 << 12),
            Square::F2 => Self(1 << 13),
            Square::G2 => Self(1 << 14),
            Square::H2 => Self(1 << 15),
            Square::A3 => Self(1 << 16),
            Square::B3 => Self(1 << 17),
            Square::C3 => Self(1 << 18),
            Square::D3 => Self(1 << 19),
            Square::E3 => Self(1 << 20),
            Square::F3 => Self(1 << 21),
            Square::G3 => Self(1 << 22),
            Square::H3 => Self(1 << 23),
            Square::A4 => Self(1 << 24),
            Square::B4 => Self(1 << 25),
            Square::C4 => Self(1 << 26),
            Square::D4 => Self(1 << 27),
            Square::E4 => Self(1 << 28),
            Square::F4 => Self(1 << 29),
            Square::G4 => Self(1 << 30),
            Square::H4 => Self(1 << 31),
            Square::A5 => Self(1 << 32),
            Square::B5 => Self(1 << 33),
            Square::C5 => Self(1 << 34),
            Square::D5 => Self(1 << 35),
            Square::E5 => Self(1 << 36),
            Square::F5 => Self(1 << 37),
            Square::G5 => Self(1 << 38),
            Square::H5 => Self(1 << 39),
            Square::A6 => Self(1 << 40),
            Square::B6 => Self(1 << 41),
            Square::C6 => Self(1 << 42),
            Square::D6 => Self(1 << 43),
            Square::E6 => Self(1 << 44),
            Square::F6 => Self(1 << 45),
            Square::G6 => Self(1 << 46),
            Square::H6 => Self(1 << 47),
            Square::A7 => Self(1 << 48),
            Square::B7 => Self(1 << 49),
            Square::C7 => Self(1 << 50),
            Square::D7 => Self(1 << 51),
            Square::E7 => Self(1 << 52),
            Square::F7 => Self(1 << 53),
            Square::G7 => Self(1 << 54),
            Square::H7 => Self(1 << 55),
            Square::A8 => Self(1 << 56),
            Square::B8 => Self(1 << 57),
            Square::C8 => Self(1 << 58),
            Square::D8 => Self(1 << 59),
            Square::E8 => Self(1 << 60),
            Square::F8 => Self(1 << 61),
            Square::G8 => Self(1 << 62),
            Square::H8 => Self(1 << 63),
        }
    }
}

impl<T: Into<Self>> BitOr<T> for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: T) -> Self::Output {
        Self(self.0 | rhs.into().0)
    }
}

impl<T: Into<Self>> BitOrAssign<T> for BitBoard {
    fn bitor_assign(&mut self, rhs: T) {
        self.0 |= rhs.into().0;
    }
}

impl<T: Into<Self>> BitXor<T> for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: T) -> Self::Output {
        Self(self.0 ^ rhs.into().0)
    }
}

impl<T: Into<Self>> BitXorAssign<T> for BitBoard {
    fn bitxor_assign(&mut self, rhs: T) {
        self.0 ^= rhs.into().0;
    }
}

impl<T: Into<Self>> BitAnd<T> for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: T) -> Self::Output {
        Self(self.0 & rhs.into().0)
    }
}

impl<T: Into<Self>> BitAndAssign<T> for BitBoard {
    fn bitand_assign(&mut self, rhs: T) {
        self.0 &= rhs.into().0;
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            writeln!(f, "{}", self.to_nice_display())?;
            Ok(())
        } else {
            write!(f, "{:08b}", self.0)
        }
    }
}

impl IntoIterator for BitBoard {
    type Item = Square;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { board: self }
    }
}

/// An iterator over the occupied squares of the bitboard
pub struct IntoIter {
    board: BitBoard,
}

impl Iterator for IntoIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        self.board.pop_first()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.board.0.count_ones() as usize;

        (len, Some(len))
    }
}

impl ExactSizeIterator for IntoIter {
    fn len(&self) -> usize {
        // Manual implementation to guarantee that this function doesn't panic
        self.board.0.count_ones() as usize
    }
}

impl DoubleEndedIterator for IntoIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.board.pop_last()
    }
}

impl FusedIterator for IntoIter {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swaps() {
        // 0 1 1 1 1 0 0 0
        // 0 1 0 0 0 1 0 0
        // 0 1 0 0 0 1 0 0
        // 0 1 0 0 1 0 0 0
        // 0 1 1 1 0 0 0 0
        // 0 1 0 1 0 0 0 0
        // 0 1 0 0 1 0 0 0
        // 0 1 0 0 0 1 0 0
        let r = 0b0111_1000_0100_0100_0100_0100_0100_1000_0111_0000_0101_0000_0100_1000_0100_0100;

        // 0 1 0 0 0 1 0 0
        // 0 1 0 0 1 0 0 0
        // 0 1 0 1 0 0 0 0
        // 0 1 1 1 0 0 0 0
        // 0 1 0 0 1 0 0 0
        // 0 1 0 0 0 1 0 0
        // 0 1 0 0 0 1 0 0
        // 0 1 1 1 1 0 0 0
        let v_flipped_r =
            0b0100_0100_0100_1000_0101_0000_0111_0000_0100_1000_0100_0100_0100_0100_0111_1000;

        // 0 0 0 1 1 1 1 0
        // 0 0 1 0 0 0 1 0
        // 0 0 1 0 0 0 1 0
        // 0 0 0 1 0 0 1 0
        // 0 0 0 0 1 1 1 0
        // 0 0 0 0 1 0 1 0
        // 0 0 0 1 0 0 1 0
        // 0 0 1 0 0 0 1 0
        let h_flipped_r =
            0b0001_1110_0010_0010_0010_0010_0001_0010_0000_1110_0000_1010_0001_0010_0010_0010;

        // 00011110
        // 00100010
        // 00100010
        // 00010010
        // 00001110
        // 00000010
        // 00010010
        // 00100010

        //panic!("{:#}", BitBoard(h_flipped_r));
        assert_eq!(
            BitBoard(r).flip_vertical(),
            BitBoard(v_flipped_r),
            "Flip vertical"
        );
        assert_eq!(
            BitBoard(r).flip_horizontal(),
            BitBoard(h_flipped_r),
            "Flip horizontal"
        );

        assert_eq!(
            BitBoard(r).flip_horizontal().flip_vertical(),
            BitBoard(r).flip_vertical().flip_horizontal()
        );
    }

    #[test]
    fn iterate_empty() {
        let empty = BitBoard::EMPTY;
        assert_eq!(empty.into_iter().count(), 0);
    }

    #[test]
    fn iterate_full() {
        let full = BitBoard::FULL;
        assert!(full.into_iter().eq(Square::ALL_SQUARES));
    }

    #[test]
    fn iterate_last() {
        let mut last = BitBoard(0x8000_0000_0000_0000).into_iter();

        assert_eq!(last.next(), Some(Square::H8));
    }
}
