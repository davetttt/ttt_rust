use super::board::*;
use super::rules::*;

pub fn get_score(mut board: &Board) -> isize {
    let mut score = (board.number_of_spaces() + 1) as isize;
    match get_winner(board) {
        1 => score,
        2 => -score,
        _ => 0,
    }
}

pub fn get_token(mut board: &Board) -> usize {
    let mut token = 1;
    if board.count_tokens(1) > board.count_tokens(2) {
        token = 2;
    }
    token
}

pub fn get_weighted(score: isize) -> isize {
    if score > 0 {
        score - 1
    } else if score < 0 {
        score + 1
    } else {
        score
    }
}

pub fn find_max_index(scores: &Vec<isize>) -> usize {
    let mut max_index = 0;
    let mut max_score = scores[0];
    for index in range(0, scores.len()) {
        if scores[index] > max_score {
            max_index = index;
            max_score = scores[index];
        }
    }
    max_index
}

pub fn find_min_index(scores: &Vec<isize>) -> usize {
    let mut min_index = 0;
    let mut min_score = scores[0];
    for index in range(0, scores.len()) {
        if scores[index] < min_score {
            min_index = index;
            min_score = scores[index];
        }
    }
    min_index
}

/* returns a tuple: (chosen_space, game_score) */
pub fn minimax(mut board: Board) -> (Result<usize, String>, isize) {
    if game_is_over(&board) {
        (Err("Asked minimax to choose a move in finished game".to_string()),
         get_score(&board))
    } else {
        let token = get_token(&board);
        let spaces = board.empty_spaces();
        let mut scores = vec![];
        let mut index = 0;

        for space in spaces.iter() {
            board.set_space(*space, token);
            scores.push(get_weighted(minimax(board.clone()).1));
            board.set_space(*space, 0);
        }
        index = match token {
            1 => find_max_index(&scores),
            _ => find_min_index(&scores),
        };
        (Ok(spaces[index]), scores[index])
    }
}

