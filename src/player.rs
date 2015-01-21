use super::io::*;
use super::board::*;
use super::minimax::*;

pub trait Player {
    fn get_move(&self, mut board: Board) -> usize;
    fn get_token(&self) -> usize;
}

pub struct CpuPlayer {
    token: usize
}

impl CpuPlayer {
    pub fn new(token: usize) -> CpuPlayer {
        CpuPlayer { token: token }
    }
}

impl Player for CpuPlayer {
    fn get_move(&self, mut board: Board) -> usize {
        let chosen_space = match minimax(board).0 {
           Ok(space) => space,
           Err(e)    => panic!(e),
        };
        chosen_space
    }

    fn get_token(&self) -> usize {
        self.token
    }
}

pub struct HumanPlayer<I: Io> {
    io: I,
    token: usize
}

impl<I: Io> HumanPlayer<I> {
    pub fn new(io: I, token: usize) -> HumanPlayer<I> {
        HumanPlayer { io: io, token: token }
    }
}

impl<I: Io> Player for HumanPlayer<I> {
    fn get_move(&self, mut board: Board) -> usize {
        self.io.print(board.render_as_string() + "\n");

        let response = self.io.prompt("Enter move: ".to_string());
        let space: Option<usize> = response.trim().parse();

        if space.is_some() && board.space_is_playable(space.unwrap() - 1) {
            space.unwrap() - 1
        } else {
            self.io.print("Bad move!\n".to_string());
            self.get_move(board)
        }
    }

    fn get_token(&self) -> usize {
        self.token
    }
}

