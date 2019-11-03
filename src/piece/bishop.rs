use super::{Color, Playable};

pub struct Bishop {
    color: Color,
    symbol: char,
    has_moved: bool,
}

impl Bishop {
    pub fn new_white() -> Self {
        Self {
            color: Color::White,
            symbol: 'B',
            has_moved: false,
        }
    }
    pub fn new_black() -> Self {
        Self {
            color: Color::Black,
            symbol: 'b',
            has_moved: false,
        }
    }
}

impl Playable for Bishop {
    fn color(&self) -> Color {
        self.color
    }

    fn symbol(&self) -> char {
        self.symbol
    }
}
