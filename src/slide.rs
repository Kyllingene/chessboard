use crate::{helper, mask, shift};

macro_rules! slide_fn {
    ( $outer:meta, $name:ident, $fn:expr ) => {
        #[$outer]
        /// until it hits a stop bit. Includes any stop bits hit,
        /// but not the original bitboard.
        #[inline]
        pub fn $name(bb: u64, stop: u64) -> u64 {
            let f: fn(u64, u8) -> u64 = $fn;
            let mut out = f(bb, 1);

            for _ in 0..7 {
                out |= f(out & !stop, 1);
            }

            out & !bb
        }
    };
}

macro_rules! slide_fns {
    ( $( #[$outer:meta], $name:ident, $fn:expr ),* $(,)? ) => {
        $(slide_fn!($outer, $name, $fn);)*
    }
}

slide_fns!(
    /// Slides upwards
    , up, shift::up,
    /// Slides downwards
    , down, shift::down,
    /// Slides to the right
    , right, shift::right,
    /// Slides to the left
    , left, shift::left,
    /// Slides diagonally up-right
    , diag_right, shift::up_right,
    /// Slides diagonally down-left
    , diag_left, shift::down_left,
    /// Slides diagonally up-left
    , anti_diag_right, shift::up_left,
    /// Slides diagonally down-right
    , anti_diag_left, shift::down_right,
);
