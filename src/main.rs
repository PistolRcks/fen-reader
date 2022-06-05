mod chess;

// outputs a prettyprinted board and information from a ChessState object
fn state_to_str(b: &chess::ChessState) -> String {
    let mut castle: String = String::new();
    
    // Format castling output via binary and
    if b.castling_avail != 0 {
        if b.castling_avail & chess::CastlingAvail::WhiteKingside != 0 {
            castle += "\tWhite, Kingside\n"; 
        }
        if b.castling_avail & chess::CastlingAvail::WhiteQueenside != 0 {
            castle += "\tWhite, Queenside\n";
        }
        if b.castling_avail & chess::CastlingAvail::BlackKingside != 0 {
            castle += "\tBlack, Kingside\n"; 
        }
        if b.castling_avail & chess::CastlingAvail::BlackQueenside != 0 {
            castle += "\tBlack, Queenside\n";
        }
    } else {
        castle += "\tNo castling available.\n";
    }


    let out: String = format!("
        {board}\n\n
        Active Player: {active}\n
        Castling Availability:\n{castling}
        En Passant Target: {en_passant}\n
        Halfmoves Since Last Pawnpush or Capture: {halfmoves}\n
        No. of Fullmoves: {fullmoves}",
        board=b.board,
        active=b.active_player,
        castling=b.castling_avail,
        en_passant=b.en_passant,
        halfmoves=b.halfmove_clock,
        fullmoves=b.fullmoves
    );

    return out
}

fn main() {
    let fen = std::env::args().nth(1).expect("FEN string expected");
    let board = chess::ChessState::new(&fen);

    // Gross? but println! only accepts literals OR doing it like this
    println!("{}", state_to_str(&board));
}
