
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Color {
    White,
    Black,
    Random,
}

#[derive(Debug)]
pub struct ClockSettings {
    pub limit: i32,
    pub increment: i32,

    pub is_correspondence: bool,
    pub days: i32,
}

#[derive(Debug, Copy, Clone)]
pub enum Piece {
    None,
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

pub struct Board {
    pub side: Color,
    pub turn: Color,

    state: Vec<Vec<Piece>>
}