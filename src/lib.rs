use ai::tree::SearchLimit;
use genetic::AGENT_DEPTH;
use lazy_static::lazy_static;
use log::info;
use pyo3::pyfunction;
use pyo3::pymodule;
use pyo3::types::PyModule;
use pyo3::wrap_pyfunction;
use pyo3::PyResult;
use pyo3::Python;
use rayon::prelude::*;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path::Path;

pub mod ai;
pub mod cli;
pub mod game;
pub mod genetic;

use crate::cli::play_game;
use crate::game::gamestate::GameBuilder;
use crate::game::player::PossiblePlayer;
use crate::game::player::AI;
use crate::game::player::NN;

lazy_static! {
    pub static ref DATABASE_URL: String = env::var("DATABASE_URL").unwrap();
}

fn play_two_ai(weights_one: &String, weights_two: &String) -> u8 {
    let mut g_one = GameBuilder::new()
        .set_player_1(PossiblePlayer::AI(AI::new(
            "AI 1".to_string(),
            floats_from_string(weights_one.to_string())
                .try_into()
                .unwrap(),
            *AGENT_DEPTH,
        )))
        .set_player_2(PossiblePlayer::AI(AI::new(
            "AI 2".to_string(),
            floats_from_string(weights_two.to_string())
                .try_into()
                .unwrap(),
            *AGENT_DEPTH,
        )))
        .build();

    let mut g_two = GameBuilder::new()
        .set_player_2(PossiblePlayer::AI(AI::new(
            "AI 1".to_string(),
            floats_from_string(weights_one.to_string())
                .try_into()
                .unwrap(),
            *AGENT_DEPTH,
        )))
        .set_player_1(PossiblePlayer::AI(AI::new(
            "AI 2".to_string(),
            floats_from_string(weights_two.to_string())
                .try_into()
                .unwrap(),
            *AGENT_DEPTH,
        )))
        .build();

    play_game(&mut g_one);
    play_game(&mut g_two);
    g_one.winner.unwrap() + 1 - g_two.winner.unwrap()
}

pub fn play_two_nn(file_path_one: &str, file_path_two: &str) -> u8 {
    let mut g_one = GameBuilder::new()
        .set_player_1(PossiblePlayer::NN(NN::new(
            "NN 1".to_string(),
            file_path_one.to_string(),
        )))
        .set_player_2(PossiblePlayer::NN(NN::new(
            "NN 2".to_string(),
            file_path_two.to_string(),
        )))
        .build();

    let mut g_two = GameBuilder::new()
        .set_player_2(PossiblePlayer::NN(NN::new(
            "NN 1".to_string(),
            file_path_one.to_string(),
        )))
        .set_player_1(PossiblePlayer::NN(NN::new(
            "NN 2".to_string(),
            file_path_two.to_string(),
        )))
        .build();

    play_game(&mut g_one);
    play_game(&mut g_two);
    g_one.winner.unwrap() + 1 - g_two.winner.unwrap()
}

pub fn play_ai_vs_nn(weights: &str, file_path: &str, in_order: bool) -> u8 {
    let (player_one, player_two) = if in_order {
        (
            PossiblePlayer::AI(AI::new(
                "AI 1".to_string(),
                floats_from_string(weights.to_string()).try_into().unwrap(),
                *AGENT_DEPTH,
            )),
            PossiblePlayer::NN(NN::new(
                "NN 2".to_string(),
                file_path.to_string(),
            )),
        )
    } else {
        (
            PossiblePlayer::NN(NN::new(
                "NN 1".to_string(),
                file_path.to_string(),
            )),
            PossiblePlayer::AI(AI::new(
                "AI 2".to_string(),
                floats_from_string(weights.to_string()).try_into().unwrap(),
                *AGENT_DEPTH,
            )),
        )
    };

    let mut g_one = GameBuilder::new()
        .set_player_1(player_one.clone())
        .set_player_2(player_two.clone())
        .build();

    let mut g_two = GameBuilder::new()
        .set_player_2(player_one)
        .set_player_1(player_two)
        .build();

    play_game(&mut g_one);
    play_game(&mut g_two);
    g_one.winner.unwrap() + 1 - g_two.winner.unwrap()
}

fn floats_from_string(s: String) -> Vec<f64> {
    s.split_whitespace().map(|x| x.parse().unwrap()).collect()
}
