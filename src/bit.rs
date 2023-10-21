use crate::shift;

#[inline]
pub fn xy(x: u8, y: u8) -> u64 {
    (1 << x) << (y * 8)
}

#[inline]
pub fn set(bb: u64, x: u8, y: u8) -> u64 {
    bb | xy(x, y)
}

#[inline]
pub fn unset(bb: u64, x: u8, y: u8) -> u64 {
    bb & !xy(x, y)
}

#[inline]
pub fn get(bb: u64, x: u8, y: u8) -> bool {
    bb & xy(x, y) != 0
}

#[inline]
pub fn toggle(bb: u64, x: u8, y: u8) -> u64 {
    bb ^ xy(x, y)
}

#[inline]
pub fn idx(i: u8) -> u64 {
    1 << i
}

#[inline]
pub fn get_idx(bb: u64, i: u8) -> u64 {
    bb & idx(i)
}

#[inline]
pub fn set_idx(bb: u64, i: u8) -> u64 {
    bb | idx(i)
}

#[inline]
pub fn unset_idx(bb: u64, i: u8) -> u64 {
    bb & !idx(i)
}

#[inline]
pub fn toggle_idx(bb: u64, i: u8) -> u64 {
    bb ^ idx(i)
}

