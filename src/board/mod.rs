use std::io;
use std::io::Write;

use crate::piece::Piece;

pub fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error reading from stdin");
    input.trim().to_string()
}

#[derive(Default)]
pub struct Board {
    layout: Vec<Vec<Option<Piece>>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            layout: vec![
                vec![
                    Some(Piece::new("R")),
                    Some(Piece::new("N")),
                    Some(Piece::new("B")),
                    Some(Piece::new("K")),
                    Some(Piece::new("Q")),
                    Some(Piece::new("B")),
                    Some(Piece::new("N")),
                    Some(Piece::new("R")),
                ],
                vec![
                    Some(Piece::new("P")),
                    Some(Piece::new("P")),
                    Some(Piece::new("P")),
                    Some(Piece::new("P")),
                    Some(Piece::new("P")),
                    Some(Piece::new("P")),
                    Some(Piece::new("P")),
                    Some(Piece::new("P")),
                ],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![
                    Some(Piece::new("p")),
                    Some(Piece::new("p")),
                    Some(Piece::new("p")),
                    Some(Piece::new("p")),
                    Some(Piece::new("p")),
                    Some(Piece::new("p")),
                    Some(Piece::new("p")),
                    Some(Piece::new("p")),
                ],
                vec![
                    Some(Piece::new("r")),
                    Some(Piece::new("n")),
                    Some(Piece::new("b")),
                    Some(Piece::new("q")),
                    Some(Piece::new("k")),
                    Some(Piece::new("b")),
                    Some(Piece::new("n")),
                    Some(Piece::new("r")),
                ],
            ],
        }
    }

    pub fn prompt(&self) -> String {
        loop {
            print!(":");
            io::stdout().flush().expect("error flushing");
            let input = read_line();
            if !input.is_empty() {
                return input;
            } else {
                println!("{}", self.show());
            }
        }
    }

    pub fn show(&self) -> String {
        let mut grid = String::new();
        for y in self.layout.iter() {
            for x in y {
                if let Some(piece) = x {
                    grid.push_str(&format!("{} ", &piece.symbol));
                } else {
                    grid.push_str(". ");
                }
            }
            grid.push_str("\n");
        }
        grid
    }
}
