pub mod ai;
pub mod cli;
pub mod game;
pub mod genetic;

use dotenv::dotenv;
use env_logger;
use log::info;
use std::{env, io::Stdout};

#[derive(Debug)]
struct StateTest {
    id: i32,
    state: String,
}

use rusqlite::Connection;

#[allow(dead_code)]
fn main() {
    dotenv().ok();

    // configure logger
    env_logger::builder()
        .format_timestamp(None)
        .format_target(false)
        .format_module_path(false)
        .target(env_logger::Target::Stdout)
        .init();

    let url = "./src/database/example.sqlite3";

    let conn = Connection::open(url).unwrap();

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
            generation_id INTEGER,
            agents INTEGER,
            timestamp INTEGER
        )
        "#,
        [],
    )
    .unwrap_or(0);

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        info!("Argument passed. Starting the genetic process");
        cli::start_genetic_process()
    } else {
        cli::choose_phase()
    }
}
