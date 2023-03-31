//! all heuristics are oriented to provide high scores when black is in a good
//! situation, and low (negative) scores for white advantaged positions

#![allow(non_camel_case_types)]

use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    iter::zip,
    ops::Div,
};

use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;

use crate::game::{
    board::{Hole, Move},
    gamestate::State,
    pieces::Piece::Black,
    pieces::Piece::White,
};

use super::location_maps::{
    anti_centrality, black_proximity, black_proximity_row, centrality,
    middle_proximity, white_proximity, white_proximity_row,
};

#[enum_dispatch]
#[derive(Clone)]
enum Heuristics {
    PieceDifferential,
    MiddleLineDifferential,
    ImportantPieces,
    MiddleProximity,
    AggressionDifferential,
    PassivenessDifferential,
    Centrality,
    AntiCentrality,
    DefendedHexes,
    DefendedHexes_MiddleProximity,
    UndefendedPieces,
    UndefendedPieces_MiddleProximity,
    AttackTiming,
    LimitOppoMoves,
    StraightLines,
    StraightLines_MiddleProximity,
    AggressivePieces,
    AggressivePieces_MiddleProximity,
    AggressivePieces_AntiCentrality,
}

pub const NUM_HEURISTICS: usize = 19;

#[enum_dispatch(Heuristics)]
trait Heuristic {
    fn score(&self, state: &State) -> i64;

    fn name(&self) -> &'static str;
}

#[derive(Clone)]
struct PieceDifferential;

impl Heuristic for PieceDifferential {
    fn score(&self, state: &State) -> i64 {
        let black_score = state
            .board
            .current_players_pieces(0)
            .len()
            .try_into()
            .unwrap_or(0);
        let white_score = state
            .board
            .current_players_pieces(1)
            .len()
            .try_into()
            .unwrap_or(0);

        unsigned100_normalize(-10, 10, black_score - white_score)
    }

    fn name(&self) -> &'static str {
        "Piece Diff"
    }
}

#[derive(Clone)]
struct MiddleLineDifferential;

impl Heuristic for MiddleLineDifferential {
    fn score(&self, state: &State) -> i64 {
        lazy_static! {
            static ref MIDDLE_PIECES: HashSet<usize> = {
                let mut middle_pieces = HashSet::new();
                middle_pieces.insert(0);
                middle_pieces.insert(4);
                middle_pieces.insert(11);
                middle_pieces.insert(18);
                middle_pieces.insert(25);
                middle_pieces.insert(32);
                middle_pieces.insert(36);

                middle_pieces
            };
        }

        let black_score: i64 = state
            .board
            .current_players_pieces(0)
            .iter()
            .map(|&elt| if MIDDLE_PIECES.contains(&elt) { 1 } else { 0 })
            .sum();

        let white_score: i64 = state
            .board
            .current_players_pieces(1)
            .iter()
            .map(|&elt| if MIDDLE_PIECES.contains(&elt) { 1 } else { 0 })
            .sum();
        unsigned100_normalize(-7, 7, black_score - white_score)
    }

    fn name(&self) -> &'static str {
        "Middle Line Diff"
    }
}

#[derive(Clone)]
struct ImportantPieces;

impl Heuristic for ImportantPieces {
    fn score(&self, state: &State) -> i64 {
        let mut important_pieces_black: HashMap<usize, i64> = HashMap::new();
        let mut important_pieces_white: HashMap<usize, i64> = HashMap::new();

        important_pieces_black.insert(0, 3);
        important_pieces_black.insert(1, 2);
        important_pieces_black.insert(2, 2);

        important_pieces_white.insert(36, 3);
        important_pieces_white.insert(34, 2);
        important_pieces_white.insert(35, 2);

        let black_score: i64 = state
            .board
            .current_players_pieces(0)
            .iter()
            .map(|&elt| important_pieces_black.get(&elt).unwrap_or(&0))
            .sum();

        let white_score: i64 = state
            .board
            .current_players_pieces(1)
            .iter()
            .map(|&elt| important_pieces_white.get(&elt).unwrap_or(&0))
            .sum();

        unsigned100_normalize(-5, 5, black_score - white_score)
    }

