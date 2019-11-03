use super::{Color, Playable};

pub struct Knight {
    color: Color,
    symbol: char,
    has_moved: bool,
}

impl Knight {
    pub fn new_white() -> Self {
        Self {
            color: Color::White,
            symbol: 'N',
            has_moved: false,
        }
    }
    pub fn new_black() -> Self {
        Self {
            color: Color::Black,
            symbol: 'n',
            has_moved: false,
        }
    }
}

impl Playable for Knight {
    fn color(&self) -> Color {
        self.color
    }

    fn symbol(&self) -> char {
        self.symbol
    }
}
