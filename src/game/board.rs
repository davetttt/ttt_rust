pub struct Board {
    board: [usize; 9]
}

impl Board {
    pub fn new() -> Board {
        Board { board:[0us; 9] }
    }

    pub fn get_board(&self) -> [usize; 9] {
        self.board
    }

    pub fn print_board(&self) {
        for space in self.board.iter() {
            print!("{}", *space);
        }
        println!("");
    }

    pub fn set_space(&mut self, space: usize, token: usize) {
        self.board[space] = token;
    }

    pub fn empty_spaces(&self) -> Vec<usize> {
        let mut empties = vec![];
        for space in 0..9 {
            if self.board[space] == 0 {
                empties.push(space);
            }
        }
        empties
    }

    pub fn space_is_playable(&self, space: usize) -> bool {
        let mut is_empty = false;
        if space < self.board.len() && self.board[space] == 0 {
            is_empty = true;
        }
        is_empty
    }
}