    fn name(&self) -> &'static str {
        "Impt Pieces"
    }
}

#[derive(Clone)]
struct MiddleProximity;

impl Heuristic for MiddleProximity {
    fn score(&self, state: &State) -> i64 {
        let black_score: i64 = state
            .board
            .current_players_pieces(0)
            .iter()
            .map(middle_proximity)
            .sum();

        let white_score: i64 = state
            .board
            .current_players_pieces(1)
            .iter()
            .map(middle_proximity)
            .sum();

        unsigned100_normalize(
            -upperbound_middle_proximity(6, 3),
            upperbound_middle_proximity(6, 3),
            black_score - white_score,
        )
    }

    fn name(&self) -> &'static str {
        "Middle Prox"
    }
}

#[derive(Clone)]
struct AggressionDifferential;

impl Heuristic for AggressionDifferential {
    fn score(&self, state: &State) -> i64 {
        let black_score: i64 = state
            .board
            .current_players_pieces(0)
            .iter()
            .map(white_proximity)
            .sum();

        let white_score: i64 = state
            .board
            .current_players_pieces(1)
            .iter()
            .map(black_proximity)
            .sum();

        unsigned100_normalize(-34, 34, black_score - white_score)
    }

    fn name(&self) -> &'static str {
        "Aggression Diff"
    }
}

#[derive(Clone)]
struct PassivenessDifferential;

impl Heuristic for PassivenessDifferential {
    fn score(&self, state: &State) -> i64 {
        let black_score: i64 = state
            .board
            .current_players_pieces(0)
            .iter()
            .map(black_proximity)
            .sum();

        let white_score: i64 = state
            .board
            .current_players_pieces(1)
            .iter()
            .map(white_proximity)
            .sum();

        unsigned100_normalize(-34, 34, black_score - white_score)
    }

    fn name(&self) -> &'static str {
        "Passive Diff"
    }
}

#[derive(Clone)]
struct Centrality;

impl Heuristic for Centrality {
    fn score(&self, state: &State) -> i64 {
        let black_score: i64 = state
            .board
            .current_players_pieces(0)
            .iter()
            .map(centrality)
            .sum();

        let white_score: i64 = state
            .board
            .current_players_pieces(1)
            .iter()
            .map(centrality)
            .sum();

        unsigned100_normalize(-18, 18, black_score - white_score)
    }

    fn name(&self) -> &'static str {
        "Centrality"
    }
}

#[derive(Clone)]
struct AntiCentrality;

impl Heuristic for AntiCentrality {
    fn score(&self, state: &State) -> i64 {
        let black_score: i64 = state
            .board
            .current_players_pieces(0)
            .iter()
            .map(anti_centrality)
            .sum();

        let white_score: i64 = state
            .board
            .current_players_pieces(1)
            .iter()
            .map(anti_centrality)
            .sum();

        unsigned100_normalize(-30, 30, black_score - white_score)
    }

    fn name(&self) -> &'static str {
        "Anti Centrality"
    }
}

#[derive(Clone)]
struct DefendedHexes;

impl Heuristic for DefendedHexes {
    fn score(&self, state: &State) -> i64 {
        let black_pieces = state.board.current_players_pieces(0);

        let white_pieces = state.board.current_players_pieces(1);

        let black_straight_hexes = black_pieces
            .iter()
            .map(|&elt| state.board.get_straight_hex(0, elt));

        let black_score = black_straight_hexes
            .filter(|&elt| {
                match elt {
                    Some(i) => {
                        // let hex_taken = black_pieces.contains(i) || white_pieces.contains(i);
                        // !hex_taken
                        let straight_hole = state.board.board[*i];
                        match straight_hole {
                            Hole(Some(_)) => false,
                            Hole(None) => true,
                        }
                    }
                    None => false,
                }
            })
            .count();

        let white_straight_hexes = white_pieces
            .iter()
            .map(|&elt| state.board.get_straight_hex(1, elt));

        let white_score = white_straight_hexes
            .filter(|&elt| match elt {
                Some(i) => {
                    let straight_hole = state.board.board[*i];
                    match straight_hole {
                        Hole(Some(_)) => false,
                        Hole(None) => true,
                    }
                }
                None => false,
            })
            .count();
        unsigned100_normalize(
            -10,
            10,
            i64::try_from(black_score).unwrap()
                - i64::try_from(white_score).unwrap(),
        )
    }

