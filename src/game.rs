use super::player::*;
use super::board::*;
use super::rules::*;

pub struct Game<P: Player> {
    board: Board,
    player: P,
}

impl<P: Player> Game<P> {
    pub fn new(player: P) -> Game<P> {
        Game { board: Board::new(), player: player }
    }

    fn game_loop(&mut self) {
        if game_is_over(&self.board) {
            return;
        } else {
        let space = self.player.get_move(self.board.clone());
        self.board.set_space(space, 1);
        self.game_loop();}
    }

    pub fn start(&mut self) {
        self.game_loop();
    }
}

