// 0b0101010100110011000011110000000011110000110011001010101011111111
// =>
// 0 1 0 1 0 1 0 1
// 0 0 1 1 0 0 1 1
// 0 0 0 0 1 1 1 1
// 0 0 0 0 0 0 0 0
// 1 1 1 1 0 0 0 0
// 1 1 0 0 1 1 0 0
// 1 0 1 0 1 0 1 0
// 1 1 1 1 1 1 1 1

#[allow(unused)]
fn print_bitboard(bb: u64) {
    let board = bitboard_to_grid(bb);
    for row in board {
        for bit in row {
            print!("{} ", if bit { '1' } else { '0' });
        }
        println!();
    }
    println!();
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
    bitboard_shd(bitboard_shr(1, x), y)
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
    bb ^ bitxy(x, y)
}

#[inline]
pub fn file(x: u8) -> u64 {
    0xff << (x * 8)
}

#[inline]
pub fn row(y: u8) -> u64 {
    0x0101010101010101 << y
}

#[inline]
pub fn to_right(x: u8) -> u64 {
    !0 << (x * 8)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn indev() {
        print_bitboard(to_right(4) & row(3));
    }
}

