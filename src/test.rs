use crate::*;

fn print(title: &str, board: u64) {
    println!("== {title} ==");
    print_bitboard(board);
}

#[test]
fn indev() {
    let board = bit::xy(1, 1) | bit::xy(3, 5);
    let stop = bit::xy(1, 4) | bit::xy(7, 5);

    print("original", board);
    print("stop", stop);
    print("up", slide::up(board, stop));
}

#[test]
fn sliding() {
    let board = bit::xy(1, 1) | bit::xy(3, 5);
    assert!(slide::up(board, 0) & board == 0);
    assert!(slide::down(board, 0) & board == 0);
    assert!(slide::left(board, 0) & board == 0);
    assert!(slide::right(board, 0) & board == 0);
}

