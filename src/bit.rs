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
