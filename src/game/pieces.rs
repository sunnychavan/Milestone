use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    Black,
    White,
}

impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            Piece::White => "⚪",
            Piece::Black => "⚫",
        };

        write!(f, "{}", val)
    }
}
