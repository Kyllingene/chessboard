use crate::*;

fn print(title: &str, board: u64) {
    println!("== {title} ==");
    print_bitboard(board);
}

#[test]
fn indev() {
    let board = bit::xy(1, 1) | bit::xy(3, 5);
    let stop = bit::xy(1, 4) | bit::xy(6, 5);

    let board =
    board::Board::fen("rnbqkbnr/ppp1pppp/3p4/1Q6/8/8/PPPPPPPP/RNB1KBNR b KQkq - 1 1").unwrap();
    println!("{board}");


    let (f, t) = uci::two("g8f6").unwrap();
    println!("g8f6 legal: {}", board.legal(f, t));
    println!("g8f6: {:?}", board.get(f.0, f.1).unwrap());
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
