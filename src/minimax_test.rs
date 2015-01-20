use super::minimax::*;
use super::board::*;

#[test]
fn get_score_gets_scores() {
    let empty_board = Board::new();
    assert_eq!(get_score(&empty_board), 0);

    let mut x_won_board = Board::new();
    x_won_board.set_spaces(vec![0, 1, 2], 1);
    assert_eq!(get_score(&x_won_board), 10);

    let mut o_won_board = Board::new();
    o_won_board.set_spaces(vec![2, 5, 8], 2);
    assert_eq!(get_score(&o_won_board), -10);
}

#[test]
fn get_token_for_empty_board_returns_X() {
    let board = Board::new();
    assert_eq!(get_token(&board), 1); // X goes first
}

#[test]
fn get_token_returns_O_when_its_Os_turn() {
    let mut board = Board::new();
    board.set_spaces(vec![0, 1, 5], 1);
    board.set_spaces(vec![2, 3], 2);
    assert_eq!(get_token(&board), 2); // O's turn
}

#[test]
fn find_max_and_find_min() {
    let scores = vec![1, -25, 3, 134, 0];
    assert_eq!(find_max_index(&scores), 3);
    assert_eq!(find_min_index(&scores), 1);
}

#[test]
fn minimax_returns_tuple_of_space_to_score() {
    let mut board = Board::new();
    board.set_spaces(vec![6, 8], 1);
    board.set_spaces(vec![0, 4], 2);
    //  2 | 0 | 0
    // ---+---+---
    //  0 | 2 | 0
    // ---+---+---
    //  1 | 0 | 1
    let space_to_score = minimax(board);
    let space = match space_to_score.0 {
        Ok(x)  => x,
        Err(e) => panic!(e),
    };
    let score = space_to_score.1;
    assert_eq!(space, 7);
    assert_eq!(score, 9);
}

#[test]
fn minimax_returns_Err_to_score_if_game_is_over() {
    let mut board = Board::new();
    board.set_spaces(vec![0, 1, 2], 1);

    let space_to_score = minimax(board);
    let space = match space_to_score.0 {
        Ok(x)  => x,
        Err(e) => 999,
    };
    let score = space_to_score.1;
    assert_eq!(space, 999);
    assert_eq!(score, 10);
}
