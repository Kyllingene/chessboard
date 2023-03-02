// 0b0101010100110011000011110000000011110000110011001010101011110111
// =>
// 0 1 0 1 0 1 0 1
// 0 0 1 1 0 0 1 1
// 0 0 0 0 1 1 1 1
// 0 0 0 0 0 0 0 0
// 1 1 1 1 0 0 0 0
// 1 1 0 0 1 1 0 0
// 1 0 1 0 1 0 1 0
// 1 1 1 1 0 1 1 1

use std::cmp::max;

#[allow(unused)]
fn print_bitboard(bb: u64) {
    let board = bitboard_to_grid(bb);
    for row in board {
        for bit in row {
            print!("{} ", if bit { '1' } else { '0' });
        }
        println!();
    }
}

pub fn bitboard_to_grid(mut bb: u64) -> [[bool; 8]; 8] {
    let mut rows = [0u8; 8];
    let mut grid = [[false; 8]; 8];

    for x in &mut rows {
        *x = bb as u8;
        bb >>= 8;
    }

    for (y, mut row) in rows.into_iter().enumerate() {
        for col in &mut grid {
            col[y] = match row & 1 {
                0 => false,
                1 => true,
                _ => unreachable!("x & 1 == 0 or 1"),
            };
            row >>= 1;
        }
    }

    grid
}

pub fn grid_to_coords(grid: [[bool; 8]; 8]) -> Vec<(u8, u8)> {
    let mut moves = Vec::new();

    for (x, col) in grid.iter().enumerate() {
        for (y, bit) in col.iter().enumerate() {
            if *bit {
                moves.push((x as u8, y as u8));
            }
        }
    }

    moves
}

pub fn print_board(board: Board) {
    println!();
    for y in 0..8 {
        for x in 0..8 {
            if !get_bit(board.white | board.black, x, y) {
                print!(". ");
                continue;
            }

            let white = get_bit(board.white, x, y);

            let piece = if get_bit(board.pawns, x, y) {
                'p'
            } else if get_bit(board.knights, x, y) {
                'n'
            } else if get_bit(board.bishops, x, y) {
                'b'
            } else if get_bit(board.rooks, x, y) {
                'r'
            } else if get_bit(board.queens, x, y) {
                'q'
            } else {
                'k'
            };

            print!(
                "{} ",
                if white {
                    piece.to_ascii_uppercase()
                } else {
                    piece
                }
            );
        }

        println!();
    }
}

#[inline]
pub fn bitboard_shl(bb: u64, i: u8) -> u64 {
    (bb >> i) & 0b0111111101111111011111110111111101111111011111110111111101111111
}

#[inline]
pub fn bitboard_shr(bb: u64, i: u8) -> u64 {
    (bb << i) & 0b1111111011111110111111101111111011111110111111101111111011111110
}

#[inline]
pub fn bitboard_shu(bb: u64, i: u8) -> u64 {
    bb >> (i * 8)
}

#[inline]
pub fn bitboard_shd(bb: u64, i: u8) -> u64 {
    bb << (i * 8)
}

#[inline]
pub fn bitboard_shul(bb: u64, i: u8) -> u64 {
    bitboard_shu(bitboard_shl(bb, i), i)
}

#[inline]
pub fn bitboard_shdl(bb: u64, i: u8) -> u64 {
    bitboard_shd(bitboard_shl(bb, i), i)
}

#[inline]
pub fn bitboard_shur(bb: u64, i: u8) -> u64 {
    bitboard_shu(bitboard_shr(bb, i), i)
}

#[inline]
pub fn bitboard_shdr(bb: u64, i: u8) -> u64 {
    bitboard_shd(bitboard_shr(bb, i), i)
}

#[inline]
pub fn bitxy(x: u8, y: u8) -> u64 {
    if x == 0 && y == 0 {
        1u64
    } else if x == 0 {
        bitboard_shd(1, y)
    } else if y == 0 {
        bitboard_shr(1, x)
    } else {
        bitboard_shd(bitboard_shr(1, x), y)
    }
}

#[inline]
pub fn set_bit(bb: u64, x: u8, y: u8) -> u64 {
    bb | bitxy(x, y)
}

