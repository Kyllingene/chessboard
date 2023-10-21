use crate::{helper, shift, mask};
use crate::precompute::FIRST_BIT;

/// Internal macro to ease creation of slide functions.
macro_rules! slide_fn {
    ( $name:ident, $fn:expr ) => {
        #[inline]
        pub fn $name(bb: u64, stop: u64) -> u64 {
            let mut out = 0;

            for _ in 0..8 {
                out |= $fn();
            }

            out
        }
    }
}

