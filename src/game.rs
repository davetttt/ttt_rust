use super::player::*;
use super::board::*;

pub struct Game<P: Player> {
    board: Board,
    player: P,
}

impl<P: Player> Game<P> {
    pub fn new(player: P) -> Game<P> {
        Game { board: Board::new(), player: player }
    }

    fn next_turn(&mut self) {
        let space = self.player.get_move(&self.board);
        self.board.set_space(space, 1);
        self.next_turn();
    }

    pub fn start(&mut self) {
        self.next_turn();
    }
}