#[inline]
pub fn unset_bit(bb: u64, x: u8, y: u8) -> u64 {
    bb & !bitxy(x, y)
}

#[inline]
pub fn get_bit(bb: u64, x: u8, y: u8) -> bool {
    bb & bitxy(x, y) != 0
}

#[inline]
pub fn toggle_bit(bb: u64, x: u8, y: u8) -> u64 {
    if get_bit(bb, x, y) {
        unset_bit(bb, x, y)
    } else {
        set_bit(bb, x, y)
    }
}

// TODO: can you inline a Fn() closure?
// this is a macro because the alternative (a function with a Fn() argument)
// might not support inlining the Fn(). is that actually true though?
#[macro_export]
macro_rules! slide_fn {
    ($bb:expr, $stop:expr, $f:ident) => {{
        let mut nbb = $f($bb, 1);
        for _ in 0..7 {
            nbb |= $f(!$stop & nbb, 1);
        }

        nbb & !$bb
    }};
}

#[inline]
pub fn slide_horiz(bb: u64, stop: u64) -> u64 {
    slide_fn!(bb, stop, bitboard_shu)
        | slide_fn!(bb, stop, bitboard_shd)
        | slide_fn!(bb, stop, bitboard_shl)
        | slide_fn!(bb, stop, bitboard_shr)
}

#[inline]
pub fn slide_diag(bb: u64, stop: u64) -> u64 {
    slide_fn!(bb, stop, bitboard_shul)
        | slide_fn!(bb, stop, bitboard_shdl)
        | slide_fn!(bb, stop, bitboard_shur)
        | slide_fn!(bb, stop, bitboard_shdr)
}

#[inline]
pub fn slide_all(bb: u64, stop: u64) -> u64 {
    slide_horiz(bb, stop) | slide_diag(bb, stop)
}

#[inline]
pub fn pawns_up(bb: u64, stop: u64) -> u64 {
    !stop & bitboard_shu(bb, 1)
}

#[inline]
pub fn pawns_down(bb: u64, stop: u64) -> u64 {
    !stop & bitboard_shd(bb, 1)
}

#[inline]
pub fn pawns_double_up(bb: u64, stop: u64) -> u64 {
    !stop
        & pawns_up(
            !stop
                & pawns_up(
                    0b0000000011111111000000000000000000000000000000000000000000000000 & bb,
                    stop,
                ),
            stop,
        )
}

#[inline]
pub fn pawns_double_down(bb: u64, stop: u64) -> u64 {
    pawns_down(
        pawns_down(
            0b0000000000000000000000000000000000000000000000001111111100000000 & bb,
            stop,
        ),
        stop,
    )
}

#[inline]
pub fn pawns_capture_up(bb: u64, stop: u64) -> u64 {
    !stop & (bitboard_shul(bb, 1) | bitboard_shur(bb, 1))
}

#[inline]
pub fn pawns_capture_down(bb: u64, stop: u64) -> u64 {
    !stop & (bitboard_shdl(bb, 1) | bitboard_shdr(bb, 1))
}

#[inline]
pub fn knight_moves(bb: u64, stop: u64) -> u64 {
    !stop
        & (bitboard_shu(bitboard_shl(bb, 1), 2)
            | bitboard_shd(bitboard_shl(bb, 1), 2)
            | bitboard_shu(bitboard_shr(bb, 1), 2)
            | bitboard_shd(bitboard_shr(bb, 1), 2)
            | bitboard_shu(bitboard_shl(bb, 2), 1)
            | bitboard_shd(bitboard_shl(bb, 2), 1)
            | bitboard_shu(bitboard_shr(bb, 2), 1)
            | bitboard_shd(bitboard_shr(bb, 2), 1))
}

#[inline]
pub fn king_moves(bb: u64, stop: u64) -> u64 {
    !stop
        & (bitboard_shur(bb, 1)
            | bitboard_shu(bb, 1)
            | bitboard_shul(bb, 1)
            | bitboard_shl(bb, 1)
            | bitboard_shdl(bb, 1)
            | bitboard_shd(bb, 1)
            | bitboard_shdr(bb, 1)
            | bitboard_shr(bb, 1))
}

