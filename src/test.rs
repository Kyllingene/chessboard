use crate::*;

#[test]
fn indev() {
    let board = bit::xy(1, 1) | bit::xy(3, 5);
    print_bitboard(board);
    print_bitboard(mask::right(3));
    return;
    print_bitboard(slide::left(board, 0));
}
