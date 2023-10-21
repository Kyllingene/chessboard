use crate::{helper, shift, mask};
use crate::precompute::FIRST_BIT;

#[inline]
pub fn up(bb: u64, stop: u64) -> u64 {
    let mut out = 0;
    let mut mask = 0x0101010101010101;

    for i in 0..8 {
        let masked = bb & mask;
        let first_bit = FIRST_BIT[helper::mask_to_byte(masked, mask as u8) as usize];

        out |= (masked != 0) as u64 * helper::shl(mask, first_bit * 8);
        mask <<= 1;
    }

    out
}

#[inline]
pub fn down(bb: u64, stop: u64) -> u64 {
    let mut out = 0;
    let mut mask = 0x0101010101010101;

    for i in 0..8 {
        let masked = bb & mask;
        let first_bit =
            FIRST_BIT[helper::mask_to_byte(masked, mask as u8) as usize].saturating_sub(1);
        out |= (masked != 0) as u64 * helper::shr(mask, (8 - first_bit) * 8);
        mask <<= 1;
    }

    out
}

#[inline]
pub fn left(bb: u64, stop: u64) -> u64 {
    let mut out = 0;
    let mut mask = 0xff;

    for i in 0..8 {
        let masked = (bb & mask) >> (i * 8);
        let first_bit = FIRST_BIT[masked as usize].saturating_sub(1);

        out |= (masked != 0) as u64 * shift::left(mask, first_bit);
        mask <<= 8;
    }

    out
}

#[inline]
pub fn right(bb: u64, stop: u64) -> u64 {
    let mut out = 0;
    let mut mask = 0xff;

    for i in 0..8 {
        let masked = (bb & mask) >> (i * 8);
        let first_bit = FIRST_BIT[masked as usize].saturating_sub(1);

        out |= (masked != 0) as u64 * shift::right(mask, 8 - first_bit);
        mask <<= 8;
    }

    out
}

#[inline]
pub fn diag(bb: u64, stop: u64) -> u64 {
    let mut out = 0;
    let mut mask = 0x102040810204080;

    for i in 0..8 {
        let masked = bb & mask;
        let first_bit = FIRST_BIT[helper::mask_to_byte(masked, mask as u8) as usize];

        out |= (masked != 0) as u64 * shift::up_right(mask, first_bit);
        mask = shift::left(mask, 1);
    }

    let mut mask = 0x102040810204080;

    for i in 0..7 {
        let masked = bb & mask;
        let first_bit = FIRST_BIT[helper::mask_to_byte(masked, mask as u8) as usize];

        out |= (masked != 0) as u64 * shift::up_right(mask, first_bit);
        mask = shift::right(mask, 1);
    }

    out
}

#[inline]
pub fn anti_diag(bb: u64, stop: u64) -> u64 {
    let mut out = 0;
    let mut mask = 0x8040201008040201;

    for i in 0..8 {
        let masked = bb & mask;
        let first_bit =
            FIRST_BIT[helper::mask_to_byte(masked, mask as u8) as usize].saturating_sub(1);
        out |= (masked != 0) as u64 * helper::shr(mask, (8 - first_bit) * 8);
        mask <<= 1;
    }

    out
}
