use crate::{helper, mask, shift};

macro_rules! slide_fn {
    ( $outer:meta, $name:ident, $fn:expr ) => {
        #[$outer]
        /// until it hits a stop bit. Includes any stop bits hit.
        #[inline]
        pub fn $name(bb: u64, stop: u64) -> u64 {
            let f: fn(u64, u8) -> u64 = $fn;
            let mut out = f(bb, 1);

            for _ in 0..7 {
                out |= f(!stop & out, 1);
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
    /// Slide upwards
    , up, shift::up,
    /// Slide downwards
    , down, shift::down,
    /// Slide to the right
    , right, shift::right,
    /// Slide to the left
    , left, shift::left,
    /// Slide diagonally up-right
    , diag_right, shift::up_right,
    /// Slide diagonally down-left
    , diag_left, shift::down_left,
    /// Slide diagonally up-left
    , anti_diag_right, shift::up_left,
    /// Slide diagonally down-right
    , anti_diag_left, shift::down_right,
);