    fn name(&self) -> &'static str {
        "Defended Hexes"
    }
}

#[derive(Clone)]
struct DefendedHexes_MiddleProximity;

impl Heuristic for DefendedHexes_MiddleProximity {
    fn score(&self, state: &State) -> i64 {
        let black_pieces = state.board.current_players_pieces(0);

        let white_pieces = state.board.current_players_pieces(1);

        let black_straight_hexes = black_pieces
            .iter()
            .map(|&elt| state.board.get_straight_hex(0, elt));

        let black_score: i64 = black_straight_hexes
            .map(|elt| match elt {
                Some(i) => {
                    let straight_hole = state.board.board[*i];
                    match straight_hole {
                        Hole(Some(_)) => 0,
                        Hole(None) => middle_proximity(i),
                    }
                }
                None => 0,
            })
            .sum();

        let white_straight_hexes = white_pieces
            .iter()
            .map(|&elt| state.board.get_straight_hex(1, elt));

        let white_score: i64 = white_straight_hexes
            .map(|elt| match elt {
                Some(i) => {
                    let straight_hole = state.board.board[*i];
                    match straight_hole {
                        Hole(Some(_)) => 0,
                        Hole(None) => middle_proximity(i),
                    }
                }
                None => 0,
            })
            .sum();

        unsigned100_normalize(-37, 37, black_score - white_score)
    }

    fn name(&self) -> &'static str {
        "Defended Hexes Middle Prox"
    }
}

#[derive(Clone)]
struct AttackTiming;

impl Heuristic for AttackTiming {
    fn score(&self, state: &State) -> i64 {
        lazy_static! {
            static ref MIDDLE_PIECES: HashSet<usize> = {
                let mut middle_pieces = HashSet::new();
                middle_pieces.insert(0);
                middle_pieces.insert(4);
                middle_pieces.insert(11);
                middle_pieces.insert(18);
                middle_pieces.insert(25);
                middle_pieces.insert(32);
                middle_pieces.insert(36);

                middle_pieces
            };
        }

        let black_pieces = state.board.current_players_pieces(0);

        let black_most_adv_mid_val = black_pieces
            .iter()
            .filter(|hex| MIDDLE_PIECES.contains(hex))
            .map(white_proximity)
            .max();

        let black_most_adv_side_val = black_pieces
            .iter()
            .filter(|hex| !MIDDLE_PIECES.contains(hex))
            .map(white_proximity)
            .max();

        let black_diff = (black_most_adv_mid_val.unwrap_or_default()
            - black_most_adv_side_val.unwrap_or_default())
        .abs();

        let white_pieces = state.board.current_players_pieces(1);

        let white_most_adv_mid_val = white_pieces
            .iter()
            .filter(|hex| MIDDLE_PIECES.contains(hex))
            .map(black_proximity)
            .max();

        let white_most_adv_side_val = white_pieces
            .iter()
            .filter(|hex| !MIDDLE_PIECES.contains(hex))
            .map(black_proximity)
            .max();

        let white_diff = (white_most_adv_mid_val.unwrap_or_default()
            - white_most_adv_side_val.unwrap_or_default())
        .abs();

        unsigned100_normalize(-10, 10, white_diff - black_diff)
    }

    fn name(&self) -> &'static str {
        "Attack in-sync"
    }
}

#[derive(Clone)]
struct LimitOppoMoves;

