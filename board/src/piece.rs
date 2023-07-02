pub enum Color {
    Black,
    White,
}

pub enum Role {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl Piece {
    #[must_use]
    pub const fn from_parts(color: Color, role: Role) -> Self {
        #[allow(clippy::enum_glob_use)]
        use {Color::*, Piece::*, Role::*};

        match (color, role) {
            (White, Pawn) => WhitePawn,
            (White, Knight) => WhiteKnight,
            (White, Bishop) => WhiteBishop,
            (White, Rook) => WhiteRook,
            (White, Queen) => WhiteQueen,
            (White, King) => WhiteKing,
            (Black, Pawn) => BlackPawn,
            (Black, Knight) => BlackKnight,
            (Black, Bishop) => BlackBishop,
            (Black, Rook) => BlackRook,
            (Black, Queen) => BlackQueen,
            (Black, King) => BlackKing,
        }
    }
}
