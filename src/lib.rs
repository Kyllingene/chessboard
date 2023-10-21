#![allow(unused)]
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

mod bit;
mod board;
mod error;
mod helper;
mod mask;
mod piece;
mod shift;
mod slide;

#[cfg(test)]
mod test;

#[allow(unused)]
fn print_bitboard(bb: u64) {
    let board = bitboard_to_grid(bb);
    for row in board.into_iter().rev() {
        for bit in row.into_iter().rev() {
            print!("{} ", if bit { '#' } else { '.' });
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