#[inline]
pub fn white_moves(board: Board) -> u64 {
    pawns_up(board.pawns & board.white, board.white)
        | pawns_double_down(board.pawns & board.white, board.white)
        | knight_moves(board.knights & board.white, board.white)
        | (slide_diag(board.bishops & board.white, board.white | board.black) & !board.white)
        | (slide_horiz(board.rooks & board.white, board.white | board.black) & !board.white)
        | (slide_all(board.queens & board.white, board.white | board.black) & !board.white)
        | king_moves(board.kings & board.white, board.white)
}

#[inline]
pub fn black_moves(board: Board) -> u64 {
    pawns_up(board.pawns & board.black, board.black)
        | pawns_double_down(board.pawns & board.black, board.black)
        | knight_moves(board.knights & board.black, board.black)
        | (slide_diag(board.bishops & board.black, board.black | board.white) & !board.black)
        | (slide_horiz(board.rooks & board.black, board.black | board.white) & !board.black)
        | (slide_all(board.queens & board.black, board.black | board.white) & !board.black)
        | king_moves(board.kings & board.black, board.black)
}

#[inline]
pub fn white_captures(board: Board) -> u64 {
    pawns_capture_down(board.pawns & board.white, board.white)
        | knight_moves(board.knights & board.white, board.white)
        | (slide_diag(board.bishops & board.white, board.white | board.black) & !board.white)
        | (slide_horiz(board.rooks & board.white, board.white | board.black) & !board.white)
        | (slide_all(board.queens & board.white, board.white | board.black) & !board.white)
        | king_moves(board.kings & board.white, board.white)
}

#[inline]
pub fn black_captures(board: Board) -> u64 {
    pawns_capture_up(board.pawns & board.black, board.black)
        | knight_moves(board.knights & board.black, board.black)
        | (slide_diag(board.bishops & board.black, board.white | board.black) & !board.black)
        | (slide_horiz(board.rooks & board.black, board.white | board.black) & !board.black)
        | (slide_all(board.queens & board.black, board.white | board.black) & !board.black)
        | king_moves(board.kings & board.black, board.black)
}

pub fn piece_moves(board: Board, x: u8, y: u8) -> u64 {
    if !get_bit(board.white | board.black, x, y) {
        return 0;
    }

    let bit = bitxy(x, y);
    let white = get_bit(board.white, x, y);

    let (mine, other, castle) = if white {
        (board.white, board.black, board.white_can_castle)
    } else {
        (board.black, board.white, board.black_can_castle)
    };

    if get_bit(board.pawns, x, y) {
        if white {
            !mine
                & (pawns_down(bit, other)
                    | pawns_double_down(bit, mine | other)
                    | ((board.en_passant_targets | other) & pawns_capture_down(bit, mine)))
        } else {
            !mine
                & (pawns_up(bit, other)
                    | pawns_double_up(bit, mine | other)
                    | ((board.en_passant_targets | other) & pawns_capture_up(bit, mine)))
        }
    } else if get_bit(board.knights, x, y) {
        knight_moves(bit, mine)
    } else if get_bit(board.bishops, x, y) {
        !mine & slide_diag(bit, mine | other)
    } else if get_bit(board.rooks, x, y) {
        !mine & slide_horiz(bit, mine | other)
    } else if get_bit(board.queens, x, y) {
        !mine & slide_all(bit, mine | other)
    } else if get_bit(board.kings, x, y) {
        !mine & king_moves(bit, mine)
            | (if castle.0 {
                set_bit(0, 6, if white { 0 } else { 7 })
            } else {
                0
            })
            | (if castle.1 {
                set_bit(0, 2, if white { 0 } else { 7 })
            } else {
                0
            })
    } else {
        0
    }
}

pub fn white_piece_moves(board: Board) -> Vec<((u8, u8), (u8, u8))> {
    let mut all = Vec::new();
    for y in 0..8 {
        for x in 0..8 {
            if !get_bit(board.white, x, y) {
                continue;
            }

            let moves = grid_to_coords(bitboard_to_grid(piece_moves(board, x, y)));

            for m in moves {
                all.push(((x, y), m));
            }
        }
    }

    all
}

