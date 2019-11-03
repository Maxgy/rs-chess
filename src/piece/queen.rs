use super::{Color, Playable};

pub struct Queen {
    color: Color,
    symbol: char,
    has_moved: bool,
}

impl Queen {
    pub fn new_white() -> Self {
        Self {
            color: Color::White,
            symbol: 'Q',
            has_moved: false,
        }
    }
    pub fn new_black() -> Self {
        Self {
            color: Color::Black,
            symbol: 'q',
            has_moved: false,
        }
    }
}

impl Playable for Queen {
    fn color(&self) -> Color {
        self.color
    }

    fn symbol(&self) -> char {
        self.symbol
    }
}
