use super::io::*;
use self::board::*;

mod board;
mod board_test;

pub struct Game<I: Io> {
    board: Board,
    io: I,
}

impl<I: Io> Game<I> {
    pub fn new(io: I) -> Game<I> {
        Game { board: Board::new(), io: io }
    }
}

