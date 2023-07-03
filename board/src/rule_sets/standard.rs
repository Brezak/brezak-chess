mod constants;

use crate::{Board, Move};
use super::RuleSet;

#[derive(Eq, PartialEq, Clone, Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct StandardRules {
    board: Board,
}

impl StandardRules {
}

impl RuleSet for StandardRules {
    fn legal_moves(&self) -> Vec<Move> {
        todo!()
    }
}
