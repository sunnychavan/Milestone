//! all heuristics are oriented to provide high scores when black is in a good
//! situation, and low (negative) scores for white advantaged positions

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

use super::location_maps::{self, middle_proximity};

#[enum_dispatch]
#[derive(Clone)]
enum Heuristics {
    PieceDifferential,
    HoldImportantPieces,
    MiddleProximity,
    MiddlePieceDifferential,
    WinLose,
    NumberDefendedEmptyHexes,
    ValueOfDefendedEmptyHexes,
    NumberUndefendedPieces,
    ValueUndefendedPieces,
}

#[enum_dispatch(Heuristics)]
trait Heuristic {
    fn score(&self, state: &State) -> i64;

    fn name(&self) -> &'static str;
}

#[derive(Clone)]
struct PieceDifferential;

impl Heuristic for PieceDifferential {
    fn score(&self, state: &State) -> i64 {
        let current_player_num = state
            .board
            .current_players_pieces(0)
            .len()
            .try_into()
            .unwrap_or(0);
        let opponent_num = state
            .board
            .current_players_pieces(1)
            .len()
            .try_into()
            .unwrap_or(0);

        unsigned100_normalize(-10, 10, current_player_num - opponent_num)
    }

    fn name(&self) -> &'static str {
        "Piece Diff"
    }
}

#[derive(Clone)]
struct HoldImportantPieces;

impl Heuristic for HoldImportantPieces {
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
            .map(|&elt| location_maps::middle_proximity(elt))
            .sum();

        let white_score: i64 = state
            .board
            .current_players_pieces(1)
            .iter()
            .map(|&elt| location_maps::middle_proximity(elt))
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
struct MiddlePieceDifferential;

impl Heuristic for MiddlePieceDifferential {
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
        "Mid Piece Diff"
    }
}

#[derive(Clone)]
struct WinLose;

impl Heuristic for WinLose {
    fn score(&self, state: &State) -> i64 {
        // don't need to normalize
        match state.winner {
            Some(0) => 1000,
            Some(1) => -1000,
            _ => 0,
        }
    }

    fn name(&self) -> &'static str {
        "Win / Lose"
    }
}

#[derive(Clone)]
struct NumberDefendedEmptyHexes;

impl Heuristic for NumberDefendedEmptyHexes {
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
        unsigned100_normalize(
            -10,
            10,
            i64::try_from(black_score).unwrap()
                - i64::try_from(white_score).unwrap(),
        )
    }

    fn name(&self) -> &'static str {
        "Number of Defended Empty Hexes"
    }
}

#[derive(Clone)]
struct ValueOfDefendedEmptyHexes;

impl Heuristic for ValueOfDefendedEmptyHexes {
    fn score(&self,state: &State) -> i64 {
        let black_pieces = state
        .board
        .current_players_pieces(0);

        let white_pieces = state
            .board
            .current_players_pieces(1);

        let black_straight_hexes  = black_pieces
            .iter()
            .map(|&elt| state.board.get_straight_hex(0, elt));
        
        let black_score: i64 = black_straight_hexes.map(|elt| {
            match elt {
                Some(i) => {
                    // let hex_taken = black_pieces.contains(i) || white_pieces.contains(i);
                    // !hex_taken
                    let straight_hole = state.board.board[*i];
                    match straight_hole {
                        Hole(Some(_)) => 0,
                        Hole(None) => middle_proximity(*i),
                    }}
                    None => 0,
                }
            })
            .sum();

        let white_straight_hexes = white_pieces
            .iter()
            .map(|&elt| state.board.get_straight_hex(1, elt));
        
        let white_score: i64 = white_straight_hexes.map(|elt| {
            match elt {
                Some(i) => {
                    // let hex_taken = black_pieces.contains(i) || white_pieces.contains(i);
                    // !hex_taken
                    let straight_hole = state.board.board[*i];
                    match straight_hole {
                        Hole(Some(_)) => 0,
                        Hole(None) => middle_proximity(*i),
                    }
                },
                None => 0
            }
        }).sum();
        
        unsigned100_normalize(-37, 37, black_score-white_score)            
    } 

    fn name(&self) -> &'static str {
        "Value of Defended Empty Hexes"
    }
}

#[derive(Clone)]
struct NumberUndefendedPieces;

impl Heuristic for NumberUndefendedPieces{

