use crate::ai::heuristics::{
    normalize_weights, HeuristicWeights, Weights, NUM_HEURISTICS,
};
use crate::ai::tree::SearchLimit;
use crate::game::board::Move;

use crate::ai::tree::get_best_move;

use super::gamestate::State;
use core::fmt::Debug;
use std::path::Path;

use log::trace;
use pyo3::types::{PyString, PyTuple};
use serde::{Deserialize, Serialize};
use std::{fmt, io};

use crate::game::player::Move::{Diagonal, Straight};

use ordered_float::OrderedFloat;
use pyo3::{prelude::*, types::IntoPyDict};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Person {
    name: String,
}

impl Person {
    pub fn new(name: String) -> Person {
        Person { name }
    }
}

impl Player for Person {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn one_turn(&self, state: &mut State) {
        println!("Input your move:");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match handle_move_input(state, input.trim()) {
                    Ok(_) => (),
                    Err(e) => {
                        println!(
                            "Couldn't process that move ({e}). Please try again"
                        );
                        self.one_turn(state);
                    }
                };
            }
            Err(e) => println!("Oops. Something went wrong ({e})"),
        }
    }
}

fn handle_move_input(
    game: &mut State,
    input: &str,
) -> Result<(), &'static str> {
    match input.split('-').collect::<Vec<&str>>()[..] {
        [a, b] => {
            let from = a.parse::<usize>();
            let to = b.parse::<usize>();
            match (from, to) {
                (Ok(origin), Ok(dest)) => game.move_piece(origin, dest, true),
                _ => Err("couldn't parse your move"),
            }
        }
        _ => Err("improperly formatted move"),
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct AI {
    name: String,
    pub weights: Weights,
    limit: SearchLimit,
}

impl Default for AI {
    fn default() -> Self {
        AI {
            name: String::default(),
            weights: [1.0; NUM_HEURISTICS],
            limit: SearchLimit::default(),
        }
    }
}

impl Debug for AI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt_struct = format!("{:.2?}", HeuristicWeights::new(self.weights));

        f.write_str(&fmt_struct)
    }
}

impl AI {
    pub fn from_name(name: String) -> AI {
        AI {
            name,
            weights: [1.0; NUM_HEURISTICS],
            limit: SearchLimit::default(),
        }
    }

    pub fn from_weights(name: String, vec_weights: Vec<f64>) -> AI {
        let mut array_weights = [0.0; NUM_HEURISTICS];
        for (i, w) in vec_weights.iter().enumerate() {
            array_weights[i] = w.to_owned();
        }
        AI {
            name,
            weights: array_weights,
            limit: SearchLimit::default(),
        }
    }

    pub fn new(name: String, weights: Weights, limit: SearchLimit) -> AI {
        normalize_weights(&mut weights.to_owned());

        AI {
            name,
            weights,
            limit,
        }
    }
}

impl Player for AI {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn one_turn(&self, state: &mut State) {
        let sugg_move = get_best_move(state, &self.limit, &self.weights);

        let (Move::Diagonal(origin, dest) | Move::Straight(origin, dest)) =
            sugg_move.suggestion;

        state
            .move_piece(origin, dest, true)
            .expect("could not play the AI-suggested move");

        trace!("{sugg_move:#?}");
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct NN {
    name: String,
    path: String,
}

impl NN {
    pub fn new(name: String, path: String) -> NN {
        NN { name, path }
    }

    fn run_python_nn(state_string_repr: &str) -> PyResult<f64> {
        let code_location = "neuralnet/neural_network.py";
        let code = std::fs::read_to_string(code_location)?;

        pyo3::prepare_freethreaded_python();
        let black_score = Python::with_gil(|py| -> PyResult<f64> {
            let module = PyModule::from_code(
                py,
                &code,
                "neural_network.py",
                "neural_network.py",
            )?;

            let nn_function = module.getattr("run_nn")?;
            let args = PyTuple::new(
                py,
                [
                    PyString::new(py, state_string_repr),
                    PyString::new(py, "neuralnet/nn.joblib"),
                    // state_string_repr.into_py(py),
                    // "neuralnet/nn.joblib".into_py(py),
                ],
            );
            let result = nn_function.call(args, None)?.extract::<f64>()?;
            Ok(result)
        });

        Ok(black_score.unwrap())
    }
}

impl Player for NN {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn one_turn(&self, state: &mut State) {
        let next_move_vec = state.current_possible_moves(state.current_turn);
        let mut string_next_state_repr_vec: Vec<String> = Vec::new();
        for m @ Straight(origin, dest) | m @ Diagonal(origin, dest) in
            &next_move_vec
        {
            let mut potential_state = state.clone();
            potential_state.move_piece(*origin, *dest, true).unwrap();
            string_next_state_repr_vec.push(potential_state.to_repr_string())
        }

        let next_state_nn_black_score: Vec<f64> = string_next_state_repr_vec
            .into_iter()
            .map(|e| 
                {

                    match e.is_empty() {
                        false => NN::run_python_nn(&e).unwrap(),
                        true => {
                            match state.current_turn {
                                0 => 1.0,
                                1 => 0.0,
                                _ => panic!("The current turn is a value other than 0 or 1."),
                        }
                    }
                }
            })
            .collect();

        // let (max_index_move, max_value) = next_state_nn_black_score
        //     .iter()
        //     .enumerate()
        //     .max_by_key(|&(_, value)| OrderedFloat(*value))
        //     .map(|(index, value)| (index,*value)).unwrap();

        let enumerable_state_score = next_state_nn_black_score
            .iter()
            .enumerate();

        let (best_index_move, best_value) = match state.current_turn {
            0 => {enumerable_state_score.max_by_key(|&(_, value)| OrderedFloat(*value))
            .map(|(index, value)| (index,*value)).unwrap()},
            1 => {enumerable_state_score.min_by_key(|&(_, value)| OrderedFloat(*value))
            .map(|(index, value)| (index,*value)).unwrap()},
            _ => panic!("The current turn is a value other than 0 or 1.")
        };

        // print!("HERE ARE THE VALUES");
        // println!("{:?}", max_index_move);
        // println!("{:?}", max_value);
        let best_nn_move = next_move_vec[best_index_move];

        match best_nn_move {
            Straight(origin, dest) | Diagonal(origin, dest) => state
                .move_piece(origin, dest, true)
                .expect("could not play the NN suggested move"),
        }
    }
}

pub trait Player {
    fn one_turn(&self, state: &mut State);

    fn name(&self) -> String;
}

#[derive(Clone, Debug, PartialEq)]
pub enum PossiblePlayer {
    Person(Person),
    AI(AI),
    NN(NN),
}

impl Default for PossiblePlayer {
    fn default() -> Self {
        PossiblePlayer::Person(Person::default())
    }
}

impl Player for PossiblePlayer {
    fn name(&self) -> String {
        match self {
            PossiblePlayer::Person(p) => p.name(),
            PossiblePlayer::AI(a) => a.name(),
            PossiblePlayer::NN(n) => n.name(),
        }
    }

    fn one_turn(&self, state: &mut State) {
        match self {
            PossiblePlayer::Person(p) => p.one_turn(state),
            PossiblePlayer::AI(a) => a.one_turn(state),
            PossiblePlayer::NN(n) => n.one_turn(state),
        }
    }
}
