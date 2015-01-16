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
fn space_is_playable() {
    let mut board = Board::new();
    let x_token = 2;
    board.set_space(4, x_token);
    assert!(board.space_is_playable(0));
    assert!(board.space_is_playable(8));
    assert_eq!(board.space_is_playable(4), false);
    assert_eq!(board.space_is_playable(9), false);
}

