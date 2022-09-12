#[warn(clippy::all)]

pub type Square = [usize; 2];

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CanCastle {
    None,
    Queenside,
    Kingside,
    Both,
}

#[derive(Debug, Clone, Copy)]
pub struct Castling {
    white: CanCastle,
    black: CanCastle,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
    Random,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White  => write!(f, "White"),
            Color::Black  => write!(f, "Black"),
            Color::Random => write!(f, "Random"),
        }
    }
}

#[derive(Debug)]
pub struct ClockSettings {
    pub limit: i32,
    pub increment: i32,

    pub is_correspondence: bool,
    pub days: i32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Piece {
    None,
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Piece::None => {write!(f, " ")},
            Piece::Pawn(c) => {
                if c == Color::White {
                    write!(f, "P")
                } else {
                    write!(f, "p")
                }
            },
            Piece::Knight(c) => {
                if c == Color::White {
                    write!(f, "N")
                } else {
                    write!(f, "n")
                }
            },
            Piece::Bishop(c) => {
                if c == Color::White {
                    write!(f, "B")
                } else {
                    write!(f, "b")
                }
            },
            Piece::Rook(c) => {
                if c == Color::White {
                    write!(f, "R")
                } else {
                    write!(f, "r")
                }
            },
            Piece::Queen(c) => {
                if c == Color::White {
                    write!(f, "Q")
                } else {
                    write!(f, "q")
                }
            },
            Piece::King(c) => {
                if c == Color::White {
                    write!(f, "K")
                } else {
                    write!(f, "k")
                }
            },
        }
    }
}

pub struct Board {
    pub side: Color,
    pub turn: Color,

    pub halfmoves: i32,
    pub fullmoves: i32,

    pub castling: Castling,

    pub en_passant: Option<Square>,

    state: Vec<Vec<Piece>>,
}

impl Board {
    // TODO: many things
    // first: make the fen parsing better
    // second: add error handling
    // third: refactor everything to be better

    /// Creates a board with an optional fen string
    pub fn new(fen: Option<String>, side: Color) -> Board {
        let initial = match fen {
            None => String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            Some(f) => f,
        };

        let mut board: Vec<Vec<Piece>> = vec![
            vec![Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None],
            vec![Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None],
            vec![Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None],
            vec![Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None],
            vec![Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None],
            vec![Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None],
            vec![Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None],
            vec![Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None, Piece::None],
        ];
        let mut turn = Color::White;
        let mut castling = Castling{
            white: CanCastle::Both,
            black: CanCastle::Both,
        };
        let mut en_passant = String::new();
        let mut halfmoves = String::new();
        let mut fullmoves = String::new();

        let mut spaces = 0;

        let mut col = 0;
        let mut row = 0;
        for char in initial.chars() {
            if spaces == 0 {
                match char {
                    '/' => {
                        col = 0;
                        row += 1;
                    },

                    'r' => {
                        board[row][col] = Piece::Rook(Color::Black);
                        col += 1;
                    },
                    'R' => {
                        board[row][col] = Piece::Rook(Color::White);
                        col += 1;
                    },
                    'n' => {
                        board[row][col] = Piece::Knight(Color::Black);
                        col += 1;
                    },
                    'N' => {
                        board[row][col] = Piece::Knight(Color::White);
                        col += 1;
                    },
                    'b' => {
                        board[row][col] = Piece::Bishop(Color::Black);
                        col += 1;
                    },
                    'B' => {
                        board[row][col] = Piece::Bishop(Color::White);
                        col += 1;
                    },
                    'q' => {
                        board[row][col] = Piece::Queen(Color::Black);
                        col += 1;
                    },
                    'Q' => {
                        board[row][col] = Piece::Queen(Color::White);
                        col += 1;
                    },
                    'k' => {
                        board[row][col] = Piece::King(Color::Black);
                        col += 1;
                    },
                    'K' => {
                        board[row][col] = Piece::King(Color::White);
                        col += 1;
                    },
                    'p' => {
                        board[row][col] = Piece::Pawn(Color::Black);
                        col += 1;
                    },
                    'P' => {
                        board[row][col] = Piece::Pawn(Color::White);
                        col += 1;
                    },

                    '1' => {
                        col += 1;
                    },
                    '2' => {
                        col += 2;
                    },
                    '3' => {
                        col += 3;
                    },
                    '4' => {
                        col += 4;
                    },
                    '5' => {
                        col += 5;
                    },
                    '6' => {
                        col += 6;
                    },
                    '7' => {
                        col += 7;
                    },
                    '8' => {
                        col += 8;
                    },

                    ' ' => {
                        spaces += 1;
                    }

                    _ => {},
                };
            } else if spaces == 1 {
                match char {
                    'w' => turn = Color::White,
                    'b' => turn = Color::Black,
                    ' ' => spaces += 1,
                    _ => {},
                }
            } else if spaces == 2 {
                match char {
                    '-' => castling = Castling{white: CanCastle::None, black: CanCastle::None},
                    'K' => {
                        if castling.white == CanCastle::None {
                            castling.white = CanCastle::Kingside;
                        } else {
                            castling.white = CanCastle::Both;
                        }
                    },
                    'k' => {
                        if castling.black == CanCastle::None {
                            castling.black = CanCastle::Kingside;
                        } else {
                            castling.black = CanCastle::Both;
                        }
                    },
                    'Q' => {
                        if castling.white == CanCastle::None {
                            castling.white = CanCastle::Queenside;
                        } else {
                            castling.white = CanCastle::Both;
                        }
                    },
                    'q' => {
                        if castling.black == CanCastle::None {
                            castling.black = CanCastle::Queenside;
                        } else {
                            castling.black = CanCastle::Both;
                        }
                    },
                    ' ' => spaces += 1,
                    _ => {},
                }
            } else if spaces == 3 {
                if char == '-' {
                    en_passant = String::new();
                } else if char == ' ' {
                    spaces += 1;
                } else {
                    en_passant.push(char);
                }
            } else if spaces == 4 {
                if char == ' ' {
                    spaces += 1;
                } else {
                    halfmoves.push_str(format!("{}", char).as_str())
                }
            } else if spaces == 5 {
                if char == ' ' {
                    spaces += 1;
                } else {
                    fullmoves.push_str(format!("{}", char).as_str())
                }
            }
        }

        let e = if en_passant != "" {
            Some(Board::coords_to_indices(en_passant).unwrap())
        } else {
            None
        };

        Board {
            side: side,
            turn: turn,
            halfmoves: halfmoves.parse().unwrap(),
            fullmoves: fullmoves.parse().unwrap(),
            castling: castling,
            en_passant: e,
            state: board,
        }
    }

