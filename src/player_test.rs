use super::player::*;
use super::io::*;
use super::board::*;

#[test]
fn get_move_returns_move_when_move_is_valid() {
    let io = TestIo::new("3".to_string());
    let player = HumanPlayer::new(io);
    let board = Board::new();
    assert_eq!(player.get_move(board), 3);
}

