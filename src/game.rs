use super::player::*;
use super::board::*;
use super::rules::*;

pub struct Game<P1: Player, P2: Player> {
    board: Board,
    player_one: P1,
    player_two: P2,
}

impl<P1: Player, P2: Player> Game<P1, P2> {
    pub fn new(player_one: P1, player_two: P2) -> Game<P1, P2> {
        Game {
            board: Board::new(),
            player_one: player_one,
            player_two: player_two,
        }
    }

    /* returns winning token */
    pub fn play(&mut self) -> usize {
        if game_is_over(&self.board) {
            return get_winner(&self.board)
        } else {
            let space = self.player_one.get_move(self.board.clone());
            self.board.set_space(space, self.player_one.get_token());
        }
        if game_is_over(&self.board) {
            return get_winner(&self.board)
        } else {
            let space = self.player_two.get_move(self.board.clone());
            self.board.set_space(space, self.player_two.get_token());
        }
        self.play()
    }
}

