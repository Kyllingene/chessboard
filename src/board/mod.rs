use std::fmt::Display;

use pfen::PfenError;

use crate::error::{BoardError, BoardResult};
use crate::uci::{self, UciError};
use crate::{bit, piece};
use crate::{Castling, Color, Piece, PieceKind};

mod movegen;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
    castling: Castling,

    // (square to move to, piece you would take)
    en_passant_target: Option<(u8, u8)>,

    halfmove: u32,
    fullmove: u32,
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

    pub fn white(&self) -> u64 {
        self.white
    }

    pub fn black(&self) -> u64 {
        self.black
    }

    pub fn pawns(&self) -> u64 {
        self.pawns
    }
    pub fn knights(&self) -> u64 {
        self.knights
    }
    pub fn bishops(&self) -> u64 {
        self.bishops
    }
    pub fn rooks(&self) -> u64 {
        self.rooks
    }
    pub fn queens(&self) -> u64 {
        self.queens
    }
    pub fn kings(&self) -> u64 {
        self.kings
    }
    pub fn turn(&self) -> Color {
        self.turn
    }
    pub fn castling(&self) -> Castling {
        self.castling
    }
    pub fn en_passant_target(&self) -> Option<(u8, u8)> {
        self.en_passant_target
    }
    pub fn halfmove(&self) -> u32 {
        self.halfmove
    }
    pub fn fullmove(&self) -> u32 {
        self.fullmove
    }

    pub fn empty() -> Self {
        Self::default()
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

    pub fn get(&self, x: u8, y: u8) -> BoardResult<Option<Piece>> {
        check_xy(x, y)?;

        let color = if bit::get(self.black, x, y) {
            Color::Black
        } else {
            Color::White
        };

        let kind = if bit::get(self.pawns, x, y) {
            Some(PieceKind::Pawn)
        } else if bit::get(self.knights, x, y) {
            Some(PieceKind::Knight)
        } else if bit::get(self.bishops, x, y) {
            Some(PieceKind::Bishop)
        } else if bit::get(self.rooks, x, y) {
            Some(PieceKind::Rook)
        } else if bit::get(self.queens, x, y) {
            Some(PieceKind::Queen)
        } else if bit::get(self.kings, x, y) {
            Some(PieceKind::King)
        } else {
            None
        };

        Ok(kind.map(|kind| Piece { kind, color }))
    }

    pub fn set(&mut self, x: u8, y: u8, piece: Option<Piece>) -> BoardResult<()> {
        check_xy(x, y)?;

        let bit = bit::xy(x, y);
        self.clear(bit);

        if let Some(Piece { kind, color }) = piece {
            match kind {
                PieceKind::Pawn => self.pawns |= bit,
                PieceKind::Knight => self.knights |= bit,
                PieceKind::Bishop => self.bishops |= bit,
                PieceKind::Rook => self.rooks |= bit,
                PieceKind::Queen => self.queens |= bit,
                PieceKind::King => self.kings |= bit,
            }

            match color {
                Color::White => self.white |= bit,
                Color::Black => self.black |= bit,
            }
        }

        Ok(())
    }

    pub fn get_uci(&self, uci: &str) -> Result<Option<Piece>, UciError> {
        let (x, y) = uci::one(uci)?;
        Ok(self.get(x, y).unwrap())
    }

    pub fn set_uci(&mut self, uci: &str, piece: Option<Piece>) -> Result<(), UciError> {
        let (x, y) = uci::one(uci)?;
        self.set(x, y, piece).unwrap();
        Ok(())
    }

    pub fn fen(fen: &str) -> Result<Self, PfenError> {
        let fen = pfen::parse(fen)?;
        let mut board = Self::empty();

        for (y, row) in fen.board.iter().enumerate() {
            for (x, piece) in row.iter().enumerate() {
                board.set(x as u8, y as u8, *piece);
            }
        }

        board.turn = fen.turn;
        board.castling = fen.castling;
        board.en_passant_target = fen.en_passant;
        board.halfmove = fen.halfmove;
        board.fullmove = fen.fullmove;

        Ok(board)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for y in (0..8).rev() {
            for x in (0..8).rev() {
                let c = if let Some(Piece { kind, color }) = self.get(x, y).unwrap() {
                    let mut c = match kind {
                        PieceKind::Pawn => 'p',
                        PieceKind::Knight => 'n',
                        PieceKind::Bishop => 'b',
                        PieceKind::Rook => 'r',
                        PieceKind::Queen => 'q',
                        PieceKind::King => 'k',
                    };

                    if color == Color::White {
                        c = c.to_ascii_uppercase();
                    }

                    c
                } else {
                    ' '
                };

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

impl Default for Board {
    fn default() -> Self {
        Self {
            white: 0,
            black: 0,

            pawns: 0,
            knights: 0,
            bishops: 0,
            rooks: 0,
            queens: 0,
            kings: 0,

            castling: Castling {
                black_kingside: true,
                white_kingside: true,
                black_queenside: true,
                white_queenside: true,
            },
            turn: Color::White,
            en_passant_target: None,

            halfmove: 0,
            fullmove: 1,
        }
    }
}
