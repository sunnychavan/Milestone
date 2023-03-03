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
    board::Hole, gamestate::State, pieces::Piece::Black, pieces::Piece::White,
};

#[enum_dispatch]
#[derive(Clone)]
enum Heuristics {
    PieceDifferential,
    HoldImportantPieces,
    MiddleProximity,
    MiddlePieceDifferential,
    WinLose,
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
        lazy_static! {
            static ref IMPORTANT_PIECES: HashMap<usize, i64> = {
                let mut hm = HashMap::new();
                hm.insert(0, 3);
                hm.insert(1, 1);
                hm.insert(2, 1);
                hm.insert(36, 3);
                hm.insert(34, 1);
                hm.insert(35, 1);
                hm
            };
        }

        let black_score: i64 = state
            .board
            .current_players_pieces(0)
            .iter()
            .map(|&elt| IMPORTANT_PIECES.get(&elt).unwrap_or(&0))
            .sum();

        let white_score: i64 = state
            .board
            .current_players_pieces(1)
            .iter()
            .map(|&elt| IMPORTANT_PIECES.get(&elt).unwrap_or(&0))
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
        lazy_static! {
            static ref MIDDLE_PROXIMITY: HashMap<usize, i64> = {
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

                middle_proximity
            };
        }
        let black_score: i64 = state
            .board
            .current_players_pieces(0)
            .iter()
            .map(|&elt| MIDDLE_PROXIMITY.get(&elt).unwrap_or(&0))
            .sum();

        let white_score: i64 = state
            .board
            .current_players_pieces(1)
            .iter()
            .map(|&elt| MIDDLE_PROXIMITY.get(&elt).unwrap_or(&0))
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
        let blacks_home = state.board.board[0];
        let whites_home = state.board.board[36];
        match (blacks_home, whites_home) {
            (Hole(Some(White)), _) => -100,
            (_, Hole(Some(Black))) => 100,
            _ => 0,
        }
    }

    fn name(&self) -> &'static str {
        "Win / Lose"
    }
}

fn unsigned100_normalize(min: i64, max: i64, value: i64) -> i64 {
    //  ((2 * (value - lb)) / (ub - lb)) - 1) * 100
    let numerator = 100 * 2 * (value - min);
    let denominator = max - min;

    numerator.div(denominator) - 100
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
    functions: [Heuristics; 5],
    weights: [i64; 5],
}

impl Debug for HeuristicWeights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.weights).finish()
    }
}

impl HeuristicWeights {
    pub fn new(weights: [i64; 5]) -> Self {
        HeuristicWeights {
            functions: [
                Heuristics::PieceDifferential(PieceDifferential),
                Heuristics::HoldImportantPieces(HoldImportantPieces),
                Heuristics::MiddleProximity(MiddleProximity),
                Heuristics::MiddlePieceDifferential(MiddlePieceDifferential),
                Heuristics::WinLose(WinLose),
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

    pub fn difference(
        &self,
        old_state: &State,
        new_state: &State,
    ) -> HeuristicWeightsWithTwoStates {
        HeuristicWeightsWithTwoStates {
            heuristic_weights: self.to_owned(),
            old_state: old_state.to_owned(),
            new_state: new_state.to_owned(),
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
        let mut fmt_struct = format!(
            "Total Score: {} -> {}",
            &self.heuristic_weights.score(&self.old_state),
            &self.heuristic_weights.score(&self.new_state),
        );

        for (heuristic_fn, w) in zip(
            self.heuristic_weights.functions.iter(),
            self.heuristic_weights.weights,
        ) {
            fmt_struct.push_str(&format!(
                "\n\t{} (weight {}): {} -> {}",
                heuristic_fn.name(),
                w,
                heuristic_fn.score(&self.old_state),
                heuristic_fn.score(&self.new_state),
            ));
        }

        f.write_str(&fmt_struct)
    }
}
