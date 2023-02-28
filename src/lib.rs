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

#[allow(unused)]
fn print_bitboard(bb: u64) {
    let board = bitboard_to_grid(bb);
    for y in 0..8 {
        for x in 0..8 {
            print!("{} ", if board[x][y] { '1' } else { '0' });
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
        for x in 0..8 {
            grid[x][y] = match row & 1 {
                0 => false,
                1 => true,
                _ => unreachable!("x & 1 == 0 or 1")
            };
            row >>= 1;
        }
    }

    grid
}

pub fn grid_to_coords(grid: [[bool; 8]; 8]) -> Vec<(u8, u8)> {
    let mut moves = Vec::new();

    for x in 0..8 {
        for y in 0..8 {
            if grid[x][y] {
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

            print!("{} ", if white { piece.to_ascii_uppercase() } else { piece });
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

// this is a macro, because the alternative (a function with a Fn() argument)
// wouldn't support inlining the Fn()
#[macro_export]
macro_rules! slide_fn {
    ($bb:expr, $stop:expr, $f:ident) => {
        {           
            let mut nbb = $f($bb, 1);
            for _ in 0..7 {
                nbb |= $f(!$stop & nbb, 1);
            }
            
            nbb & !$bb
        }
    };
}

#[inline]
pub fn slide_horiz(bb: u64, stop: u64) -> u64 {
    slide_fn!(bb, stop, bitboard_shu) |
        slide_fn!(bb, stop, bitboard_shd) |
        slide_fn!(bb, stop, bitboard_shl) |
        slide_fn!(bb, stop, bitboard_shr)
}

#[inline]
pub fn slide_diag(bb: u64, stop: u64) -> u64 {
    slide_fn!(bb, stop, bitboard_shul) |
        slide_fn!(bb, stop, bitboard_shdl) |
        slide_fn!(bb, stop, bitboard_shur) |
        slide_fn!(bb, stop, bitboard_shdr)
}

#[inline]
pub fn slide_all(bb: u64, stop: u64) -> u64 {
    slide_horiz(bb, stop) |
        slide_diag(bb, stop)
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
    !stop & pawns_up(!stop & pawns_up(0b0000000011111111000000000000000000000000000000000000000000000000 & bb, stop), stop)
}

#[inline]
pub fn pawns_double_down(bb: u64, stop: u64) -> u64 {
    pawns_down(pawns_down(0b0000000000000000000000000000000000000000000000001111111100000000 & bb, stop), stop)
}

#[inline]
pub fn pawns_capture_up(bb: u64, stop: u64) -> u64 {
    !stop & (bitboard_shul(bb, 1) |
        bitboard_shur(bb, 1))
}

#[inline]
pub fn pawns_capture_down(bb: u64, stop: u64) -> u64 {
    !stop & (bitboard_shdl(bb, 1) |
        bitboard_shdr(bb, 1))
}

#[inline]
pub fn knight_moves(bb: u64, stop: u64) -> u64 {
    !stop & (bitboard_shu(bitboard_shl(bb, 1), 2) |
        bitboard_shd(bitboard_shl(bb, 1), 2) |
        bitboard_shu(bitboard_shr(bb, 1), 2) |
        bitboard_shd(bitboard_shr(bb, 1), 2) |
        bitboard_shu(bitboard_shl(bb, 2), 1) |
        bitboard_shd(bitboard_shl(bb, 2), 1) |
        bitboard_shu(bitboard_shr(bb, 2), 1) |
        bitboard_shd(bitboard_shr(bb, 2), 1))
}

#[inline]
pub fn king_moves(bb: u64, stop: u64) -> u64 {
    !stop & (bitboard_shur(bb, 1) |
        bitboard_shu(bb, 1) |
        bitboard_shul(bb, 1) |
        bitboard_shl(bb, 1) |
        bitboard_shdl(bb, 1) |
        bitboard_shd(bb, 1) |
        bitboard_shdr(bb, 1) |
        bitboard_shr(bb, 1))
}

#[inline]
pub fn white_moves(board: Board) -> u64 {
    pawns_up(board.pawns & board.white, board.white) |
        pawns_double_up(board.pawns & board.white, board.white) |
        knight_moves(board.knights & board.white, board.white) |
        (slide_diag(board.bishops & board.white, board.white | board.black) & !board.white) |
        (slide_horiz(board.rooks & board.white, board.white | board.black) & !board.white) |
        (slide_all(board.queens & board.white, board.white | board.black) & !board.white) |
        king_moves(board.kings & board.white, board.white)
}

#[inline]
pub fn white_captures(board: Board) -> u64 {
    pawns_capture_up(board.pawns & board.white, board.white) |
            knight_moves(board.knights & board.white, board.white) |
            (slide_diag(board.bishops & board.white, board.white | board.black) & !board.white) |
            (slide_horiz(board.rooks & board.white, board.white | board.black) & !board.white) |
            (slide_all(board.queens & board.white, board.white | board.black) & !board.white) |
            king_moves(board.kings & board.white, board.white)
}

#[inline]
pub fn black_captures(board: Board) -> u64 {
    pawns_capture_down(board.pawns & board.black, board.black) |
            knight_moves(board.knights & board.black, board.black) |
            (slide_diag(board.bishops & board.black, board.white | board.black) & !board.black) |
            (slide_horiz(board.rooks & board.black, board.white | board.black) & !board.black) |
            (slide_all(board.queens & board.black, board.white | board.black) & !board.black) |
            king_moves(board.kings & board.black, board.black)
}

#[inline]
pub fn piece_moves(board: Board, x: u8, y: u8) -> u64 {
    if !get_bit(board.white | board.black, x, y) {
        return 0;
    }

    let bit = bitxy(x, y);
    let white = get_bit(board.white, x, y);

    let (mine, stop) = if white {
        (board.white, board.black)
    } else {
        (board.black, board.white)
    };

    if get_bit(board.pawns, x, y) {
        if white {
            !mine & (pawns_up(bit, stop) |
                pawns_double_up(bit, mine | stop))
        } else {
            !mine & (pawns_down(bit, stop) |
                pawns_double_down(bit, mine | stop))
        }
    } else if get_bit(board.knights, x, y) {
        knight_moves(bit, mine)
    } else if get_bit(board.bishops, x, y) {
        !mine & slide_diag(bit, mine | stop)
    } else if get_bit(board.rooks, x, y) {
        !mine & slide_horiz(bit, mine | stop)
    } else if get_bit(board.queens, x, y) {
        !mine & slide_all(bit, mine | stop)
    } else if get_bit(board.kings, x, y) {
        !mine & king_moves(bit, mine | stop)
    } else {
        0
    }
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
}

impl Default for Board {
    fn default() -> Self {
        Self {
            white: 0b1111111111111111000000000000000000000000000000000000000000000000,
            black: 0b0000000000000000000000000000000000000000000000001111111111111111,
            pawns: 0b0000000011111111000000000000000000000000000000001111111100000000,
            knights: 0b0100001000000000000000000000000000000000000000000000000001000010,
            bishops: 0b0010010000000000000000000000000000000000000000000000000000100100,
            rooks: 0b1000000100000000000000000000000000000000000000000000000010000001,
            queens: 0b0001000000000000000000000000000000000000000000000000000000010000,
            kings: 0b0000100000000000000000000000000000000000000000000000000000001000,
            en_passant_targets: 0,

            white_can_castle: (true, true),
            black_can_castle: (true, true),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black
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
        }
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
            Piece::King => self.kings = set_bit(self.kings, x, y)
        }
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
        let moves = pawns_down(pawns, other) |
            pawns_double_down(pawns, other);

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
}