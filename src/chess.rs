use regex::{Regex, Captures};
use std::{fmt, ops::BitAnd};

pub struct Position {
    pub rank: u8,
    pub file: char
}

// Note to self: basically the to_string of Rust
// (Rust actually has a ToString trait, but Display implements ToString)
impl fmt::Display for Position {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{}{}", self.file, self.rank)
   } 
}

pub enum Player {
    WHITE,
    BLACK
}

impl fmt::Display for Player {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
           Player::WHITE => write!(f, "White"),
           Player::BLACK => write!(f, "Black")
       }
   } 
}

#[repr(u8)]
pub enum CastlingAvail {
    WhiteKingside = 1,
    WhiteQueenside = 2,
    BlackKingside = 4,
    BlackQueenside = 8
}

// Makes it easier to bitwise and to find which castling moves are available
impl BitAnd<CastlingAvail> for u8 {
    fn bitand(self, rhs: CastlingAvail) -> u8 {
        self & (rhs as u8)
    }
    type Output = u8;
}


pub struct ChessState {
    pub board: String,
    pub active_player: Player,
    pub castling_avail: u8,
    pub en_passant: Position,
    pub halfmove_clock: u8,
    pub fullmoves: u8
}

impl ChessState {
    pub fn new(fen: &String) -> ChessState {
        let mut new_board: String = String::new();

        // step 0: regex to split the sections up
        let re: Regex = Regex::new(r"[\w/-]+").unwrap();
        // still not understanding the whole borrow thing, I guess it's due to being immutable?
        let sections: Vec<String> = re.captures_iter(&fen)
            .map(|x: Captures| String::from(x.get(0).unwrap().as_str())) // Also map into string type
            .collect();

        // step 1: get boardstate
        for c in sections[0].chars() {
            if c == '/' {
               new_board.push('\n');
            } else if c.is_numeric() {
               // Really wierd thing? Doesn't compile unless the borrow is there
               // Also `to_digit` gives an Option, from which you must `unwrap` the 
               // `Some` object. (also necessary typecast, potentially unsafe?)
               new_board.push_str(&".".repeat(c.to_digit(10).unwrap() as usize));
            // break if we get to the options, which should have a space afterwards
            } else if c == ' ' {
               break;
            } else {
               new_board.push(c);
            }
        }

        // step 2: get active color
        let player: Player;
        if sections[1] == "w" {
            player = Player::WHITE
        } else {
            player = Player::BLACK
        }

        // step 3: castling (i'm just gonna hardcode it)
        let mut castling: u8 = 0;
        // add (similar to bitwise or) for the available sides
        if sections[2] != "-" {
            for c in sections[2].chars() {
                match c {
                    'K' => castling += CastlingAvail::WhiteKingside as u8,
                    'Q' => castling += CastlingAvail::WhiteQueenside as u8,
                    'k' => castling += CastlingAvail::BlackKingside as u8,
                    'q' => castling += CastlingAvail::BlackQueenside as u8,
                    _ => castling += 0  // wildcard required
                }
            }
        }

        // step 4: en passant target
        let ep_target: Position;
        if sections[3] == "-" {
            ep_target = Position{rank: 0, file: '_'}
        } else {
            let chars: Vec<char> = sections[3].chars().collect();
            ep_target = Position{
                // gotta figure out how unwraps work, this seems superfluous
                rank: chars[1].to_digit(10).unwrap() as u8, 
                file: chars[0]
            };
        }

        // step 5: halfmoves since last pawnpush or capture
        let halfmove: u8 = sections[4].parse::<u8>().unwrap();

        let fullmove: u8 = sections[5].parse::<u8>().unwrap();

        return ChessState {
            board: new_board,
            active_player: player,
            castling_avail: castling,
            en_passant: ep_target,
            halfmove_clock: halfmove,
            fullmoves: fullmove
        }
    }

    // Format castling availability to look nicer
    pub fn castling_to_str(&self) -> String {
        let mut out: String = String::new();
        
        if self.castling_avail != 0 {
            if self.castling_avail & CastlingAvail::WhiteKingside != 0 {
                out += "White, Kingside; "; 
            }
            if self.castling_avail & CastlingAvail::WhiteQueenside != 0 {
                out += "White, Queenside; ";
            }
            if self.castling_avail & CastlingAvail::BlackKingside != 0 {
                out += "Black, Kingside; "; 
            }
            if self.castling_avail & CastlingAvail::BlackQueenside != 0 {
                out += "Black, Queenside";
            }
        } else {
            out += "None";
        }

        out
    }
}

impl fmt::Display for ChessState {
    // outputs a prettyprinted board and information from a ChessState object
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // check that en passant is possible
        let en_passant: String; 
        if self.en_passant.rank == 0 && self.en_passant.file == '_'  {
            en_passant = String::from("No en passant possible.")
        } else {
            en_passant = format!("{}", self.en_passant)
        }

        write!(f, "
{board}\n
Active Player: {active}
Castling Availability: {castling}
En Passant Target: {en_passant}
Halfmoves Since Last Pawnpush or Capture: {halfmoves}
No. of Fullmoves: {fullmoves}",
            board=self.board,
            active=self.active_player,
            castling=self.castling_to_str(),
            en_passant=en_passant,
            halfmoves=self.halfmove_clock,
            fullmoves=self.fullmoves
        )
    }
}
