pub mod ai;
pub mod cli;
pub mod game;
pub mod genetic;

use cli::play_game;
use dotenv::dotenv;
use game::{
    gamestate::{GameBuilder, State},
    player::{PossiblePlayer, AI, NN},
};
use log::info;

use std::env;

use milestone::{play_ai_vs_nn, play_two_nn, DATABASE_URL};
use rusqlite::Connection;

#[allow(dead_code)]
fn main() {
    dotenv().ok();
    // configure logger
    log4rs::init_file("log4rs_config.yaml", Default::default()).unwrap();

    // normal_milestone();

    // evaluate_agents("data/exp2.agents", "data/exp2_agent_comparison.csv");
    // evaluate_agents("data/exp3.agents", "data/exp3_agent_comparison.csv");

    // evaluate_exps(
    //     "data/exp2.best_agents",
    //     "data/exp3.best_agents",
    //     5,
    //     "data/exp_comparison.csv",
    // );

    // exp 2
    // let result = play_ai_vs_nn("0.37 0.86 0.21 0.38 0.09 0.79 0.35 0.82 0.84 0.32 0.14 0.53 0.9 0.83 0.56 0.98 0.11 0.08 0.1", "neuralnet/exp2.joblib", true);
    // exp3
    // let result = play_ai_vs_nn("2.69 0.53 0.39 0.45 0.11 3.15 0.04 0.2 1.1 0.72 0.1 0.45 0.14 0.18 0.21 0.63 0.01 0.01 0.04", "neuralnet/exp3.joblib", true);
    // NN2 vs NN3
    // let result = play_two_nn("neuralnet/exp2.joblib", "neuralnet/exp3.joblib");
    // println!("{result}");
}

fn normal_milestone() {
    info!("STARTING MILESTONE PROCESS");

    let conn = Connection::open(&*DATABASE_URL).unwrap();

    // Create a table called `game_table`
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS game_table (
            game_id INTEGER PRIMARY KEY,
            result INTEGER
        )
        "#,
        [],
    )
    .unwrap_or(0);

    // Create a table called `state_table`
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS state_table (
            state_id INTEGER PRIMARY KEY,
            state TEXT NOT NULL,
            move_number INTEGER,
            game_id INTEGER,
            FOREIGN KEY(game_id) REFERENCES game_table(game_id)
        )
        "#,
        [],
    )
    .unwrap_or(0);

    // Create a table called `recovery_table`
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS recovery_table (
            batch_id INTEGER PRIMARY KEY,
            agents BLOB,
            timestamp DATETIME
        )
        "#,
        [],
    )
    .unwrap_or(0);

    conn.close();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        info!("Argument passed. Starting the genetic process");
        cli::start_genetic_process()
    } else {
        cli::choose_phase()
    }
}
