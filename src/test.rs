use std::fmt::Write;

use crate::*;

fn fmt_bb(title: &str, bb: u64) -> String {
    let mut s = String::new();

    writeln!(s, "== {title} ==").unwrap();
    let board = helper::bitboard_to_grid(bb);
    for row in board.into_iter().rev() {
        for bit in row.into_iter().rev() {
            write!(s, "{} ", if bit { '#' } else { '.' }).unwrap();
        }
        writeln!(s).unwrap();
    }
    writeln!(s).unwrap();

    s
}

fn print(title: &str, bb: u64) {
    println!("{}", fmt_bb(title, bb));
}

#[test]
fn indev() {
    let board = bit::xy(1, 1) | bit::xy(3, 5);
    let stop = bit::xy(1, 4) | bit::xy(6, 5);

    let board =
        board::Board::fen("rnbqkbnr/ppp1pppp/3p4/1Q6/8/8/PPPPPPPP/RNB1KBNR b KQkq - 1 1").unwrap();
    println!("{board}");

    let m = "b8c6";
    let (f, t) = uci::two(m).unwrap();
    println!("{m} legal: {}", board.legal(f, t));

    let f = board.future(f, t).unwrap();
    print("coverage", board.coverage(Color::White));
}

macro_rules! assert_bitboard {
    ( $left:expr, $right:expr ) => {
        let left = $left;
        let right = $right;
        assert_eq!(
            left,
            right,
            "\n{}{}on {}",
            fmt_bb("expected", right),
            fmt_bb("got", left),
            stringify!($left),
        );
    };
}

#[test]
fn sliding() {
    let board = bit::xy(1, 1) | bit::xy(3, 5);
    assert_bitboard!(slide::up(board, 0) & board, 0);
    assert_bitboard!(slide::down(board, 0) & board, 0);
    assert_bitboard!(slide::left(board, 0) & board, 0);
    assert_bitboard!(slide::right(board, 0) & board, 0);

    let stop = bit::xy(1, 4) | bit::xy(6, 5);
    assert_bitboard!(slide::up(board, stop), 0x808000202020000);
    assert_bitboard!(slide::down(board, stop), 0x80808080a);
    assert_bitboard!(slide::left(board, stop), 0x70000000fc00);
    assert_bitboard!(slide::right(board, stop), 0x70000000100);

    let board = bit::xy(1, 1) | bit::xy(3, 5);
    assert_bitboard!(slide::up(board, 0) & board, 0);
    assert_bitboard!(slide::down(board, 0) & board, 0);
    assert_bitboard!(slide::left(board, 0) & board, 0);
    assert_bitboard!(slide::right(board, 0) & board, 0);

    let stop = bit::xy(1, 3) | bit::xy(6, 6);
    assert_bitboard!(slide::diag_left(board, stop), 0x1020408004);
    assert_bitboard!(slide::diag_right(board, stop), 0x204000000010000);
    assert_bitboard!(slide::anti_diag_right(board, stop), 0x2050201008040000);
    assert_bitboard!(slide::anti_diag_left(board, stop), 0x402000001);
}
