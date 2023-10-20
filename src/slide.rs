use crate::mask;
use crate::precompute::FIRST_BIT;
use crate::helper;

#[inline]
pub fn up(bb: u64, stop: u64) -> u64 {
    let mut out = 0;
    let mut mask = 0x0101010101010101;

    for i in 0..7 {
        let masked = bb & mask;
        let first_bit = FIRST_BIT[helper::mask_to_byte(masked, mask as u8) as usize];

        out |= (masked != 0) as u64 * (mask << (first_bit * 8));
        mask <<= 1;
    }

    // one final iteration to avoid branches
    let masked = bb & mask;
    let first_bit = FIRST_BIT[helper::mask_to_byte(masked, mask as u8) as usize];

    out |= (masked != 0) as u64 * (mask << (first_bit * 8));
    mask <<= 1;

    out
}


#[inline]
pub fn down(bb: u64, stop: u64) -> u64 {
    let mut out = 0;
    let mut mask = 0x0101010101010101;

    for i in 0..7 {
        let masked = bb & mask;
        let first_bit = FIRST_BIT[helper::mask_to_byte(masked, mask as u8) as usize].saturating_sub(1);
        out |= (masked != 0) as u64 * helper::shr(mask, ((8 - first_bit) * 8));
        mask <<= 1;
    }

    // one final iteration to avoid branches
    let masked = bb & mask;
    let first_bit = FIRST_BIT[helper::mask_to_byte(masked, mask as u8) as usize].saturating_sub(1);
    out |= (masked != 0) as u64 * helper::shr(mask, ((8 - first_bit) * 8));
    mask <<= 1;

    out
}

#[inline]
pub fn left(bb: u64, stop: u64) -> u64 {
    let mut out = 0;
    let mut mask = 0xff;

    for i in 0..7 {
        let masked = (bb & mask) >> (i * 8);
        let first_bit = FIRST_BIT[masked as usize].saturating_sub(1);

        out |= (masked != 0) as u64 * (mask << first_bit);
        mask <<= 8;
    }

    out
}

