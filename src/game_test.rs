use super::game::*;
use super::io::*;

#[test]
fn can_create_game_with_test_io() {
    let io: TestIo = Io::new();
    let mut game = Game::new(io);
}
