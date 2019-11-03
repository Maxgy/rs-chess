use super::{Color, Playable};

pub struct King {
    color: Color,
    symbol: char,
    has_moved: bool,
}

impl King {
    pub fn new_white() -> Self {
        Self {
            color: Color::White,
            symbol: 'K',
            has_moved: false,
        }
    }
    pub fn new_black() -> Self {
        Self {
            color: Color::Black,
            symbol: 'k',
            has_moved: false,
        }
    }
}

impl Playable for King {
    fn color(&self) -> Color {
        self.color
    }

    fn symbol(&self) -> char {
        self.symbol
    }
}
