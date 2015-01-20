use super::board::*;

#[test]
fn new_board_is_empty() {
    let board = Board::new();
    assert_eq!(board.get_board(), [0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn set_space() {
    let mut board = Board::new();
    let (x_token, o_token) = (1, 2);
    board.set_space(0, x_token);
    board.set_space(5, o_token);
    assert_eq!(board.get_board(), [1, 0, 0, 0, 0, 2, 0, 0, 0]);
}

#[test]
fn set_spaces() {
    let mut board = Board::new();
    let x_token = 1;
    board.set_spaces(vec![0, 2, 5], x_token);
    assert_eq!(board.get_board(), [1, 0, 1, 0, 0, 1, 0, 0, 0]);
}

#[test]
fn empty_spaces() {
    let mut board = Board::new();
    let (x_token, o_token) = (1, 2);
    board.set_space(0, x_token);
    board.set_space(1, o_token);
    board.set_space(4, x_token);
    board.set_space(7, o_token);
    assert_eq!(board.empty_spaces(), [2, 3, 5, 6, 8]);
}

#[test]
fn get_line() {
    let mut board = Board::new();
    let (x_token, o_token) = (1, 2);
    board.set_space(0, x_token);
    board.set_space(1, o_token);
    assert_eq!(board.get_line(vec![0, 1, 2]), [1, 2, 0]);
}

#[test]
fn space_is_playable() {
    let mut board = Board::new();
    let x_token = 2;
    board.set_space(4, x_token);
    assert!(board.space_is_playable(0));
    assert!(board.space_is_playable(8));
    assert_eq!(board.space_is_playable(4), false);
    assert_eq!(board.space_is_playable(9), false);
}

#[test]
fn is_full_returns_false_when_not_full() {
    let mut board = Board::new();
    let (x_token, o_token) = (1, 2);
    board.set_space(0, x_token);
    board.set_space(1, o_token);
    assert_eq!(board.is_full(), false);
}

#[test]
fn is_full_returns_true_when_full() {
    let mut board = Board::new();
    let x_token = 1;
    for space in 0..9 {
        board.set_space(space, x_token);
    }
    assert_eq!(board.is_full(), true);
}

#[test]
fn returns_rows_columns_diagonals_as_lines() {
    let mut board = Board::new();
    let (x_token, o_token) = (1, 2);
    board.set_space(0, x_token);
    board.set_space(1, o_token);
    board.set_space(4, x_token);
    board.set_space(7, o_token);
    //  1 | 2 | 0
    // ---+---+---
    //  0 | 1 | 0
    // ---+---+---
    //  0 | 2 | 0
    assert_eq!(board.get_lines(),
               [[1, 2, 0], [0, 1, 0], [0, 2, 0], // rows
                [1, 0, 0], [2, 1, 2], [0, 0, 0], // columns
                [1, 1, 0], [0, 1, 0]]);          // diagonals
}

#[test]
fn count_tokens_returns_count_of_specified_token() {
    let mut board = Board::new();
    assert_eq!(board.count_tokens(1), 0);
    assert_eq!(board.count_tokens(2), 0);

    board.set_spaces(vec![0, 1, 5], 1);
    board.set_spaces(vec![2, 3], 2);
    assert_eq!(board.count_tokens(1), 3);
    assert_eq!(board.count_tokens(2), 2);
}
