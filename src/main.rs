pub mod ai;
pub mod cli;
pub mod game;

use dotenv::dotenv;
use std::env;

#[allow(dead_code)]
fn main() {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Arugment passed. Starting the genetic process");
        cli::start_genetic_process()
    } else {
        cli::choose_phase()
    }
}
