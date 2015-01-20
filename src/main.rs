use self::io::*;
use self::game::*;
use self::player::*;

mod game;
mod game_test;
mod io;
mod io_test;
mod player;
mod player_test;
mod board;
mod board_test;
mod rules;
mod rules_test;
mod minimax;
mod minimax_test;

fn main() {
    let io = ConsoleIo::new();
    let player_one = CpuPlayer::new(1);
    let player_two = HumanPlayer::new(io.clone(), 2);
    let mut game = Game::new(player_one, player_two);
    let winner = game.play();
    io.print(format!("{} wins!\n", winner));
}
