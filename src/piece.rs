use crate::color::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Piece {
    None,
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Piece::None => {write!(f, " ")},
            Piece::Pawn(c) => {
                if c == Color::White {
                    write!(f, "P")
                } else {
                    write!(f, "p")
                }
            },
            Piece::Knight(c) => {
                if c == Color::White {
                    write!(f, "N")
                } else {
                    write!(f, "n")
                }
            },
            Piece::Bishop(c) => {
                if c == Color::White {
                    write!(f, "B")
                } else {
                    write!(f, "b")
                }
            },
            Piece::Rook(c) => {
                if c == Color::White {
                    write!(f, "R")
                } else {
                    write!(f, "r")
                }
            },
            Piece::Queen(c) => {
                if c == Color::White {
                    write!(f, "Q")
                } else {
                    write!(f, "q")
                }
            },
            Piece::King(c) => {
                if c == Color::White {
                    write!(f, "K")
                } else {
                    write!(f, "k")
                }
            },
        }
    }
}
