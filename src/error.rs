use thiserror::Error;

pub type BoardResult<T> = Result<T, BoardError>;

#[derive(Clone, Copy, Debug, Error, PartialEq, Eq, Hash)]
pub enum BoardError {
    #[error("invalid X coordinate: {0}")]
    InvalidXCoord(u8),
    #[error("invalid Y coordinate: {0}")]
    InvalidYCoord(u8),

    #[error("square {0}, {1} is empty and cannot be moved")]
    SquareIsEmpty(u8, u8),
    #[error("square {0}, {1} is already occupied by a piece of the same color")]
    SquareIsOccupied(u8, u8),
}