    fn score(&self,state: &State) -> i64 {
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


        let black_pieces = state
        .board
        .current_players_pieces(0);

        let num_valid_black_pieces = black_pieces.iter().filter(|&elt|{
            !INVALID_BLACK_DEFENDED_PIECES.contains(elt)
        }).count();

        let white_pieces = state
            .board
            .current_players_pieces(1);
        
        let num_valid_white_pieces = white_pieces.iter().filter(|&elt|{
            !INVALID_WHITE_DEFENDED_PIECES.contains(elt)
        }).count();

        let black_straight_hexes  = black_pieces
            .iter()
            .map(|&elt| state.board.get_straight_hex(0, elt));

        
        let black_defended_pieces = black_straight_hexes.filter(|&elt| {
            match elt {
                Some(i) => {
                    let straight_hole = state.board.board[*i];
                    match straight_hole {
                        Hole(Some(Black)) => true,
                        Hole(_) => false,
                    }
                },
                None => false
            }
        }).count();

        let black_undefended_pieces = num_valid_black_pieces - black_defended_pieces;


        let white_straight_hexes  = white_pieces
            .iter()
            .map(|&elt| state.board.get_straight_hex(1, elt));
        
        let white_defended_pieces = white_straight_hexes.filter(|&elt| {
            match elt {
                Some(i) => {
                    let straight_hole = state.board.board[*i];
                    match straight_hole {
                        Hole(Some(White)) => true,
                        Hole(_) => false,
                    }
                },
                None => false
            }
        }).count();

        let white_undefended_pieces = num_valid_white_pieces - white_defended_pieces;

        unsigned100_normalize(-10, 10, i64::try_from(white_undefended_pieces).unwrap()-i64::try_from(black_undefended_pieces).unwrap())            
    } 

    fn name(&self) ->  &'static str {
        "Number of Undefended Pieces"
    }
}


#[derive(Clone)]
struct ValueUndefendedPieces;

impl Heuristic for ValueUndefendedPieces{

    fn score(&self,state: &State) -> i64 {
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


        let black_pieces = state
        .board
        .current_players_pieces(0);


        let white_pieces = state
        .board
        .current_players_pieces(1);

        
        let black_straight_hexes  = black_pieces
        .iter()
        .map(|&elt| state.board.get_straight_hex(0, elt));

        
        let black_defended_pieces: HashSet<&usize>= black_straight_hexes.filter(|&elt| {
            match elt {
                Some(i) => {
                    let straight_hole = state.board.board[*i];
                    match straight_hole {
                        Hole(Some(Black)) => true,
                        Hole(_) => false,
                    }
                },
                None => false
            }
        }).map({|elt| elt.unwrap()}).collect();

        let value_black_undefended_pieces: i64 = black_pieces.iter().map(|&elt|{
            if INVALID_BLACK_DEFENDED_PIECES.contains(&elt) || black_defended_pieces.contains(&elt) {
                0
            }
            else{
                middle_proximity(elt)
            }
        }).sum();

        let white_straight_hexes  = white_pieces
        .iter()
        .map(|&elt| state.board.get_straight_hex(1, elt));

        
        let white_defended_pieces: HashSet<&usize>= white_straight_hexes.filter(|&elt| {
            match elt {
                Some(i) => {
                    let straight_hole = state.board.board[*i];
                    match straight_hole {
                        Hole(Some(White)) => true,
                        Hole(_) => false,
                    }
                },
                None => false
            }
        }).map({|elt| elt.unwrap()}).collect();

        let value_white_undefended_pieces:i64 = white_pieces.iter().map(|&elt|{
            if INVALID_WHITE_DEFENDED_PIECES.contains(&elt) || white_defended_pieces.contains(&elt) {
                0
            }
            else{
                middle_proximity(elt)
            }
        }).sum();
        unsigned100_normalize(-37, 37, value_white_undefended_pieces-value_black_undefended_pieces)            
    } 

    fn name(&self) ->  &'static str {
        "Value of Undefended Pieces"
    }
}


fn unsigned100_normalize(min: i64, max: i64, value: i64) -> i64 {
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
    functions: [Heuristics; 9],
    weights: [i64; 9],
}

impl Debug for HeuristicWeights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.weights).finish()
    }
}

impl HeuristicWeights {
    pub fn new(weights: [i64; 9]) -> Self {
        HeuristicWeights {
            functions: [
                Heuristics::PieceDifferential(PieceDifferential),
                Heuristics::HoldImportantPieces(HoldImportantPieces),
                Heuristics::MiddleProximity(MiddleProximity),
                Heuristics::MiddlePieceDifferential(MiddlePieceDifferential),
                Heuristics::WinLose(WinLose),
                Heuristics::NumberDefendedEmptyHexes(NumberDefendedEmptyHexes),
                Heuristics::ValueOfDefendedEmptyHexes(ValueOfDefendedEmptyHexes),
                Heuristics::NumberUndefendedPieces(NumberUndefendedPieces),
                Heuristics::ValueUndefendedPieces(ValueUndefendedPieces),
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

    pub fn score(&self, state: &State) -> i64 {
        let mut result = 0;
        for (w, heuristic_fn) in zip(self.weights, self.functions.iter()) {
            let weighted_score = w * heuristic_fn.score(state);
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
        let mut fmt_struct = f.debug_struct("Heuristic Change");

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
