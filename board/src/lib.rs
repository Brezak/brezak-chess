#![warn(clippy::pedantic, clippy::nursery)]

pub mod bitboard;
mod board;
mod piece;
mod square;

pub use bitboard::BitBoard;
pub use piece::*;
pub use square::*;

#[cfg(test)]
mod tests {
    use super::*;
}
