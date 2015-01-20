use super::game::*;
use super::player::*;

#[test]
fn game_between_two_cpu_players_results_in_a_draw() {
    let player_one = CpuPlayer::new(1);
    let player_two = CpuPlayer::new(2);
    let mut game = Game::new(player_one, player_two);
    let winner = game.play();
    assert_eq!(winner, 0);
}
