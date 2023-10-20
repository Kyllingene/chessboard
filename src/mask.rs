use crate::{shift, helper};

#[inline]
pub fn file(x: u8) -> u64 {
    0x0101010101010101 << x
}

#[inline]
pub fn row(y: u8) -> u64 {
    0xff << (y * 8)
}

#[inline]
pub fn rows(mut bb: u64) -> u64 {
    let mut out = 0;
    let mut mask = 0xff;

    for i in 0..7 {
        out |= (bb & mask != 0) as u64 * mask;
        mask <<= 8;
    }

    // one final iteration to avoid branches
    out |= (bb & mask != 0) as u64 * mask;

    out
}

#[inline]
pub fn files(mut bb: u64) -> u64 {
    let mut out = 0;
    let mut mask = 0x0101010101010101;

    for i in 0..7 {
        out |= (bb & mask != 0) as u64 * mask;
        mask <<= 1;
    }

    // one final iteration to avoid branches
    out |= (bb & mask != 0) as u64 * mask;

    out
}

#[inline]
pub fn up(y: u8) -> u64 {
    shift::up(!0, y)
}

#[inline]
pub fn down(y: u8) -> u64 {
    shift::down(!0, y)
}

#[inline]
pub fn left(x: u8) -> u64 {
    0xff << x
        | 0xff << (x + 8)
        | 0xff << (x + 16)
        | 0xff << (x + 24)
        | 0xff << (x + 32)
        | 0xff << (x + 40)
        | 0xff << (x + 48)
        | 0xff << (x + 56)
}

#[inline]
pub fn right(x: u8) -> u64 {
    0xff >> x
        | 0xff >> (x + 8)
        | 0xff >> (x + 16)
        | 0xff >> (x + 24)
        | 0xff >> (x + 32)
        | 0xff >> (x + 40)
        | 0xff >> (x + 48)
        | helper::shr(0xff, x + 56)
}