impl Heuristic for LimitOppoMoves {
    fn score(&self, state: &State) -> i64 {
        let black_moves = state.current_possible_moves(0).len() as i64;
        let white_moves = state.current_possible_moves(1).len() as i64;

        unsigned100_normalize(-30, 30, black_moves - white_moves)
    }

    fn name(&self) -> &'static str {
        "Limit Oppo Moves"
    }
}

#[derive(Clone)]
struct UndefendedPieces;

impl Heuristic for UndefendedPieces {
    fn score(&self, state: &State) -> i64 {
        lazy_static! {
            static ref INVALID_BLACK_DEFENDED_PIECES: HashSet<usize> = {
                let mut invalid_black_defended_pieces = HashSet::new();
                invalid_black_defended_pieces.insert(0);
                invalid_black_defended_pieces.insert(1);
                invalid_black_defended_pieces.insert(2);
                invalid_black_defended_pieces.insert(3);
                invalid_black_defended_pieces.insert(5);
                invalid_black_defended_pieces.insert(6);
                invalid_black_defended_pieces.insert(9);
                invalid_black_defended_pieces
            };
        }

        lazy_static! {
            static ref INVALID_WHITE_DEFENDED_PIECES: HashSet<usize> = {
                let mut invalid_white_defended_pieces = HashSet::new();
                invalid_white_defended_pieces.insert(27);
                invalid_white_defended_pieces.insert(30);
                invalid_white_defended_pieces.insert(31);
                invalid_white_defended_pieces.insert(33);
                invalid_white_defended_pieces.insert(34);
                invalid_white_defended_pieces.insert(35);
                invalid_white_defended_pieces.insert(36);
                invalid_white_defended_pieces
            };
        }

        let black_pieces = state.board.current_players_pieces(0);

        let num_valid_black_pieces = black_pieces
            .iter()
            .filter(|&elt| !INVALID_BLACK_DEFENDED_PIECES.contains(elt))
            .count();

        let white_pieces = state.board.current_players_pieces(1);

        let num_valid_white_pieces = white_pieces
            .iter()
            .filter(|&elt| !INVALID_WHITE_DEFENDED_PIECES.contains(elt))
            .count();

        let black_straight_hexes = black_pieces
            .iter()
            .map(|&elt| state.board.get_straight_hex(0, elt));

        let black_defended_pieces = black_straight_hexes
            .filter(|&elt| match elt {
                Some(i) => {
                    let straight_hole = state.board.board[*i];
                    match straight_hole {
                        Hole(Some(Black)) => true,
                        Hole(_) => false,
                    }
                }
                None => false,
            })
            .count();

        let black_undefended_pieces =
            num_valid_black_pieces - black_defended_pieces;

        let white_straight_hexes = white_pieces
            .iter()
            .map(|&elt| state.board.get_straight_hex(1, elt));

        let white_defended_pieces = white_straight_hexes
            .filter(|&elt| match elt {
                Some(i) => {
                    let straight_hole = state.board.board[*i];
                    match straight_hole {
                        Hole(Some(White)) => true,
                        Hole(_) => false,
                    }
                }
                None => false,
            })
            .count();

        let white_undefended_pieces =
            num_valid_white_pieces - white_defended_pieces;

        unsigned100_normalize(
            -10,
            10,
            i64::try_from(white_undefended_pieces).unwrap()
                - i64::try_from(black_undefended_pieces).unwrap(),
        )
    }

    fn name(&self) -> &'static str {
        "Undefended Pieces"
    }
}

#[derive(Clone)]
struct UndefendedPieces_MiddleProximity;

