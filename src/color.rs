#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
    Random,
    Invalid,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White   => write!(f, "White"),
            Color::Black   => write!(f, "Black"),
            Color::Random  => write!(f, "Random"),
            Color::Invalid => write!(f, "Invalid"),
        }
    }
}

impl std::ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::White   => Color::Black,
            Color::Black   => Color::White,
            Color::Random  => Color::Random,
            Color::Invalid => Color::Invalid,
        }
    }
}

impl std::convert::From<i32> for Color {
    fn from(item: i32) -> Self {
        match item {
            -1 => Color::Random,
            0 => Color::Black,
            1 => Color::White,
            _ => Color::Invalid,
        }
    }
}

impl std::convert::From<Color> for i32 {
    fn from(c: Color) -> i32 {
        match c {
            Color::Random => -1,
            Color::Invalid => -1,
            Color::White => 0,
            Color::Black => 1,
        }
    }
}

impl Color {
    pub fn unwrap(self) -> Color {
        match self {
            Color::Invalid => panic!("Invalid color (Color::Invalid)"),
            _ => self,
        }
    }

    pub fn turn(&self) -> i32 {
        (*self).into()
    }
}