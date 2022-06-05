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

    // check that en passant is possible
    let en_passant: String; 
    if b.en_passant.rank == 0 && b.en_passant.file == '_'  {
        en_passant = String::from("No en passant possible.")
    } else {
        en_passant = format!("{}", b.en_passant)
    }

    let out: String = format!("
{board}\n
Active Player: {active}
Castling Availability:\n{castling}
En Passant Target: {en_passant}
Halfmoves Since Last Pawnpush or Capture: {halfmoves}
No. of Fullmoves: {fullmoves}",
        board=b.board,
        active=b.active_player,
        castling=castle,
        en_passant=en_passant,
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