pub fn black_piece_moves(board: Board) -> Vec<((u8, u8), (u8, u8))> {
    let mut all = Vec::new();
    for y in 0..8 {
        for x in 0..8 {
            if get_bit(board.white, x, y) {
                continue;
            }

            let moves = grid_to_coords(bitboard_to_grid(piece_moves(board, x, y)));

            for m in moves {
                all.push(((x, y), m));
            }
        }
    }

    all
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Board {
    white: u64,
    black: u64,
    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    queens: u64,
    kings: u64,
    en_passant_targets: u64,

    white_can_castle: (bool, bool),
    black_can_castle: (bool, bool),

    whites_turn: bool,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            white: 0b0000000000000000000000000000000000000000000000001111111111111111,
            black: 0b1111111111111111000000000000000000000000000000000000000000000000,
            pawns: 0b0000000011111111000000000000000000000000000000001111111100000000,
            knights: 0b0100001000000000000000000000000000000000000000000000000001000010,
            bishops: 0b0010010000000000000000000000000000000000000000000000000000100100,
            rooks: 0b1000000100000000000000000000000000000000000000000000000010000001,
            queens: 0b0000100000000000000000000000000000000000000000000000000000001000,
            kings: 0b0001000000000000000000000000000000000000000000000000000000010000,
            en_passant_targets: 0,

            white_can_castle: (true, true),
            black_can_castle: (true, true),

            whites_turn: true,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl From<&str> for Board {
    // TODO: i hate `fen`'s error handling, but i have to deal with it someday
    fn from(value: &str) -> Self {
        let mut board = Board::new();

        let state = fen::BoardState::from_fen(value).unwrap();

        board.white_can_castle = (state.white_can_oo, state.white_can_ooo);
        board.black_can_castle = (state.black_can_oo, state.black_can_ooo);

        if let Some(s) = state.en_passant_square {
            board.en_passant_targets = 1 << s;
        }

        for (i, piece) in state.pieces.into_iter().enumerate() {
            if let Some(piece) = piece {
                if piece.color == fen::Color::White {
                    board.white |= 1 << i;
                } else {
                    board.black |= 1 << i;
                }

                match piece.kind {
                    fen::PieceKind::Pawn => board.pawns |= 1 << i,
                    fen::PieceKind::Knight => board.knights |= 1 << i,
                    fen::PieceKind::Bishop => board.bishops |= 1 << i,
                    fen::PieceKind::Rook => board.rooks |= 1 << i,
                    fen::PieceKind::Queen => board.queens |= 1 << i,
                    fen::PieceKind::King => board.kings |= 1 << i,
                }
            }
        }

        board.whites_turn = state.side_to_play == fen::Color::White;

        board
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            white: 0,
            black: 0,
            pawns: 0,
            knights: 0,
            bishops: 0,
            rooks: 0,
            queens: 0,
            kings: 0,
            en_passant_targets: 0,

            white_can_castle: (true, true),
            black_can_castle: (true, true),

            whites_turn: true,
        }
    }

    pub fn as_grid(&self) -> [[(Option<Piece>, Color); 8]; 8] {
        let mut grid = [[(None, Color::Black); 8]; 8];
        for y in 0..8 {
            for x in 0..8 {
                let c = if get_bit(self.white, x, y) {
                    Color::White
                } else {
                    Color::Black
                };
                let p = if get_bit(self.pawns, x, y) {
                    Some(Piece::Pawn)
                } else if get_bit(self.knights, x, y) {
                    Some(Piece::Knight)
                } else if get_bit(self.bishops, x, y) {
                    Some(Piece::Bishop)
                } else if get_bit(self.rooks, x, y) {
                    Some(Piece::Rook)
                } else if get_bit(self.queens, x, y) {
                    Some(Piece::Queen)
                } else if get_bit(self.kings, x, y) {
                    Some(Piece::King)
                } else {
                    None
                };

                grid[x as usize][y as usize] = (p, c);
            }
        }

        grid
    }

    pub fn set(&mut self, piece: Piece, color: Color, x: u8, y: u8) {
        match color {
            Color::White => {
                self.white = set_bit(self.white, x, y);
                self.black = unset_bit(self.black, x, y);
            }
            Color::Black => {
                self.black = set_bit(self.black, x, y);
                self.white = unset_bit(self.white, x, y);
            }
        }

        match piece {
            Piece::Pawn => self.pawns = set_bit(self.pawns, x, y),
            Piece::Knight => self.knights = set_bit(self.knights, x, y),
            Piece::Bishop => self.bishops = set_bit(self.bishops, x, y),
            Piece::Rook => self.rooks = set_bit(self.rooks, x, y),
            Piece::Queen => self.queens = set_bit(self.queens, x, y),
            Piece::King => self.kings = set_bit(self.kings, x, y),
        }
    }

    pub fn unset(&mut self, x: u8, y: u8) {
        self.white = unset_bit(self.white, x, y);
        self.black = unset_bit(self.black, x, y);
        self.pawns = unset_bit(self.pawns, x, y);
        self.knights = unset_bit(self.knights, x, y);
        self.bishops = unset_bit(self.bishops, x, y);
        self.rooks = unset_bit(self.rooks, x, y);
        self.queens = unset_bit(self.queens, x, y);
        self.kings = unset_bit(self.kings, x, y);
    }

    pub fn check(&self) -> (bool, bool) {
        // TODO: CONFIRM that this is EXHAUSTIVE
        (
            black_captures(*self) & self.kings != 0,
            white_captures(*self) & self.kings != 0,
        )
    }

    pub fn mate(&self) -> (bool, bool) {
        // TODO: CONFIRM that this is EXHAUSTIVE
        let (mut white, mut black) = self.check();

        if white {
            for ((sx, sy), (dx, dy)) in white_piece_moves(*self) {
                let mut test = *self;

                let piece = if get_bit(self.pawns, sx, sy) {
                    Piece::Pawn
                } else if get_bit(self.knights, sx, sy) {
                    Piece::Knight
                } else if get_bit(self.bishops, sx, sy) {
                    Piece::Bishop
                } else if get_bit(self.rooks, sx, sy) {
                    Piece::Rook
                } else if get_bit(self.queens, sx, sy) {
                    Piece::Queen
                } else if get_bit(self.kings, sx, sy) {
                    // castling not allowed in check
                    if (sx, sy) == (4, 0) && ((dx, dy) == (2, 0) || (dx, dy) == (6, 0)) {
                        continue;
                    }
                    Piece::King
                } else {
                    panic!("internal error: {sx},{sy} is not empty, but is not a valid piece type");
                };

                test.white = unset_bit(test.white, sx, sy);
                test.black = unset_bit(test.black, sx, sy);

                test.unset(dx, dy);
                test.unset(sx, sy);
                test.set(piece, Color::White, dx, dy);

                if get_bit(test.en_passant_targets, dx, dy) {
                    test.en_passant_targets = unset_bit(test.en_passant_targets, dx, dy);

                    // if you took an en passant target, the target has to be
                    // on a certain rank (dependent on color), and on the same file
                    // that you're moving to
                    test.unset(dx, 4);
                }

                if !test.check().0 {
                    white = false;
                }
            }
        }

        if black {
            for ((sx, sy), (dx, dy)) in black_piece_moves(*self) {
                let mut test = *self;

                let piece = if get_bit(self.pawns, sx, sy) {
                    Piece::Pawn
                } else if get_bit(self.knights, sx, sy) {
                    Piece::Knight
                } else if get_bit(self.bishops, sx, sy) {
                    Piece::Bishop
                } else if get_bit(self.rooks, sx, sy) {
                    Piece::Rook
                } else if get_bit(self.queens, sx, sy) {
                    Piece::Queen
                } else if get_bit(self.kings, sx, sy) {
                    // castling not allowed in check
                    if (sx, sy) == (4, 7) && ((dx, dy) == (2, 7) || (dx, dy) == (7, 5)) {
                        continue;
                    }
                    Piece::King
                } else {
                    panic!("internal error: {sx},{sy} is not empty, but is not a valid piece type");
                };

                test.white = unset_bit(test.white, sx, sy);
                test.black = unset_bit(test.black, sx, sy);

                test.unset(dx, dy);
                test.unset(sx, sy);
                test.set(piece, Color::Black, dx, dy);

                if get_bit(test.en_passant_targets, dx, dy) {
                    test.en_passant_targets = unset_bit(test.en_passant_targets, dx, dy);

                    // if you took an en passant target, the target has to be
                    // on a certain rank (dependent on color), and on the same file
                    // that you're moving to
                    test.unset(dx, 3);
                }

                if !test.check().1 {
                    black = false;
                }
            }
        }

        (white, black)
    }

    pub fn move_piece(
        &mut self,
        sx: u8,
        sy: u8,
        dx: u8,
        dy: u8,
        promote: Option<Piece>,
    ) -> Result<(), String> {
        if max(max(sx, sy), max(dx, dy)) > 7 {
            return Err("coordinates must be within the range `1..=7`".to_string());
        }

        if !get_bit(self.white | self.black, sx, sy) {
            return Err(format!("square {sx},{sy} is empty and cannot move"));
        }

        let white = get_bit(self.white, sx, sy);

        if white != self.whites_turn {
            return Err(format!(
                "it's not {}'s turn",
                if white { "white" } else { "black" }
            ));
        }

        let piece = if get_bit(self.pawns, sx, sy) {
            Piece::Pawn
        } else if get_bit(self.knights, sx, sy) {
            Piece::Knight
        } else if get_bit(self.bishops, sx, sy) {
            Piece::Bishop
        } else if get_bit(self.rooks, sx, sy) {
            Piece::Rook
        } else if get_bit(self.queens, sx, sy) {
            Piece::Queen
        } else if get_bit(self.kings, sx, sy) {
            Piece::King
        } else {
            return Err(format!(
                "internal error: {sx},{sy} is not empty, but is not a valid piece type"
            ));
        };

        let moves = grid_to_coords(bitboard_to_grid(piece_moves(*self, sx, sy)));
        if !moves.contains(&(dx, dy)) {
            return Err(format!(
                "the move {sx},{sy} -> {dx},{dy} is not a valid move"
            ));
        }

        let mut test = *self;
        test.whites_turn = !test.whites_turn;
        test.white = unset_bit(test.white, sx, sy);
        test.black = unset_bit(test.black, sx, sy);

        test.unset(dx, dy);
        test.unset(sx, sy);

        if piece == Piece::Pawn && get_bit(test.en_passant_targets, dx, dy) {
            test.en_passant_targets = unset_bit(test.en_passant_targets, dx, dy);

            // if you took an en passant target, the target has to be
            // on a certain rank (dependent on color), and on the same file
            // that you're moving to
            test.unset(dx, if white { 4 } else { 3 });
        }

        if piece == Piece::King && white {
            if (sx, sy, dx, dy) == (4, 0, 6, 0) {
                if (test.white | test.black) & 0b01100000 != 0 {
                    return Err("cannot short castle as white: pieces are in the way".to_string());
                } else if black_captures(test) & 0b01110000 != 0 {
                    return Err(
                        "cannot short castle as white: would castle through check".to_string()
                    );
                } else {
                    test.set(Piece::King, Color::White, dx, dy);
                    test.unset(7, 0);
                    test.set(Piece::Rook, Color::White, 5, 0);
                }
            } else if (sx, sy, dx, dy) == (4, 0, 2, 0) {
                if (test.white | test.black) & 0b00001110 != 0 {
                    return Err("cannot long castle as white: pieces are in the way".to_string());
                } else if black_captures(test) & 0b00011100 != 0 {
                    return Err(
                        "cannot long castle as white: would castle through check".to_string()
                    );
                } else {
                    test.set(Piece::King, Color::White, dx, dy);
                    test.unset(0, 0);
                    test.set(Piece::Rook, Color::White, 3, 0);
                }
            } else {
                test.set(
                    piece,
                    if white { Color::White } else { Color::Black },
                    dx,
                    dy,
                );
            }
        } else if piece == Piece::King && !white {
            if (sx, sy, dx, dy) == (4, 7, 6, 7) {
                if (test.white | test.black) & (0b01100000 << 56) != 0 {
                    return Err("cannot short castle as black: pieces are in the way".to_string());
                } else if white_captures(test) & (0b01110000 << 56) != 0 {
                    return Err(
                        "cannot short castle as black: would castle through check".to_string()
                    );
                } else {
                    test.set(Piece::King, Color::Black, dx, dy);
                    test.unset(7, 7);
                    test.set(Piece::Rook, Color::Black, 5, 7);
                }
            } else if (sx, sy, dx, dy) == (4, 7, 2, 7) {
                if (test.white | test.black) & (0b00001110 << 56) != 0 {
                    return Err("cannot long castle as black: pieces are in the way".to_string());
                } else if white_captures(test) & (0b00011100 << 56) != 0 {
                    return Err(
                        "cannot long castle as black: would castle through check".to_string()
                    );
                } else {
                    test.set(Piece::King, Color::Black, dx, dy);
                    test.unset(0, 7);
                    test.set(Piece::Rook, Color::Black, 3, 7);
                }
            } else {
                test.set(
                    piece,
                    if white { Color::White } else { Color::Black },
                    dx,
                    dy,
                );
            }
        } else if piece == Piece::Pawn && (dy == 7 || dy == 0) {
            test.set(
                promote.unwrap_or(Piece::Queen),
                if white { Color::White } else { Color::Black },
                dx,
                dy,
            );
        } else {
            test.set(
                piece,
                if white { Color::White } else { Color::Black },
                dx,
                dy,
            );
        }

        if white && test.check().0 {
            return Err(format!(
                "the move {sx},{sy} -> {dx},{dy} is not a legal move (puts white in check)"
            ));
        } else if !white && test.check().1 {
            return Err(format!(
                "the move {sx},{sy} -> {dx},{dy} is not a legal move (puts black in check)"
            ));
        }

        *self = test;
        self.en_passant_targets = 0;

        if piece == Piece::Pawn {
            if white && sy == 6 && dy == 4 {
                self.en_passant_targets = set_bit(self.en_passant_targets, dx, 5);
            } else if !white && sy == 1 && dy == 3 {
                self.en_passant_targets = set_bit(self.en_passant_targets, dx, 2);
            }
        } else if piece == Piece::King {
            if white {
                self.white_can_castle = (false, false);
            } else {
                self.black_can_castle = (false, false);
            }
        }

        if (sx, sy) == (0, 0) || (dx, dy) == (0, 0) {
            self.white_can_castle.0 = false;
        } else if (sx, sy) == (7, 0) || (dx, dy) == (7, 0) {
            self.white_can_castle.1 = false;
        }

        if (sx, sy) == (0, 7) || (dx, dy) == (0, 7) {
            self.black_can_castle.0 = false;
        } else if (sx, sy) == (7, 7) || (dx, dy) == (7, 7) {
            self.black_can_castle.1 = false;
        }

        Ok(())
    }

    pub fn uci_to_coords(sx: char, dx: char) -> Result<(u8, u8), String> {
        Ok((
            match sx {
                'a' | 'A' => 0,
                'b' | 'B' => 1,
                'c' | 'C' => 2,
                'd' | 'D' => 3,
                'e' | 'E' => 4,
                'f' | 'F' => 5,
                'g' | 'G' => 6,
                'h' | 'H' => 7,

                _ => Err(format!("invalid src rank: {sx}"))?,
            },
            match dx {
                'a' | 'A' => 0,
                'b' | 'B' => 1,
                'c' | 'C' => 2,
                'd' | 'D' => 3,
                'e' | 'E' => 4,
                'f' | 'F' => 5,
                'g' | 'G' => 6,
                'h' | 'H' => 7,

                _ => Err(format!("invalid dest rank: {dx}"))?,
            },
        ))
    }

    pub fn uci(&mut self, uci: String, promote: Option<Piece>) -> Result<(), String> {
        if uci.len() != 4 {
            return Err(format!(
                "UCI format uses 4 characters, recieved {}",
                uci.len()
            ));
        }

        let chars = uci.chars().collect::<Vec<char>>();
        let sy = chars[1]
            .to_string()
            .parse::<u8>()
            .map_err(|_| format!("invalid src file: {}", chars[1]))?
            .checked_sub(1)
            .ok_or("0 is an invalid file".to_string())?;

        let dy = chars[3]
            .to_string()
            .parse::<u8>()
            .map_err(|_| format!("invalid src file: {}", chars[3]))?
            .checked_sub(1)
            .ok_or("0 is an invalid file".to_string())?;

        let (sx, dx) = Self::uci_to_coords(chars[0], chars[2])?;

        self.move_piece(sx, sy, dx, dy, promote)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_queens() {
        let mut queens = set_bit(0, 3, 1);
        queens = set_bit(queens, 5, 4);

        let mut other = set_bit(0, 2, 1);
        other = set_bit(other, 5, 1);
        other = set_bit(other, 2, 4);
        other = set_bit(other, 3, 6);

        let moves = slide_all(queens, other);

        println!("\n\n === queen ===");
        print_bitboard(queens);

        println!("\n\n === other ===");
        print_bitboard(other);

        println!("\n\n === slide ===");
        print_bitboard(moves);
    }

    #[test]
    fn test_pawns() {
        let mut pawns = set_bit(0, 3, 2);
        pawns = set_bit(pawns, 4, 2);
        pawns = set_bit(pawns, 6, 5);

        let mut other = set_bit(0, 2, 3);
        other = set_bit(other, 3, 3);
        other = set_bit(other, 6, 7);

        let takes = pawns_capture_down(pawns, other);
        let moves = pawns_down(pawns, other) | pawns_double_down(pawns, other);

        println!("\n\n === takes ===");
        print_bitboard(takes);

        println!("\n\n === other ===");
        print_bitboard(other);

        println!("\n\n === moves ===");
        print_bitboard(moves);
    }

    #[test]
    fn test_knights() {
        let knights = set_bit(0, 4, 4);

        let mut other = set_bit(0, 3, 2);
        other = set_bit(other, 6, 5);

        let moves = knight_moves(knights, other);

        println!("\n\n === kngts ===");
        print_bitboard(knights);

        println!("\n\n === other ===");
        print_bitboard(other);

        println!("\n\n === moves ===");
        print_bitboard(moves);
    }

    #[test]
    fn test_kings() {
        let kings = set_bit(0, 4, 4);

        let mut other = set_bit(0, 3, 3);
        other = set_bit(other, 5, 4);

        let moves = king_moves(kings, other);

        println!("\n\n === kings ===");
        print_bitboard(kings);

        println!("\n\n === other ===");
        print_bitboard(other);

        println!("\n\n === moves ===");
        print_bitboard(moves);
    }

    #[test]
    fn test_default_board() {
        let board = Board::default();

        print_board(board);

        println!("\n\n   white possible captures");
        print_bitboard(white_captures(board));

        println!("\n\n   black possible captures");
        print_bitboard(black_captures(board));
    }

    #[test]
    fn test_piece_moves() {
        let mut board = Board::new();
        board.set(Piece::Queen, Color::White, 3, 4);
        board.set(Piece::Rook, Color::White, 6, 5);
        board.set(Piece::Knight, Color::White, 2, 3);

        board.set(Piece::Queen, Color::Black, 6, 7);
        board.set(Piece::Rook, Color::Black, 4, 5);
        board.set(Piece::Knight, Color::Black, 5, 3);

        print_board(board);

        println!("\n\n white possible queen moves");
        print_bitboard(piece_moves(board, 3, 4));

        println!("\n\n black possible knight moves");
        print_bitboard(piece_moves(board, 5, 3));

        println!("\n\n black possible rook moves");
        print_bitboard(piece_moves(board, 4, 5));
    }

    #[test]
    fn test_fen() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let board = Board::try_from(fen).unwrap();

        println!("\n\n == default == ");
        print_board(Board::default());

        println!("\n\n == initial == ");
        print_board(board);

        let fen = "r2qnrk1/3nbppp/p2pb3/4p1P1/1p2PP2/1N2B3/PPPQN2P/2KR1B1R b - - 0 14";

        let board = Board::try_from(fen).unwrap();

        println!("\n\n == board 1 == ");
        print_board(board);

        let fen = "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1 b - - 0 1";

        let board = Board::try_from(fen).unwrap();

        println!("\n\n == board 2 == ");
        print_board(board);
    }
}