impl Heuristic for UndefendedPieces_MiddleProximity {
    fn score(&self, state: &State) -> i64 {
        lazy_static! {
            static ref INVALID_BLACK_DEFENDED_PIECES: HashSet<usize> = {
                let mut invalid_black_defended_pieces = HashSet::new();
                invalid_black_defended_pieces.insert(0);
                invalid_black_defended_pieces.insert(1);
                invalid_black_defended_pieces.insert(2);
                invalid_black_defended_pieces.insert(3);
                invalid_black_defended_pieces.insert(5);
                invalid_black_defended_pieces.insert(6);
                invalid_black_defended_pieces.insert(9);
                invalid_black_defended_pieces
            };
        }

        lazy_static! {
            static ref INVALID_WHITE_DEFENDED_PIECES: HashSet<usize> = {
                let mut invalid_white_defended_pieces = HashSet::new();
                invalid_white_defended_pieces.insert(27);
                invalid_white_defended_pieces.insert(30);
                invalid_white_defended_pieces.insert(31);
                invalid_white_defended_pieces.insert(33);
                invalid_white_defended_pieces.insert(34);
                invalid_white_defended_pieces.insert(35);
                invalid_white_defended_pieces.insert(36);
                invalid_white_defended_pieces
            };
        }

        let black_pieces = state.board.current_players_pieces(0);

        let white_pieces = state.board.current_players_pieces(1);

        let black_straight_hexes = black_pieces
            .iter()
            .map(|&elt| state.board.get_straight_hex(0, elt));

        let black_defended_pieces: HashSet<&usize> = black_straight_hexes
            .filter(|&elt| match elt {
                Some(i) => {
                    let straight_hole = state.board.board[*i];
                    match straight_hole {
                        Hole(Some(Black)) => true,
                        Hole(_) => false,
                    }
                }
                None => false,
            })
            .map(|elt| elt.unwrap())
            .collect();

        let value_black_undefended_pieces: i64 = black_pieces
            .iter()
            .map(|&elt| {
                if INVALID_BLACK_DEFENDED_PIECES.contains(&elt)
                    || black_defended_pieces.contains(&elt)
                {
                    0
                } else {
                    middle_proximity(&elt)
                }
            })
            .sum();

        let white_straight_hexes = white_pieces
            .iter()
            .map(|&elt| state.board.get_straight_hex(1, elt));

        let white_defended_pieces: HashSet<&usize> = white_straight_hexes
            .filter(|&elt| match elt {
                Some(i) => {
                    let straight_hole = state.board.board[*i];
                    match straight_hole {
                        Hole(Some(White)) => true,
                        Hole(_) => false,
                    }
                }
                None => false,
            })
            .map(|elt| elt.unwrap())
            .collect();

        let value_white_undefended_pieces: i64 = white_pieces
            .iter()
            .map(|&elt| {
                if INVALID_WHITE_DEFENDED_PIECES.contains(&elt)
                    || white_defended_pieces.contains(&elt)
                {
                    0
                } else {
                    middle_proximity(&elt)
                }
            })
            .sum();
        unsigned100_normalize(
            -37,
            37,
            value_white_undefended_pieces - value_black_undefended_pieces,
        )
    }

    fn name(&self) -> &'static str {
        "Undefended Pieces Middle Prox"
    }
}

#[derive(Clone)]
struct StraightLines;

impl Heuristic for StraightLines {
    fn score(&self, state: &State) -> i64 {
        let black_pieces = state.board.current_players_pieces(0);

        let white_pieces = state.board.current_players_pieces(1);

        let black_straight_hexes = black_pieces
            .iter()
            .map(|&elt| state.board.get_straight_hex(0, elt));

        let black_score = black_straight_hexes
            .filter(|&elt| match elt {
                Some(i) => {
                    let straight_hole = state.board.board[*i];
                    matches!(straight_hole, Hole(Some(Black)))
                    // matches! macro equivalent to:
                    // match straight_hole {
                    //     Hole(Some(Black)) => true,
                    //     _ => false,
                    // }
                }
                None => false,
            })
            .count();

        let white_straight_hexes = white_pieces
            .iter()
            .map(|&elt| state.board.get_straight_hex(1, elt));

        let white_score = white_straight_hexes
            .filter(|&elt| match elt {
                Some(i) => {
                    let straight_hole = state.board.board[*i];
                    matches!(straight_hole, Hole(Some(White)))
                }
                None => false,
            })
            .count();
        unsigned100_normalize(
            -8,
            8,
            i64::try_from(black_score).unwrap()
                - i64::try_from(white_score).unwrap(),
        )
        // potentially use an iterative DFS to avoid double counting?
    }

