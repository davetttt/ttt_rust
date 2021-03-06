#[derive(Clone)]
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

    pub fn number_of_spaces(&self) -> usize {
        self.board.len()
    }

    pub fn render_token(&self, token: usize) -> String {
        match token {
            1 => "X".to_string(),
            2 => "O".to_string(),
            _ => "-".to_string()
        }
    }

    pub fn render_row(&self, row: &Vec<usize>) -> String {
        let mut result: String = "|".to_string();
        for space in row.iter() {
            result.push_str(format!(" {} |",
                                    self.render_token(*space)).
                                    as_slice());
        }
        result
    }

    pub fn get_rows(&self) -> Vec<Vec<usize>> {
        vec![self.get_line(vec![0, 1, 2]),
             self.get_line(vec![3, 4, 5]),
             self.get_line(vec![6, 7, 8])]
    }

    pub fn render_as_string(&self) -> String {
        let mut result: String = "+---+---+---+".to_string();

        for row in self.get_rows().iter() {
            result.push_str((format!("\n{}", self.render_row(row))).as_slice());
            result.push_str("\n+---+---+---+");
        }
        result
    }

    pub fn set_space(&mut self, space: usize, token: usize) {
        self.board[space] = token;
    }

    pub fn set_spaces(&mut self, numbers: Vec<usize>, token: usize) {
        for number in numbers.iter() {
            self.board[*number] = token;
        }
    }

    pub fn get_line(&self, numbers: Vec<usize>) -> Vec<usize> {
        let mut spaces = vec![];
        for number in numbers.iter() {
            spaces.push(self.board[*number]);
        }
        spaces
    }

    pub fn get_lines(&self) -> Vec<Vec<usize>> {
        vec![self.get_line(vec![0, 1, 2]),
             self.get_line(vec![3, 4, 5]),
             self.get_line(vec![6, 7, 8]),
             self.get_line(vec![0, 3, 6]),
             self.get_line(vec![1, 4, 7]),
             self.get_line(vec![2, 5, 8]),
             self.get_line(vec![0, 4, 8]),
             self.get_line(vec![2, 4, 6])]
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

    pub fn is_full(&self) -> bool {
        if self.empty_spaces().len() == 0 {
            true
        } else {
            false
        }
    }

    pub fn space_is_playable(&self, space: usize) -> bool {
        let mut is_empty = false;
        if space < self.board.len() && self.board[space] == 0 {
            is_empty = true;
        }
        is_empty
    }

    pub fn count_tokens(&self, token: usize) -> usize {
        let mut token_count = 0;
        for space in self.board.iter() {
            if *space == token {
                token_count += 1;
            }
        }
        token_count
    }
}
