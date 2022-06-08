mod chess;

fn main() {
    let fen = std::env::args().nth(1).expect("FEN string expected");
    let board = chess::ChessState::new(&fen);

    // Gross? but println! only accepts literals OR doing it like this
    println!("{}", board);
}
