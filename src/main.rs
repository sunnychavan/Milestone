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

use milestone::DATABASE_URL;
use rusqlite::Connection;

#[allow(dead_code)]
fn main() {
    dotenv().ok();
    // configure logger
    log4rs::init_file("log4rs_config.yaml", Default::default()).unwrap();

    normal_milestone();

    // evaluate_agents("data/exp2.agents", "data/exp2_agent_comparison.csv");
    // evaluate_agents("data/exp3.agents", "data/exp3_agent_comparison.csv");

    // evaluate_exps(
    //     "data/exp2.best_agents",
    //     "data/exp3.best_agents",
    //     5,
    //     "data/exp_comparison.csv",
    // );
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
