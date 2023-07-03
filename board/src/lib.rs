#![warn(clippy::pedantic, clippy::nursery)]

pub mod bitboard;
pub mod rule_sets;
mod board;
mod piece;
mod square;
mod r#move;

pub use bitboard::BitBoard;
pub use board::Board;
pub use piece::*;
pub use square::*;
pub use r#move::Move;

#[cfg(test)]
mod tests {}
