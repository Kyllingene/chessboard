use std::fmt::Display;

use crate::error::{BoardError, BoardResult};
use crate::{not_mine, bit, piece};

#[inline]
fn check_xy(x: u8, y: u8) -> BoardResult<()> {
    if x > 7 {
        Err(BoardError::InvalidXCoord(x))
    } else if y > 7 {
        Err(BoardError::InvalidYCoord(y))
    } else {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Color {
    #[default]
    White,
    Black,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Piece {
    #[default]
    None,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Board {
    white: u64,
    black: u64,

    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    queens: u64,
    kings: u64,

    turn: Color,

    // (square to move to, piece you would take)
    en_passant_target: Option<(u8, u8)>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            white: 0x000000000000ffff,
            black: 0xffff000000000000,

            pawns: 0x00ff00000000ff00,
            knights: 0x4200000000000042,
            bishops: 0x2400000000000024,
            rooks: 0x8100000000000081,
            queens: 0x1000000000000010,
            kings: 0x0800000000000008,

            ..Default::default()
        }
    }

    fn clear(&mut self, bit: u64) {
        self.white &= !bit;
        self.black &= !bit;

        self.pawns &= !bit;
        self.knights &= !bit;
        self.bishops &= !bit;
        self.rooks &= !bit;
        self.queens &= !bit;
        self.kings &= !bit;
    }

    pub fn get(&self, x: u8, y: u8) -> BoardResult<(Piece, Color)> {
        check_xy(x, y)?;

        let color = if bit::get(self.black, x, y) {
            Color::Black
        } else {
            Color::White
        };

        let piece = if bit::get(self.pawns, x, y) {
            Piece::Pawn
        } else if bit::get(self.knights, x, y) {
            Piece::Knight
        } else if bit::get(self.bishops, x, y) {
            Piece::Bishop
        } else if bit::get(self.rooks, x, y) {
            Piece::Rook
        } else if bit::get(self.queens, x, y) {
            Piece::Queen
        } else if bit::get(self.kings, x, y) {
            Piece::King
        } else {
            Piece::None
        };

        Ok((piece, color))
    }

    pub fn set(&mut self, x: u8, y: u8, piece: Piece, color: Color) -> BoardResult<()> {
        check_xy(x, y)?;

        let bit = bit::xy(x, y);
        self.clear(bit);
        match piece {
            Piece::None => {},
            Piece::Pawn => self.pawns |= bit,
            Piece::Knight => self.knights |= bit,
            Piece::Bishop => self.bishops |= bit,
            Piece::Rook => self.rooks |= bit,
            Piece::Queen => self.queens |= bit,
            Piece::King => self.kings |= bit,
        }

        match color {
            Color::White => self.white |= bit,
            Color::Black => self.black |= bit,
        }

        Ok(())
    }

    /// Returns all the squares that color is attacking.
    pub fn coverage(&self, color: Color) -> u64 {
        let mine = match color {
            Color::White => self.white,
            Color::Black => self.black,
        };

        let pawn: fn(u64, u64) -> u64 = match color {
            Color::White => piece::pawn_up,
            Color::Black => piece::pawn_down,
        };

        not_mine!(
            pawn,
            piece::knight,
            piece::bishop,
            piece::rook,
            piece::queen,
            piece::king =>
            self.pawns,
            self.knights,
            self.bishops,
            self.rooks,
            self.queens,
            self.kings;

            self.white | self.black,
            mine
        )
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!();
        for y in (0..8).rev() {
            for x in (0..8).rev() {
                let (piece, color) = self.get(x, y).unwrap();

                let mut c = match piece {
                    Piece::None => ' ',
                    Piece::Pawn => 'p',
                    Piece::Knight => 'n',
                    Piece::Bishop => 'b',
                    Piece::Rook => 'r',
                    Piece::Queen => 'q',
                    Piece::King => 'k',
                };

                if color == Color::White {
                    c = c.to_ascii_uppercase();
                }

                write!(f, " {c}")?;

                if x != 0 {
                    write!(f, " |")?;
                }
            }

            if y != 0 {
                writeln!(f, "\n---+---+---+---+---+---+---+---")?;
            }
        }

        writeln!(f)?;

        Ok(())
    }
}

