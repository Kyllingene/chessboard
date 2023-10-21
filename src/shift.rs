use crate::mask;

#[inline]
pub fn right(bb: u64, i: u8) -> u64 {
    (bb >> i) & mask::right(i)
}

#[inline]
pub fn left(bb: u64, i: u8) -> u64 {
    (bb << i) & mask::left(i)
}

#[inline]
pub fn up(bb: u64, i: u8) -> u64 {
    bb << (i * 8)
}

#[inline]
pub fn down(bb: u64, i: u8) -> u64 {
    bb >> (i * 8)
}

#[inline]
pub fn up_left(bb: u64, i: u8) -> u64 {
    up(left(bb, i), i)
}

#[inline]
pub fn down_left(bb: u64, i: u8) -> u64 {
    down(left(bb, i), i)
}

#[inline]
pub fn up_right(bb: u64, i: u8) -> u64 {
    up(right(bb, i), i)
}

#[inline]
pub fn down_right(bb: u64, i: u8) -> u64 {
    down(right(bb, i), i)
}