    fn name(&self) -> &'static str {
        "Straight Lines"
    }
}

#[derive(Clone)]
struct StraightLines_MiddleProximity;

impl Heuristic for StraightLines_MiddleProximity {
    fn score(&self, state: &State) -> i64 {
        let black_pieces = state.board.current_players_pieces(0);

        let white_pieces = state.board.current_players_pieces(1);

        let black_straight_hexes = black_pieces
            .iter()
            .map(|&elt| (elt, state.board.get_straight_hex(0, elt)));

        let black_score: i64 = black_straight_hexes
            .map(|(idx, elt)| match elt {
                Some(i) => {
                    let straight_hole = state.board.board[*i];
                    match straight_hole {
                        Hole(Some(Black)) => middle_proximity(&idx),
                        _ => 0,
                    }
                }
                None => 0,
            })
            .sum();

        let white_straight_hexes = white_pieces
            .iter()
            .map(|&elt| (elt, state.board.get_straight_hex(1, elt)));

        let white_score: i64 = white_straight_hexes
            .map(|(idx, elt)| match elt {
                Some(i) => {
                    let straight_hole = state.board.board[*i];
                    match straight_hole {
                        Hole(Some(White)) => middle_proximity(&idx),
                        _ => 0,
                    }
                }
                None => 0,
            })
            .sum();
        unsigned100_normalize(-42, 42, black_score - white_score)
    }

    fn name(&self) -> &'static str {
        "Straight Lines Middle Prox"
    }
}

#[derive(Clone)]
struct AggressivePieces;

impl Heuristic for AggressivePieces {
    fn score(&self, state: &State) -> i64 {
        let black_pieces = state.board.current_players_pieces(0);

        let white_pieces = state.board.current_players_pieces(1);

        let black_furthest =
            black_proximity_row(black_pieces.last().unwrap_or(&0));

        let white_furthest =
            white_proximity_row(white_pieces.first().unwrap_or(&36));

        let black_score = black_pieces
            .iter()
            .filter(|&elt| black_proximity_row(elt) >= 12 - white_furthest)
            .count();

        let white_score = white_pieces
            .iter()
            .filter(|&elt| white_proximity_row(elt) >= 12 - black_furthest)
            .count();

        let diff = i64::try_from(white_score).unwrap()
            - i64::try_from(black_score).unwrap();

        if diff <= -2 {
            -1000
        } else if diff >= 2 {
            1000
        } else {
            unsigned100_normalize(-9, 9, diff)
        }
    }

    fn name(&self) -> &'static str {
        "Aggr Pieces"
    }
}

#[derive(Clone)]
struct AggressivePieces_MiddleProximity;

impl Heuristic for AggressivePieces_MiddleProximity {
    fn score(&self, state: &State) -> i64 {
        let black_pieces = state.board.current_players_pieces(0);

        let white_pieces = state.board.current_players_pieces(1);

        let black_furthest =
            black_proximity_row(black_pieces.last().unwrap_or(&0));

        let white_furthest =
            white_proximity_row(white_pieces.first().unwrap_or(&36));

        let black_aggr_pieces = black_pieces
            .iter()
            .filter(|&elt| black_proximity_row(elt) >= 12 - white_furthest);

        let white_aggr_pieces = white_pieces
            .iter()
            .filter(|&elt| white_proximity_row(elt) >= 12 - black_furthest);

        let black_score: i64 =
            black_aggr_pieces.clone().map(middle_proximity).sum();

        let white_score: i64 =
            white_aggr_pieces.clone().map(middle_proximity).sum();

        let diff = i64::try_from(white_aggr_pieces.count()).unwrap()
            - i64::try_from(black_aggr_pieces.count()).unwrap();

        if diff <= -2 {
            -1000
        } else if diff >= 2 {
            1000
        } else {
            unsigned100_normalize(-12, 12, white_score - black_score)
        }
    }

