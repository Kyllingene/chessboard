use thiserror::Error;

pub type Uci = (u8, u8);

#[derive(Clone, Copy, Debug, Error, PartialEq, Eq, Hash)]
pub enum UciError {
    #[error("expected {0} characters, found {1}")]
    WrongSize(usize, usize),
    #[error("invalid rank: {0}")]
    InvalidRank(char),
    #[error("invalid file: {0}")]
    InvalidFile(char),
}

/// Parse a single UCI coordinate.
#[inline]
pub fn one(s: &str) -> Result<Uci, UciError> {
    if s.len() != 2 {
        return Err(UciError::WrongSize(2, s.len()));
    }

    let mut chars = s.chars();
    let file = chars.next().unwrap().to_ascii_lowercase();
    let rank = chars.next().unwrap();

    let rank = match rank {
        '1' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,

        _ => return Err(UciError::InvalidRank(rank)),
    };

    let file = match file {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,

        _ => return Err(UciError::InvalidFile(file)),
    };

    Ok((file, rank))
}

/// Parse two UCI coordinates, side-by-side or separated by a space.
#[inline]
pub fn two(s: &str) -> Result<(Uci, Uci), UciError> {
    if s.contains(' ') {
        return two(&s.replace(' ', ""));
    }

    if s.len() != 4 {
        return Err(UciError::WrongSize(4, s.len()));
    }

    let a = &s[0..2];
    let b = &s[2..4];

    let a = one(a)?;
    let b = one(b)?;

    Ok((a, b))
}
