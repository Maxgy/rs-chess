mod bishop;
mod color;
mod king;
mod knight;
mod pawn;
mod playable;
mod queen;
mod rook;

pub use bishop::Bishop;
pub use color::Color;
pub use king::King;
pub use knight::Knight;
pub use pawn::Pawn;
pub use playable::Playable;
pub use queen::Queen;
pub use rook::Rook;

pub enum Piece {
    Pawn(Pawn),
    Bishop(Bishop),
    Knight(Knight),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

impl Piece {
    pub fn new(symbol: char) -> Self {
        match symbol {
            'P' => Piece::Pawn(Pawn::new_white()),
            'p' => Piece::Pawn(Pawn::new_black()),
            'B' => Piece::Bishop(Bishop::new_white()),
            'b' => Piece::Bishop(Bishop::new_black()),
            'N' => Piece::Knight(Knight::new_white()),
            'n' => Piece::Knight(Knight::new_black()),
            'R' => Piece::Rook(Rook::new_white()),
            'r' => Piece::Rook(Rook::new_black()),
            'Q' => Piece::Queen(Queen::new_white()),
            'q' => Piece::Queen(Queen::new_black()),
            'K' => Piece::King(King::new_white()),
            'k' => Piece::King(King::new_black()),
            _ => Piece::Pawn(Pawn::new_white()),
        }
    }
}

impl Playable for Piece {
    fn color(&self) -> Color {
        match self {
            Self::Pawn(pawn) => pawn.color(),
            Self::Bishop(bishop) => bishop.color(),
            Self::Knight(knight) => knight.color(),
            Self::Rook(rook) => rook.color(),
            Self::Queen(queen) => queen.color(),
            Self::King(king) => king.color(),
        }
    }

    fn symbol(&self) -> char {
        match self {
            Self::Pawn(pawn) => pawn.symbol(),
            Self::Bishop(bishop) => bishop.symbol(),
            Self::Knight(knight) => knight.symbol(),
            Self::Rook(rook) => rook.symbol(),
            Self::Queen(queen) => queen.symbol(),
            Self::King(king) => king.symbol(),
        }
    }
}
