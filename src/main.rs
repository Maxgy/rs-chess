use rs_chess::board::Board;

fn main() {
    let board = Board::new();

    println!("{}", board.show());
    board.prompt();
}
