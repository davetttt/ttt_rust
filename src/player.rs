use super::io::*;
use super::board::*;
use super::minimax::*;

pub trait Player {
    fn get_move(&self, mut board: Board) -> usize;
}

pub struct CpuPlayer;

impl CpuPlayer {
    pub fn new() -> CpuPlayer {
        CpuPlayer
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
}

pub struct HumanPlayer<I: Io> {
    io: I,
}

impl<I: Io> HumanPlayer<I> {
    pub fn new(io: I) -> HumanPlayer<I> {
        HumanPlayer { io: io }
    }
}

impl<I: Io> Player for HumanPlayer<I> {
    fn get_move(&self, mut board: Board) -> usize {
        self.io.print(board.render_as_string() + "\n");

        let response = self.io.prompt("Enter move: ".to_string());
        let space: Option<usize> = response.trim().parse();

        if space.is_some() && board.space_is_playable(space.unwrap()) {
            space.unwrap()
        } else {
            self.io.print("Bad move!\n".to_string());
            self.get_move(board)
        }
    }
}

