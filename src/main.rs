use regex::{Regex, Captures};

// outputs a prettyprinted board from a fen string
fn fen_to_str(fen : String) -> String {
    let mut board : String = String::new();

    // step 0: regex to split the sections up
    let re : Regex = Regex::new(r"[\w/-]+").unwrap();
    // still not understanding the whole borrow thing, I guess it's due to being immutable?
    let sections : Vec<&str> = re.captures_iter(&fen)
        .map(|x: Captures| x.get(0).unwrap().as_str()) // Also map into string type
        .collect();

    for sec in &sections {
        dbg!(sec);
    }

    // step 1: get boardstate
    for c in sections[0].chars() {
        if c == '/' {
           board.push('\n');
        } else if c.is_numeric() {
           // Really wierd thing? Doesn't compile unless the borrow is there
           // Also `to_digit` gives an Option, from which you must `unwrap` the 
           // `Some` object. (also necessary typecast, potentially unsafe?)
           board.push_str(&".".repeat(c.to_digit(10).unwrap() as usize));
        // break if we get to the options, which should have a space afterwards
        } else if c == ' ' {
           break;
        } else {
           board.push(c);
        }
    }

    // step 2: get active color
    board += "\n\nActive Color: ";
    if sections[1] == "w" {
        board += "White"
    } else {
        board += "Black"
    }

    // step 3: castling (i'm just gonna hardcode it)
    board += "\nCastling Availability:";
    if sections[2].starts_with("KQ") {
        board += "\n\tWhite can castle king- or queenside."
    } else if sections[2].starts_with("K") {
        board += "\n\tWhite can only castle kingside."
    } else if sections[2].starts_with("Q") {
        board += "\n\tWhite can only castle queenside."
    }

    if sections[2].ends_with("kq") {
        board += "\n\tBlack can castle king- or queenside."
    } else if sections[2].ends_with("k") {
        board += "\n\tBlack can only castle kingside."
    } else if sections[2].ends_with("q") {
        board += "\n\tBlack can only castle queenside."
    }
    
    if sections[2] == "-" {
        board += " Neither side can castle."
    }

    // step 4: en passant target
    board += "\nEn Passant Target: ";
    if sections[3] == "-" {
        board += "None"
    } else {
        board += sections[3]
    }

    // step 5: halfmove clock (time since last pawnpush or capture)
    board += "\nNo. of Halfmoves Since Last Pawnpush or Capture: ";
    board += sections[4];

    // step 6: fullmove clock (counts a move from white and one from black as one move)
    board += "\nNo. of Fullmoves: ";
    board += sections[5];

    return board;
}

fn main() {
    let fen = std::env::args().nth(1).expect("FEN string expected");

    // Gross? but println! only accepts literals OR doing it like this
    println!("{}", fen_to_str(fen));
}