    fn name(&self) -> &'static str {
        "Aggr Pieces Middle Prox"
    }
}

#[derive(Clone)]
struct AggressivePieces_AntiCentrality;

impl Heuristic for AggressivePieces_AntiCentrality {
    fn score(&self, state: &State) -> i64 {
        let black_pieces = state.board.current_players_pieces(0);

        let white_pieces = state.board.current_players_pieces(1);

        let black_furthest =
            black_proximity_row(black_pieces.last().unwrap_or(&0));

        let white_furthest =
            white_proximity_row(white_pieces.first().unwrap_or(&36));

        let black_aggr_pieces = black_pieces
            .iter()
            .filter(|&elt| black_proximity_row(elt) >= 12 - white_furthest);

        let white_aggr_pieces = white_pieces
            .iter()
            .filter(|&elt| white_proximity_row(elt) >= 12 - black_furthest);

        let black_score: i64 =
            black_aggr_pieces.clone().map(anti_centrality).sum();

        let white_score: i64 =
            white_aggr_pieces.clone().map(anti_centrality).sum();

        let diff = i64::try_from(white_aggr_pieces.count()).unwrap()
            - i64::try_from(black_aggr_pieces.count()).unwrap();

        if diff <= -2 {
            -1000
        } else if diff >= 2 {
            1000
        } else {
            unsigned100_normalize(-6, 6, white_score - black_score)
        }
    }

    fn name(&self) -> &'static str {
        "Aggr Pieces Anti Centrality"
    }
}

pub fn unsigned100_normalize(min: i64, max: i64, value: i64) -> i64 {
    //  ((2 * (value - lb)) / (ub - lb)) - 1) * 100
    let numerator = 1000 * 2 * (value - min);
    let denominator = max - min;

    numerator.div(denominator) - 1000
}

// HEURISTIC HELPER FUNCTIONS ( E.G. LOWERBOUND UPPERBOUND CALCULATORS ) //

fn upperbound_middle_proximity(
    middle_weight: i64,
    next_middle_weight: i64,
) -> i64 {
    7 * middle_weight + 3 * next_middle_weight
}

#[derive(Clone)]
pub struct HeuristicWeights {
    functions: [Heuristics; NUM_HEURISTICS],
    weights: Weights,
}

pub type Weights = [f64; NUM_HEURISTICS];

pub fn normalize_weights(w: &mut Weights) {
    let sum: f64 = w.iter().sum();
    for weight in w.iter_mut() {
        *weight /= sum;
    }
}

impl Debug for HeuristicWeights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt_struct = f.debug_struct("Weights");

        for (heuristic_fn, w) in zip(self.functions.iter(), self.weights) {
            fmt_struct.field(heuristic_fn.name(), &format!("{w:.2}"));
        }

        fmt_struct.finish()
    }
}

impl HeuristicWeights {
    pub fn new(weights: Weights) -> Self {
        HeuristicWeights {
            functions: [
                Heuristics::PieceDifferential(PieceDifferential),
                Heuristics::MiddleLineDifferential(MiddleLineDifferential),
                Heuristics::ImportantPieces(ImportantPieces),
                Heuristics::MiddleProximity(MiddleProximity),
                Heuristics::AggressionDifferential(AggressionDifferential),
                Heuristics::PassivenessDifferential(PassivenessDifferential),
                Heuristics::Centrality(Centrality),
                Heuristics::AntiCentrality(AntiCentrality),
                Heuristics::DefendedHexes(DefendedHexes),
                Heuristics::DefendedHexes_MiddleProximity(
                    DefendedHexes_MiddleProximity,
                ),
                Heuristics::UndefendedPieces(UndefendedPieces),
                Heuristics::UndefendedPieces_MiddleProximity(
                    UndefendedPieces_MiddleProximity,
                ),
                Heuristics::AttackTiming(AttackTiming),
                Heuristics::LimitOppoMoves(LimitOppoMoves),
                Heuristics::StraightLines(StraightLines),
                Heuristics::StraightLines_MiddleProximity(
                    StraightLines_MiddleProximity,
                ),
                Heuristics::AggressivePieces(AggressivePieces),
                Heuristics::AggressivePieces_MiddleProximity(
                    AggressivePieces_MiddleProximity,
                ),
                Heuristics::AggressivePieces_AntiCentrality(
                    AggressivePieces_AntiCentrality,
                ),
            ],
            weights,
        }
    }

