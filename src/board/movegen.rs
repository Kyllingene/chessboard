use super::Board;
use crate::error::{BoardError, BoardResult};
use crate::{bit, coverage, helper, piece};

use pfen::Color;

impl Board {
    /// Returns all the squares that color is attacking.
    pub fn coverage(&self, color: Color) -> u64 {
        let (mine, other) = match color {
            Color::White => (self.white, self.black),
            Color::Black => (self.black, self.white),
        };

        let pawn: fn(u64, u64) -> u64 = match color {
            Color::White => piece::pawn_up,
            Color::Black => piece::pawn_down,
        };

        coverage!(
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

            mine,
            other,
        )
    }

    pub fn in_check(&self, color: Color) -> bool {
        let coverage = self.coverage(!color);
        let king = match color {
            Color::White => self.white,
            Color::Black => self.black,
        } & self.kings;

        coverage & king != 0
    }

    /// Makes a move with few checks, returning a new board.
    pub fn future(mut self, from: (u8, u8), to: (u8, u8)) -> BoardResult<Self> {
        let (x, y) = from;
        let piece = self.get(x, y)?;
        self.clear(bit::xy(x, y));

        let Some(piece) = piece else {
            return Err(BoardError::SquareIsEmpty(x, y));
        };

        let (x, y) = to;
        let target = self.get(x, y)?;

        if let Some(target) = target {
            if piece.color == target.color {
                return Err(BoardError::SquareIsOccupied(x, y));
            }
        }

        self.set(x, y, Some(piece))?;
        Ok(self)
    }

    pub fn legal(&self, from: (u8, u8), to: (u8, u8)) -> bool {
        let (fx, fy) = from;
        let Ok(piece) = self.get(fx, fy) else {
            return false;
        };

        let Some(piece) = piece else {
            return false;
        };

        let (mine, other) = match piece.color {
            Color::White => (self.white, self.black),
            Color::Black => (self.black, self.white),
        };

        let (tx, ty) = to;
        let Ok(target) = self.get(tx, ty) else {
            return false;
        };

        if let Some(target) = target {
            if piece.color == target.color {
                return false;
            }
        }

        let Ok(future) = self.future(from, to) else {
            return false;
        };

        if future.in_check(piece.color) {
            return false;
        }

        let moves = piece::piece(fx, fy, piece, mine | other) & !mine;
        if !helper::bitboard_to_coords_contains(moves, tx, ty) {
            return false;
        }

        // TODO: castling

        true
    }
}
