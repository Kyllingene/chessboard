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

/// Shifts right by N. Uses inline assembly to bypass overflow checks.
#[inline]
pub fn shr(mut x: u64, n: u8) -> u64 {
    unsafe {
        asm!(
            "shr {}, {}",
            inlateout(reg) x,
            in(reg_byte) n,
        );
    }

    x
}

