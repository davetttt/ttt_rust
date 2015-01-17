use super::board::*;

pub fn every(collection: Vec<usize>, value: usize) -> bool {
    let mut answer = false;
    let mut count = 0;
    for item in collection.iter() {
        if *item == value {
            count += 1;
        }
    }
    if count == collection.len() {
        answer = true
    }
    answer
}

pub fn winner_for_line(line: Vec<usize>) -> usize {
    let mut winner = 0;
    if every(line.to_vec(), 1) {
        winner = 1;
    } else if every(line.to_vec(), 2) {
        winner = 2;
    }
    winner
}

pub fn get_winner(board: Board) -> usize {
    let mut winner = 0;
    let lines = board.get_lines();
    for line in lines.iter() {
        winner = winner_for_line(line.to_vec());
        if winner != 0 {
            break;
        }
    }
    winner
}