    /// Gets a piece from the board
    pub fn get(&self, square: Square) -> Piece {
        self.state[square[0]][square[1]]
    }

    /// Sets a piece on the board
    pub fn set(&mut self, square: Square, piece: Piece) {
        self.state[square[0]][square[1]] = piece;
    }

    /// Makes a move; if the move is invalid, an error will be returned detailing the problem
    pub fn make_move(&mut self, src: Square, dst: Square) -> Result<(), String> {

        if src == dst {
            return Err("Cannot move a piece into itself".to_string());
        }

        let src_color = match self.get(src) {
            Piece::None => return Err("Cannot move an empty square".to_string()),
            Piece::Pawn(c) => c,
            Piece::Knight(c) => c,
            Piece::Rook(c) => c,
            Piece::Bishop(c) => c,
            Piece::Queen(c) => c,
            Piece::King(c) => c,
        };

        let dst_color = match self.get(dst) {
            Piece::None => {
                match src_color {
                    Color::Random => Color::White,
                    Color::White => Color::Black,
                    Color::Black => Color::White,
                }
            },
            Piece::Pawn(c) => c,
            Piece::Knight(c) => c,
            Piece::Rook(c) => c,
            Piece::Bishop(c) => c,
            Piece::Queen(c) => c,
            Piece::King(c) => c,
        };

        if src_color != self.turn {
            return Err(format!("It isn't your turn ({}'s turn, tried to move a {} piece)", self.turn, src_color));
        } else if src_color == dst_color {
            return Err("Can't move into your own piece".to_string());
        }

        // TODO: implement castling
        // TODO: implement check

        self.set(dst, self.get(src));
        self.set(src, Piece::None);

        Ok(())
    }

    pub fn uci(&mut self, m: String) -> Result<(), String> {
        if m.len() != 4 {
            return Err(format!("Move is not the right length (UCI format uses 4 characters, recieved {})", m.len()));
        }

        // TODO: there has to be a better way
        let src = Board::coords_to_indices(format!("{}{}", m.chars().collect::<Vec<char>>()[0], m.chars().collect::<Vec<char>>()[1]))?;
        let dst = Board::coords_to_indices(format!("{}{}", m.chars().collect::<Vec<char>>()[2], m.chars().collect::<Vec<char>>()[3]))?;

        println!("{}", m);
        println!("{:?}, {:?}", src, dst);
        self.make_move(src, dst)
    }

    /// Converts algebraic coordinates to board indices; if the coords are invalid, and error will be returned detailing the problem
    pub fn coords_to_indices(m: String) -> Result<Square, String> {
        if m.len() != 2 {
            return Err(format!("Coordinates must be 2 characters (recieved {})", m.len()));
        }

        let first = m.chars().collect::<Vec<char>>()[0];
        let second = m.chars().collect::<Vec<char>>()[1];

        if !"abcdefgh".contains(first) {
            return Err(format!("First character must be in the range `a-z`, was {}", first));
        }

        if !"12345678".contains(second) {
            return Err(format!("Second character must be in the range `1-8`, was {}", second));
        }

        Ok([7 - String::from(second).parse::<usize>().unwrap(), 103 - (first as usize)])
    }

    /// Returns whose turn it is
    pub fn whos_turn(&self) -> Color {
        self.turn
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();

        let mut rind = 8;
        for row in &self.state {
            board.push_str(format!("{} | ", rind).as_str());
            for col in row {
                board.push_str(format!("{} ", col).as_str());
            }
            rind -= 1;
            board.push('\n');
        }

        board.push_str("  +----------------\n    a b c d e f g h");

        write!(f, "{}", board)
    }
}