    pub fn new_with_state(&self, state: &State) -> HeuristicWeightsWithState {
        HeuristicWeightsWithState {
            // TODO: probably shouldn't clone this twice and instead use borrows.. it's fine for now
            heuristic_weights: self.clone(),
            state: state.clone(),
        }
    }

    pub fn score(&self, state: &State) -> f64 {
        if !state.active {
            match state.winner {
                Some(0) => return f64::MAX,
                Some(1) => return f64::MIN,
                _ => (),
            }
        }
        let mut result = 0.0;
        for (w, heuristic_fn) in zip(self.weights, self.functions.iter()) {
            let weighted_score = w * (heuristic_fn.score(state) as f64);
            result += weighted_score
        }
        result
    }

    pub fn new_with_state_and_move(
        &self,
        mut state: State,
        m: Move,
    ) -> HeuristicWeightsWithTwoStates {
        let (Move::Diagonal(origin, dest) | Move::Straight(origin, dest)) = m;

        HeuristicWeightsWithTwoStates {
            heuristic_weights: self.to_owned(),
            old_state: state.clone(),
            new_state: {
                state.move_piece(origin, dest, true).unwrap();
                state
            },
        }
    }

    pub fn new_with_state_and_moves(
        &self,
        mut state: State,
        moves: &Vec<Move>,
    ) -> HeuristicWeightsWithTwoStates {
        let old_state = state.clone();
        for &m in moves {
            let (Move::Diagonal(origin, dest) | Move::Straight(origin, dest)) =
                m;
            state.move_piece(origin, dest, true).unwrap()
        }

        HeuristicWeightsWithTwoStates {
            heuristic_weights: self.to_owned(),
            old_state,
            new_state: state,
        }
    }
}

pub struct HeuristicWeightsWithState {
    heuristic_weights: HeuristicWeights,
    state: State,
}

impl Debug for HeuristicWeightsWithState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt_struct = format!(
            "Total Score: {}",
            &self.heuristic_weights.score(&self.state)
        );

        for (heuristic_fn, w) in zip(
            self.heuristic_weights.functions.iter(),
            self.heuristic_weights.weights,
        ) {
            fmt_struct.push_str(&format!(
                "\n\t{} (weight {}): {}",
                heuristic_fn.name(),
                w,
                heuristic_fn.score(&self.state)
            ));
        }

        f.write_str(&fmt_struct)
    }
}

pub struct HeuristicWeightsWithTwoStates {
    heuristic_weights: HeuristicWeights,
    old_state: State,
    new_state: State,
}

impl Debug for HeuristicWeightsWithTwoStates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt_struct = f.debug_struct(
            format!(
                "Total Score: {} -> {}",
                self.heuristic_weights.score(&self.old_state),
                self.heuristic_weights.score(&self.new_state)
            )
            .as_str(),
        );

        for (heuristic_fn, w) in zip(
            self.heuristic_weights.functions.iter(),
            self.heuristic_weights.weights,
        ) {
            fmt_struct.field(
                heuristic_fn.name(),
                &format!(
                    "(weight {}): {} -> {}",
                    w,
                    heuristic_fn.score(&self.old_state),
                    heuristic_fn.score(&self.new_state)
                ),
            );
        }

        fmt_struct.finish()
    }
}
