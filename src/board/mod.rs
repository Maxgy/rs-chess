use std::io;
use std::io::Write;

use crate::piece::{Piece, Playable};

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
                vec!['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
                vec!['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
                vec!['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
            ]
            .iter()
            .map(|v| {
                v.iter()
                    .map(|&c| if c == '.' { None } else { Some(Piece::new(c)) })
                    .collect()
            })
            .collect(),
        }
    }

    pub fn play(&self) {
        loop {
            println!("{}", self.show());

            self.prompt();
        }
    }

    fn prompt(&self) -> String {
        loop {
            print!("Move : ");
            io::stdout().flush().expect("error flushing");

            let input = read_line();
            if !input.is_empty() {
                return input;
            } else {
                println!("{}", self.show());
            }
        }
    }

    fn show(&self) -> String {
        let mut grid = String::new();
        for y in self.layout.iter() {
            for x in y {
                if let Some(piece) = x {
                    grid.push_str(&format!("{} ", piece.symbol()));
                } else {
                    grid.push_str(". ");
                }
            }
            grid.push_str("\n");
        }
        grid
    }
}
