use crate::{Piece, Square};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct FromTo {
    from: Square,
    to: Square,
}

impl FromTo {
    #[inline]
    pub const fn from(self) -> Square {
        self.from
    }

    #[inline]
    pub const fn to(self) -> Square {
        self.to
    }
}

pub enum Move {
    Move {
        from_to: FromTo,
        promotion: Option<Piece>,
        capture: Option<Piece>,
    },

    EnPassant {
        from_to: FromTo,
    },

    Castle {
        from: Square,
        to: Square,
    }
}