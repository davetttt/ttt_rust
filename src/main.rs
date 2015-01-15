use self::io::*;
use self::game::*;

mod game;
mod game_test;
mod io;
mod io_test;

fn main() {
    println!("tic tac toe");
    let io: ConsoleIo = Io::new();
    let mut game: Game<ConsoleIo> = Game::new(io);

}
