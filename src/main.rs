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
    io.print("Hello! Welcome to Tic Tac Toe.\n".to_string());

    let mut game_config = 3;

    loop {
        io.print("Please choose one of the following options:\n".to_string());

        let response = io.prompt(concat!(
                    "1) You vs. the game (you go first)\n",
                    "2) The game vs. you (the game goes first)\n",
                    "3) You vs. a friend\n").to_string());

        let choice = response.trim().parse();

        if choice.is_some() && [1, 2, 3].contains(&choice.unwrap()) {
            game_config = choice.unwrap();
            break;
        }
    }

    let winner = match game_config {
        1     => Game::new(HumanPlayer::new(io.clone(), 1),
                           CpuPlayer::new(2)).play(),
        2     => Game::new(CpuPlayer::new(1),
                           HumanPlayer::new(io.clone(), 2)).play(),
        3 | _ => Game::new(HumanPlayer::new(io.clone(), 1),
                           HumanPlayer::new(io.clone(), 2)).play()
    };

    io.print(format!("{} wins!\n", winner));
}
