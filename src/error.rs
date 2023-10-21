use std::fmt::Display;
use std::error::Error;

pub type BoardResult<T> = Result<T, BoardError>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BoardError {
    InvalidXCoord(u8),
    InvalidYCoord(u8),
}

impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidXCoord(c) => write!(f, "Invalid X coordinate: {c}"),
            Self::InvalidYCoord(c) => write!(f, "Invalid Y coordinate: {c}"),
        }
    }
}

impl Error for BoardError {}

