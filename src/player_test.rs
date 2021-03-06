use super::player::*;
use super::io::*;
use super::board::*;

#[test]
fn human_player_get_move_returns_zero_based_move_when_valid() {
    // Typing in 1 to choose the first space
    // is more natural than typing 0
    let io = TestIo::new("3".to_string());
    let human_player = HumanPlayer::new(io, 1);
    let board = Board::new();
    assert_eq!(human_player.get_move(board), 2);
}

#[test]
fn cpu_player_chooses_last_valid_move() {
    let cpu_player = CpuPlayer::new(1);
    let mut board = Board::new();
    board.set_spaces(vec![0, 1, 5, 8], 1);
    board.set_spaces(vec![2, 3, 4, 7], 2);
    assert_eq!(cpu_player.get_move(board), 6);
}

#[test]
fn cpu_player_wins_immediately_if_possible() {
    let cpu_player = CpuPlayer::new(2);
    let mut board = Board::new();
    board.set_spaces(vec![1, 3, 5], 1);
    board.set_spaces(vec![0, 4], 2);
    //  2 | 1 | 0
    // ---+---+---
    //  1 | 2 | 1
    // ---+---+---
    //  0 | 0 | 0
    assert_eq!(cpu_player.get_move(board), 8);
}

