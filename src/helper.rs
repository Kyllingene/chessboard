use std::arch::asm;

/// Takes a bitmask to apply to each byte of the input,
/// and packs the resulting bits into a byte.
#[inline]
pub fn mask_to_byte(mut data: u64, mask: u8) -> u8 {
    let mut mask = (mask as u64) << (7 * 8);
    let mut out = 0;

    for i in 0..7 {
        out |= (data & mask != 0) as u8;
        mask >>= 8;
        out <<= 1;
    }

    out |= (data & mask != 0) as u8;

    out
}

/// Takes a bitmask to apply to each byte of the input,
/// and packs the resulting bits into a byte.
#[inline]
pub fn mask_to_byte_offset(mut data: u64, mask: u8, offset: i8) -> u8 {
    let mut mask = (mask as u64) << (7 * 8);
    let mut out = 0;

    for i in 0..7 {
        out |= (data & mask != 0) as u8;
        mask >>= (8 + offset) as u8;
        out <<= 1;
    }

    out |= (data & mask != 0) as u8;

    out
}

/// Shifts right by N. Bypasses overflow checks via inline assembly.
#[inline]
pub fn shr(mut x: u64, n: u8) -> u64 {
    unsafe {
        asm!(
            "shr {0}, {1}",
            inout(reg) x,
            in(reg_byte) n,
        );
    }

    x
}

/// Shifts left by N. Bypasses overflow checks via inline assembly.
#[inline]
pub fn shl(mut x: u64, n: u8) -> u64 {
    unsafe {
        asm!(
            "shl {0}, {1}",
            inout(reg) x,
            in(reg_byte) n,
        );
    }

    x
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
            grid[y][x] = row & 1 != 0;
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

pub fn bitboard_to_coords(bb: u64) -> Vec<(u8, u8)> {
    grid_to_coords(bitboard_to_grid(bb))
}

pub fn bitboard_to_coords_contains(bb: u64, x: u8, y: u8) -> bool {
    let grid = bitboard_to_grid(bb);

    for (tx, col) in grid.iter().enumerate() {
        for (ty, bit) in col.iter().enumerate() {
            if *bit && x == tx as u8 && y == ty as u8 {
                return true;
            }
        }
    }

    false
}
