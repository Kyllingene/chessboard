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

/// Takes a bitmask to apply to each byte of the input,
/// and packs the resulting bits into a byte.
#[inline]
pub fn mask_to_byte_offset(mut data: u64, mask: u8, offset: i8) -> u8 {
    let mut mask = (mask as u64) << (7 * 8);
    let mut out = 0;

    for i in 0..7 {
        out |= (data & mask != 0) as u8;
        mask >>= (8 + offset) as u8;
        out <<= 1;
    }

    out |= (data & mask != 0) as u8;

    out
}

/// Shifts right by N. Bypasses overflow checks via inline assembly.
#[inline]
pub fn shr(mut x: u64, n: u8) -> u64 {
    unsafe {
        asm!(
            "shr {0}, {1}",
            inout(reg) x,
            in(reg_byte) n,
        );
    }

    x
}

/// Shifts left by N. Bypasses overflow checks via inline assembly.
#[inline]
pub fn shl(mut x: u64, n: u8) -> u64 {
    unsafe {
        asm!(
            "shl {0}, {1}",
            inout(reg) x,
            in(reg_byte) n,
        );
    }

    x
}
