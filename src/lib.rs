use lazy_static::lazy_static;
use std::env;

pub mod ai;
pub mod cli;
pub mod game;
pub mod genetic;

lazy_static! {
    pub static ref DATABASE_URL: String = env::var("DATABASE_URL").unwrap();
}
