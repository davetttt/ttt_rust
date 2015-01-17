use super::rules::*;
use super::board::*;

#[test]
fn every_works_as_expected() {
    assert!(every(vec![2, 2, 2], 2));
    assert_eq!(every(vec![0, 1, 2], 1), false);
}

#[test]
fn winner_for_line_works_as_expected() {
    assert_eq!(winner_for_line(vec![1, 1, 1]), 1);
    assert_eq!(winner_for_line(vec![2, 2, 2]), 2);
    assert_eq!(winner_for_line(vec![0, 1, 2]), 0);
}

#[test]
fn get_winner_returns_zero_if_no_winner() {
    let mut board = Board::new();
    let winner = get_winner(board);
    assert_eq!(winner, 0);
}

#[test]
fn get_winner_returns_one_if_one_has_row_win() {
    let mut board = Board::new();
    for space in 0..3 {
        board.set_space(space, 1);
    }
    let winner = get_winner(board);
    assert_eq!(winner, 1);
}

#[test]
fn get_winner_returns_two_if_two_has_column_win() {
    let mut board = Board::new();
    board.set_space(0, 2);
    board.set_space(4, 2);
    board.set_space(8, 2);
    let winner = get_winner(board);
    assert_eq!(winner, 2);
}

#[test]
fn get_winner_returns_one_if_one_has_diagonal_win() {
    let mut board = Board::new();
    board.set_space(2, 1);
    board.set_space(4, 1);
    board.set_space(6, 1);
    let winner = get_winner(board);
    assert_eq!(winner, 1);
}
