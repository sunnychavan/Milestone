use std::{
    collections::{HashMap, HashSet},
    ops::{Div, Mul},
};

use crate::game::{
    board::Hole, gamestate::State, pieces::Piece::Black, pieces::Piece::White,
    player::Player,
};

pub fn number_of_pieces(state: &State) -> i8 {
    let raw_score = state
        .board
        .current_players_pieces(state.current_turn)
        .len()
        .try_into()
        .unwrap_or(0);

    unsigned100_normalize(0, 10, raw_score)
}

pub fn piece_differential(state: &State, turn:u8) -> i8 {
    let current_player_num = state
        .board
        .current_players_pieces(turn)
        .len()
        .try_into()
        .unwrap_or(0);
    let opponent_num = state
        .board
        .current_players_pieces(1 - turn)
        .len()
        .try_into()
        .unwrap_or(0);

    unsigned100_normalize(-10, 10, current_player_num - opponent_num)
}

pub fn hold_important_pieces(state: &State, turn:u8) -> i8 {
    let mut important_pieces: HashMap<usize, i64> = HashMap::new();
    let current_player = &state.players[turn as usize];

    match current_player.get_pieces_type() {
        Black => {
            important_pieces.insert(0, 3);
            important_pieces.insert(1, 2);
            important_pieces.insert(2, 2);
        },
        White => {
            important_pieces.insert(36, 3);
            important_pieces.insert(34, 2);
            important_pieces.insert(35, 2);
        }
        }
        let raw_val = state
        .board
        .current_players_pieces(turn)
        .iter()
        .map(|&elt| important_pieces.get(&elt).unwrap_or(&0))
        .sum();

    unsigned100_normalize(0, 7, raw_val)
    }
    
pub fn middle_proximity(state: &State, turn:u8) -> i8 {
    let mut middle_proximity: HashMap<usize, i64> = HashMap::new();
    middle_proximity.insert(0, 6);
    middle_proximity.insert(4, 6);
    middle_proximity.insert(11, 6);
    middle_proximity.insert(18, 6);
    middle_proximity.insert(25, 6);
    middle_proximity.insert(32, 6);
    middle_proximity.insert(36, 6);

    middle_proximity.insert(1, 3);
    middle_proximity.insert(2, 3);
    middle_proximity.insert(7, 3);
    middle_proximity.insert(8, 3);
    middle_proximity.insert(14, 3);
    middle_proximity.insert(15, 3);
    middle_proximity.insert(21, 3);
    middle_proximity.insert(22, 3);
    middle_proximity.insert(28, 3);
    middle_proximity.insert(29, 3);
    middle_proximity.insert(34, 3);
    middle_proximity.insert(35, 3);

    middle_proximity.insert(3, 1);
    middle_proximity.insert(5, 1);
    middle_proximity.insert(10, 1);
    middle_proximity.insert(12, 1);
    middle_proximity.insert(17, 1);
    middle_proximity.insert(19, 1);
    middle_proximity.insert(24, 1);
    middle_proximity.insert(26, 1);
    middle_proximity.insert(31, 1);
    middle_proximity.insert(33, 1);

    middle_proximity.insert(6, 0);
    middle_proximity.insert(9, 0);
    middle_proximity.insert(13, 0);
    middle_proximity.insert(16, 0);
    middle_proximity.insert(20, 0);
    middle_proximity.insert(23, 0);
    middle_proximity.insert(27, 0);
    middle_proximity.insert(30, 0);

    let raw_val = state
        .board
        .current_players_pieces(turn)
        .iter()
        .map(|&elt| middle_proximity.get(&elt).unwrap_or(&0))
        .sum();

    unsigned100_normalize(lowerbound_middle_proximity(), upperbound_middle_proximity(6, 3), raw_val)
}

pub fn middle_piece_differential(state: &State, turn:u8) -> i8 {
    let mut middle_pieces = HashSet::new();
    middle_pieces.insert(0);
    middle_pieces.insert(4);
    middle_pieces.insert(11);
    middle_pieces.insert(18);
    middle_pieces.insert(25);
    middle_pieces.insert(32);
    middle_pieces.insert(36);

    let current_player_num: i64 = state
    .board
    .current_players_pieces(turn)
    .iter()
    .map(|&elt| {
        if middle_pieces.contains(&elt){
            1
        }
        else{
            0
        }
    } ).sum();

    let opponent_player_num: i64 = state
    .board
    .current_players_pieces(1 - turn)
    .iter()
    .map(|&elt| {
        if middle_pieces.contains(&elt){
            1
        }
        else{
            0
        }
    } ).sum();
    unsigned100_normalize(-7, 7, current_player_num - opponent_player_num)
}

pub fn win_lose_condition(state: &State, turn:u8) -> i8 {
    let current_player = &state.players[turn as usize];

    // don't need to normalize
    let blacks_home = state.board.board[0];
    let whites_home = state.board.board[36];
    match current_player.get_pieces_type() {
        Black => match (blacks_home, whites_home) {
            (Hole(Some(White)), _) => -100,
            (_, Hole(Some(Black))) => 100,
            _ => 0,
        },
        White => match (blacks_home, whites_home) {
            (Hole(Some(White)), _) => 100,
            (_, Hole(Some(Black))) => -100,
            _ => 0,
        },
    }
}

fn unsigned100_normalize(min: i64, max: i64, value: i64) -> i8 {
    //  ((2 * (value - lb)) / (ub - lb)) - 1) * 100
    let numerator = 100 * 2 * (value - min);
    let denominator = max - min;

    i8::try_from(numerator.div(denominator) - 100)
        .expect("downcasting to a i8 failed")
}


// HEURISTIC HELPER FUNCTIONS ( E.G. LOWERBOUND UPPERBOUND CALCULATORS ) //

pub fn lowerbound_middle_proximity() -> i64 {
    0
}

pub fn upperbound_middle_proximity(middle_weight: i64, next_middle_weight: i64 ) -> i64 {
    7*middle_weight + 3*next_middle_weight
}
