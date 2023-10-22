use crate::{shift, slide};

#[macro_export]
macro_rules! not_mine {
    ( $( $fn:expr ),* => $( $bb:expr ),* ; $stop:expr, $mine:expr ) => {
        (0 $( | $fn($bb & $mine, $stop) )*) & !$mine
    }
}

#[inline]
pub fn rook(bb: u64, stop: u64) -> u64 {
    slide::up(bb, stop) | slide::down(bb, stop) | slide::left(bb, stop) | slide::right(bb, stop)
}

#[inline]
pub fn bishop(bb: u64, stop: u64) -> u64 {
    slide::diag_left(bb, stop)
        | slide::diag_right(bb, stop)
        | slide::anti_diag_left(bb, stop)
        | slide::anti_diag_right(bb, stop)
}

#[inline]
pub fn queen(bb: u64, stop: u64) -> u64 {
    rook(bb, stop) | bishop(bb, stop)
}

#[inline]
pub fn king(bb: u64, _: u64) -> u64 {
    shift::up(bb, 1)
        | shift::down(bb, 1)
        | shift::left(bb, 1)
        | shift::right(bb, 1)
        | shift::up_left(bb, 1)
        | shift::up_right(bb, 1)
        | shift::down_left(bb, 1)
        | shift::down_right(bb, 1)
}

// TODO: if the extra argument is unnecessary, remove
#[inline]
pub fn knight(bb: u64, _: u64) -> u64 {
    shift::left(shift::up(bb, 2), 1)
        | shift::left(shift::up(bb, 1), 2)
        | shift::right(shift::up(bb, 2), 1)
        | shift::right(shift::up(bb, 1), 2)
        | shift::left(shift::down(bb, 2), 1)
        | shift::left(shift::down(bb, 1), 2)
        | shift::right(shift::down(bb, 2), 1)
        | shift::right(shift::down(bb, 1), 2)
}

#[inline]
pub fn pawn_down(bb: u64, stop: u64) -> u64 {
    shift::down(bb, 1) | (shift::down_left(bb, 1) & stop) | (shift::down_right(bb, 1) & stop)
}

#[inline]
pub fn pawn_up(bb: u64, stop: u64) -> u64 {
    shift::up(bb, 1) | (shift::up_left(bb, 1) & stop) | (shift::up_right(bb, 1) & stop)
}
