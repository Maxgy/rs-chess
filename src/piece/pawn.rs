use super::{Color, Playable};

pub struct Pawn {
    color: Color,
    symbol: char,
    has_moved: bool,
}

impl Pawn {
    pub fn new_white() -> Self {
        Self {
            color: Color::White,
            symbol: 'P',
            has_moved: false,
        }
    }
    pub fn new_black() -> Self {
        Self {
            color: Color::Black,
            symbol: 'p',
            has_moved: false,
        }
    }
}

impl Playable for Pawn {
    fn color(&self) -> Color {
        self.color
    }

    fn symbol(&self) -> char {
        self.symbol
    }
}
