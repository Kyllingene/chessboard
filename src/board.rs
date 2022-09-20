use crate::error::*;
use crate::color::*;
use crate::piece::*;
use crate::moves::*;

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
            white: Castle::Both,
            black: Castle::Both,
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
                    '-' => castling = Castling{white: Castle::None, black: Castle::None},
                    'K' => {
                        if castling.white == Castle::None {
                            castling.white = Castle::Kingside;
                        } else {
                            castling.white = Castle::Both;
                        }
                    },
                    'k' => {
                        if castling.black == Castle::None {
                            castling.black = Castle::Kingside;
                        } else {
                            castling.black = Castle::Both;
                        }
                    },
                    'Q' => {
                        if castling.white == Castle::None {
                            castling.white = Castle::Queenside;
                        } else {
                            castling.white = Castle::Both;
                        }
                    },
                    'q' => {
                        if castling.black == Castle::None {
                            castling.black = Castle::Queenside;
                        } else {
                            castling.black = Castle::Both;
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
            Some(Square::coords_to_indices(en_passant).unwrap())
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
        self.state[7 - square.file as usize][square.rank as usize]
    }

    /// Sets a piece on the board
    pub fn set(&mut self, square: Square, piece: Piece) {
        self.state[7 - square.file as usize][square.rank as usize] = piece;
    }

    /// Validates a move; if the move is invalid, an error will be returned detailing the problem
    pub fn validate(&self, m: Move) -> ChessResult<()> {
        if m.src == m.dst {
            return ChessErr::SourceIsDestination.into();
        }

        let src_color = match self.get(m.src) {
            Piece::None => return ChessErr::SourceEmpty.into(),
            Piece::Pawn(c) => c,
            Piece::Knight(c) => c,
            Piece::Rook(c) => c,
            Piece::Bishop(c) => c,
            Piece::Queen(c) => c,
            Piece::King(c) => c,
        };

        let dst_color = match self.get(m.dst) {
            Piece::None => {
                match src_color {
                    Color::Random  => Color::Invalid,
                    Color::Invalid => Color::Invalid,
                    Color::White   => Color::Black,
                    Color::Black   => Color::White,
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
            return ChessErr::WrongTurn.into();
        } else if src_color == dst_color {
            return ChessErr::CantCaptureOwnPiece.into();
        }

        /*
        TODO: implement castling
        TODO: implement check
        TODO: implement move directions
        TODO: implement move obstruction
        TODO: implement en passant
        TODO: implement castling validity
        */

        Ok(())
    }

    fn castle(&mut self, side: Color, dir: Castle) -> ChessResult<()> {
        match side {

            _ => {Ok(())},
        }
    }

    /// Makes a move; if the move is invalid, an error will be returned detailing the problem
    /// Castling is achieved by moving the king two squares towards the rook, not onto the rook
    pub fn make_move(&mut self, m: Move) -> ChessResult<()> {
        self.validate(m)?;

        let mut castle = Castle::None;
        let mut side = Color::Invalid;

        let src = m.src;
        let dst = m.dst;

        if self.get(src) == Piece::King(Color::White) {
            if dst.rank == Rank::First && dst.file == File::B {
                castle = Castle::Kingside;
                side = Color::White;
            } else if dst.rank == Rank::First && dst.file == File::G {
                castle = Castle::Queenside;
                side = Color::White;
            }
        }

        if castle != Castle::None && side != Color::Invalid {
            match castle {
                Castle::Kingside => {

                },
                Castle::Queenside => {

                },
                _ => panic!("Internal error: {:?} is invalid castling", castle)
            }
        } else {
            self.set(m.dst, self.get(m.src));
            self.set(m.src, Piece::None);
        }

        self.turn = !self.turn;

        {
        // if any rooks have moved, remove respective castling rights
        if self.get(Square{rank: Rank::First, file: File::A}) != Piece::Rook(Color::White) {
            self.castling.white.remove(Castle::Queenside);
        }

        if self.get(Square{rank: Rank::First, file: File::H}) != Piece::Rook(Color::White) {
            self.castling.white.remove(Castle::Kingside);
        }

        if self.get(Square{rank: Rank::Eighth, file: File::A}) != Piece::Rook(Color::Black) {
            self.castling.black.remove(Castle::Queenside);
        }

        if self.get(Square{rank: Rank::Eighth, file: File::H}) != Piece::Rook(Color::Black) {
            self.castling.black.remove(Castle::Kingside);
        }

        // if either king has moved, remove respective castling rights
        if self.get(Square{rank: Rank::First, file: File::E}) != Piece::Rook(Color::White) {
            self.castling.white.remove(Castle::Both);
        }

        if self.get(Square{rank: Rank::Eighth, file: File::E}) != Piece::Rook(Color::Black) {
            self.castling.black.remove(Castle::Both);
        }}

        Ok(())
    }

    /// Makes a move without updating the turn indicator
    pub fn make_move_same_turn(&mut self, m: Move) -> ChessResult<()> {
        self.make_move(m)?;
        self.turn = !self.turn;
        Ok(())
    }

    /// Makes a move as if it were the previous turn
    pub fn make_move_prev_turn(&mut self, m: Move) -> ChessResult<()> {
        self.turn = !self.turn;
        self.make_move(m)?;
        Ok(())
    }

    pub fn uci(&mut self, m: String) -> ChessResult<()> {
        self.make_move(Move::uci(m)?)
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