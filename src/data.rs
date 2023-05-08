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

fn play_agents(file_path: &str) -> Vec<(String, String, u8)> {
    let lines = read_lines(file_path).expect("could not read this file");
    let agents = &lines
        .map(|line| {
            let line_content = line.unwrap();
            let line_split = line_content.split(',').collect::<Vec<_>>();
            let result: Vec<String> =
                line_split.into_iter().map(|elt| elt.to_string()).collect();
            result
        })
        .collect::<Vec<_>>();

    let n = agents.len();
    let result = (0..n).into_par_iter().flat_map(|i| {
        (0..i).into_par_iter().map(move |j| {
            let new_agent_weights = &agents[i][1];
            let old_agent_weights = &agents[j][1];
            info!("Playing between {i} and {j}");
            let match_result =
                play_two_ai(new_agent_weights, old_agent_weights);
            (
                agents[i][0].to_owned(),
                agents[j][0].to_owned(),
                match_result,
            )
        })
    });

    result.collect::<Vec<_>>()
}

fn agents_to_csv(a: Vec<(String, String, u8)>, file_path: &str) {
    let mut file = File::create(file_path).expect("could not open file");

    a.into_iter().for_each(|(label_one, label_two, score)| {
        writeln!(file, "{label_one}, {label_two}, {score}")
            .expect("could not write row");
    });
}

pub fn evaluate_agents(input_path: &str, output_path: &str) {
    let agents = play_agents(input_path);
    agents_to_csv(agents, output_path);
}

pub fn evaluate_exps(
    input_path_one: &str,
    input_path_two: &str,
    amt: usize,
    output_path: &str,
) {
    let exp_one_lines =
        read_lines(input_path_one).expect("could not read this file");
    let exp_one_agents = &exp_one_lines
        .map(|line| {
            let line_content = line.unwrap();
            let line_split = line_content.split(',').collect::<Vec<_>>();
            let result: Vec<String> =
                line_split.into_iter().map(|elt| elt.to_string()).collect();
            result
        })
        .collect::<Vec<_>>();

    let exp_two_lines =
        read_lines(input_path_two).expect("could not read this file");
    let exp_two_agents = &exp_two_lines
        .map(|line| {
            let line_content = line.unwrap();
            let line_split = line_content.split(',').collect::<Vec<_>>();
            let result: Vec<String> =
                line_split.into_iter().map(|elt| elt.to_string()).collect();
            result
        })
        .collect::<Vec<_>>();

    let result = (0..amt)
        .into_par_iter()
        .flat_map(|i| {
            (0..amt).into_par_iter().map(move |j| {
                let exp_one_weights = &exp_one_agents[i][1];
                let exp_two_weights = &exp_two_agents[j][1];
                info!("Playing between {i} and {j}");
                let match_result =
                    play_two_ai(exp_one_weights, exp_two_weights);
                (
                    exp_one_agents[i][0].to_owned(),
                    exp_two_agents[j][0].to_owned(),
                    match_result,
                )
            })
        })
        .collect::<Vec<_>>();

    agents_to_csv(result, output_path)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
// source: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
