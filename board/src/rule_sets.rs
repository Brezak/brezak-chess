mod standard;

pub use standard::StandardRules;
use crate::Move;

pub trait RuleSet {
    fn legal_moves(&self) -> Vec<Move>;
}