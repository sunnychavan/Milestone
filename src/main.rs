pub mod ai;
pub mod cli;
pub mod game;

use dotenv::dotenv;
use env_logger;
use log::info;
use std::{env, io::Stdout};

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

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        info!("Argument passed. Starting the genetic process");
        cli::start_genetic_process()
    } else {
        cli::choose_phase()
    }
}
