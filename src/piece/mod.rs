pub struct Piece {
    pub symbol: String,
}

impl Piece {
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_string(),
        }
    }
}
