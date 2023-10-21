use crate::*;

fn print(title: &str, board: u64) {
    println!("== {title} ==");
    print_bitboard(board);
}

#[test]
fn indev() {
    let board = bit::xy(1, 1) | bit::xy(3, 5);
    let stop = bit::xy(1, 4) | bit::xy(6, 5);

    let board = board::Board::new();
    println!("{board}");
    print("black coverage", board.coverage(board::Color::Black));
}

#[test]
fn sliding() {
    let board = bit::xy(1, 1) | bit::xy(3, 5);
    assert_eq!(slide::up(board, 0) & board, 0);
    assert_eq!(slide::down(board, 0) & board, 0);
    assert_eq!(slide::left(board, 0) & board, 0);
    assert_eq!(slide::right(board, 0) & board, 0);

    let stop = bit::xy(1, 4) | bit::xy(6, 5);
    assert_eq!(slide::up(board, stop), 0x808000202020000);
    assert_eq!(slide::down(board, stop), 0x80808080a);
    assert_eq!(slide::left(board, stop), 0x70000000fc00);
    assert_eq!(slide::right(board, stop), 0x70000000100);
}

