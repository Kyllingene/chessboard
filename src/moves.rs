use crate::error::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Square {
    pub rank: Rank,
    pub file: File,
}

impl Square {
    /// Converts algebraic coordinates to board indices; if the coords are invalid, and error will be returned detailing the problem
    pub fn coords_to_indices(m: String) -> ChessResult<Square> {
        if m.len() != 2 {
            return ChessErr::InvalidCoords(format!("Coordinates must be 2 characters, were {}", m.len())).into();
        }

        let first = m.chars().collect::<Vec<char>>()[0];
        let second = m.chars().collect::<Vec<char>>()[1];

        if !"abcdefgh".contains(first) {
            return ChessErr::InvalidCoords(format!("First character must be in the range `a-z`, was {}", first)).into();
        }

        if !"12345678".contains(second) {
            return ChessErr::InvalidCoords(format!("Second character must be in the range `1-8`, was {}", second)).into();
        }

        Ok(Square{file: ((first as usize) - 97).into(), rank: (String::from(second).parse::<usize>().unwrap() - 1).into()})
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl std::convert::From<usize> for Rank {
    fn from(item: usize) -> Self {
        match item {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eighth,
            _ => panic!("Invalid rank: {}", item),
        }
    }
}

impl std::convert::From<Rank> for usize {
    fn from(r: Rank) -> usize {
        match r {
            Rank::First   => 0,
            Rank::Second  => 1,
            Rank::Third   => 2,
            Rank::Fourth  => 3,
            Rank::Fifth   => 4,
            Rank::Sixth   => 5,
            Rank::Seventh => 6,
            Rank::Eighth  => 7,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl std::convert::From<usize> for File {
    fn from(item: usize) -> Self {
        match item {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => panic!("Invalid file: {}", item),
        }
    }
}

impl std::convert::From<File> for usize {
    fn from(f: File) -> usize {
        match f {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Move {
    pub src: Square,
    pub dst: Square,
}

impl Move {
    pub fn uci(uci: String) -> ChessResult<Self> {
        if uci.len() != 4 {
            return ChessErr::InvalidUCI(format!("UCI format uses 4 characters, recieved {}", uci.len())).into();
        }

        let chars = uci.chars().collect::<Vec<char>>();
        let src = Square::coords_to_indices(format!("{}{}", chars[0], chars[1]))?;
        let dst = Square::coords_to_indices(format!("{}{}", chars[2], chars[3]))?;

        Ok(Move{src, dst})
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Castle {
    None,
    Queenside,
    Kingside,
    Both,
}

impl Castle {
    pub fn has(&self, rights: Castle) -> bool {
        match rights {
            Castle::None => true,
            Castle::Both => self == &Castle::Both,
            Castle::Queenside => self == &Castle::Queenside || self == &Castle::Both,
            Castle::Kingside => self == &Castle::Kingside || self == &Castle::Both,
        }
    }

    pub fn remove(&self, rights: Castle) -> Castle {
        match rights {
            Castle::None => *self,
            Castle::Both => Castle::None,
            Castle::Queenside => {
                match self {
                    Castle::Both => Castle::Kingside,
                    Castle::Queenside => Castle::Queenside,
                    _ => *self,
                }
            },
            Castle::Kingside => {
                match self {
                    Castle::Both => Castle::Queenside,
                    Castle::Kingside => Castle::Kingside,
                    _ => *self,
                }
            },
        }
    }

    pub fn add(&self, rights: Castle) -> Castle {
        match rights {
            Castle::None => *self,
            Castle::Both => Castle::Both,
            Castle::Queenside => {
                match self {
                    Castle::None => Castle::Queenside,
                    Castle::Kingside => Castle::Both,
                    _ => *self,
                }
            },
            Castle::Kingside => {
                match self {
                    Castle::None => Castle::Kingside,
                    Castle::Queenside => Castle::Both,
                    _ => *self,
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Castling {
    pub white: Castle,
    pub black: Castle,
}
