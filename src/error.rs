use std::error::Error;

pub type ChessResult<T> = Result<T, ChessErr>;

#[derive(Debug, PartialEq, Clone)]
pub enum ChessErr {
    WrongTurn,
    SourceEmpty,
    CantCaptureOwnPiece, // TODO: better naming here
    SourceIsDestination,
    InCheck,
    CantMoveIntoCheck,

    InvalidUCI(String),
    InvalidCoords(String),
    // TODO: more coverage

    CantCastle(String),
    InvalidMove(String),

    Internal(String),
}

impl Error for ChessErr {}

impl std::fmt::Display for ChessErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChessErr::WrongTurn => write!(f, "It isn't your turn"),
            ChessErr::SourceEmpty => write!(f, "You cannot move an empty square"),
            ChessErr::CantCaptureOwnPiece => write!(f, "You cannot capture your own piece"),
            ChessErr::SourceIsDestination => write!(f, "You cannot move a piece into itself"),
            ChessErr::InCheck => write!(f, "You must get out of check"),
            ChessErr::CantMoveIntoCheck => write!(f, "You cannot move into check"),

            ChessErr::InvalidUCI(e) => write!(f, "Invalid UCI ({})", e),
            ChessErr::InvalidCoords(e) => write!(f, "Invalid coordinates ({})", e),

            ChessErr::CantCastle(e) => write!(f, "You cannot castle in that direction ({})", e),
            ChessErr::InvalidMove(e) => write!(f, "That is an invalid move ({})", e),

            ChessErr::Internal(e) => write!(f, "An internal error occurred: {}", e),
        }
    }
}

impl<T> std::convert::From<ChessErr> for ChessResult<T> {
    fn from(e: ChessErr) -> Self {
        Err(e)
    }
}