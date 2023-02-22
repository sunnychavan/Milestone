use std::{
    collections::HashMap,
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

pub fn piece_differential(state: &State) -> i8 {
    let current_player_num = state
        .board
        .current_players_pieces(state.current_turn)
        .len()
        .try_into()
        .unwrap_or(0);
    let opponent_num = state
        .board
        .current_players_pieces(1 - state.current_turn)
        .len()
        .try_into()
        .unwrap_or(0);

    unsigned100_normalize(-10, 10, current_player_num - opponent_num)
}

pub fn middle_proximity(state: &State) -> i8 {
    let mut middle_proximity: HashMap<usize, i64> = HashMap::new();
    middle_proximity.insert(0, 3);
    middle_proximity.insert(4, 3);
    middle_proximity.insert(11, 3);
    middle_proximity.insert(18, 3);
    middle_proximity.insert(25, 3);
    middle_proximity.insert(32, 3);
    middle_proximity.insert(36, 3);

    middle_proximity.insert(1, 2);
    middle_proximity.insert(2, 2);
    middle_proximity.insert(7, 2);
    middle_proximity.insert(8, 2);
    middle_proximity.insert(14, 2);
    middle_proximity.insert(15, 2);
    middle_proximity.insert(21, 2);
    middle_proximity.insert(22, 2);
    middle_proximity.insert(28, 2);
    middle_proximity.insert(29, 2);
    middle_proximity.insert(34, 2);
    middle_proximity.insert(35, 2);

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
        .current_players_pieces(state.current_turn)
        .iter()
        .map(|&elt| middle_proximity.get(&elt).unwrap_or(&0))
        .sum();

    unsigned100_normalize(0, 55, raw_val)
}

pub fn win_lose_condition(state: &State) -> i8 {
    let current_player = &state.players[state.current_turn as usize];

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
