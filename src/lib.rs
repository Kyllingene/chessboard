#![allow(unused)]
// 0b0101010100110011000011110000000011110000110011001010101011111111
// =>
// 0 1 0 1 0 1 0 1
// 0 0 1 1 0 0 1 1
// 0 0 0 0 1 1 1 1
// 0 0 0 0 0 0 0 0
// 1 1 1 1 0 0 0 0
// 1 1 0 0 1 1 0 0
// 1 0 1 0 1 0 1 0
// 1 1 1 1 1 1 1 1

mod bit;
mod board;
mod error;
mod helper;
mod mask;
mod piece;
mod shift;
mod slide;
mod uci;

pub use pfen::{Castling, Color, Piece, PieceKind};

#[cfg(test)]
mod test;
