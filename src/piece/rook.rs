use super::{Color, Playable};

pub struct Rook {
    color: Color,
    symbol: char,
    has_moved: bool,
}

impl Rook {
    pub fn new_white() -> Self {
        Self {
            color: Color::White,
            symbol: 'R',
            has_moved: false,
        }
    }
    pub fn new_black() -> Self {
        Self {
            color: Color::Black,
            symbol: 'r',
            has_moved: false,
        }
    }
}

impl Playable for Rook {
    fn color(&self) -> Color {
        self.color
    }

    fn symbol(&self) -> char {
        self.symbol
    }
}
