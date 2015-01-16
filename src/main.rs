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

fn main() {
    let io = ConsoleIo::new();
    let human_player = HumanPlayer::new(io);
    let mut game = Game::new(human_player);
    game.start();
}
