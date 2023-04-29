use crate::ai::heuristics::NUM_HEURISTICS;
use crate::ai::tree::SearchLimit;
use crate::game::gamestate::{GameBuilder, State};

use crate::genetic::mutate_from_recovery;
use crate::{genetic, DATABASE_URL};

use crate::game::player::PossiblePlayer;

use crate::game::player::{Person, AI};
use log::info;
use rusqlite::Connection;
use std::{env, io};

#[derive(PartialEq)]
enum GameType {
    Black,
    White,
    Humans,
    AIs,
    String,
    Genetic,
    BlackWeights,
    WhiteWeights,
}

fn get_gametype_from_user() -> GameType {
    println!(
        "Enter:\n\t(1) to play a game as black vs the AI,\
               \n\t(2) to play a game as white vs the AI,\
               \n\t(3) to play against another human,\
               \n\t(4) to have two AIs play each other\
               \n\t(5) to load from a string, or\
               \n\t(6) to play as white vs a black AI with inputted weights, or\
               \n\t(7) to play as black vs a white AI with inputted weights, or\
               \n\t(0) to launch the genetic algorithm."
    );

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match get_gametype_from_string(input.as_str().trim()) {
                Some(gt) => gt,
                None => {
                    println!("Sorry, couldn't recognize that input. Please try again:");
                    get_gametype_from_user()
                }
            }
        }
        Err(e) => {
            panic!("Oops. Something went wrong ({e})");
        }
    }
}

fn get_gametype_from_string(g: &str) -> Option<GameType> {
    match g {
        "1" => Some(GameType::Black),
        "2" => Some(GameType::White),
        "3" => Some(GameType::Humans),
        "4" => Some(GameType::AIs),
        "5" => Some(GameType::String),
        "6" => Some(GameType::BlackWeights),
        "7" => Some(GameType::WhiteWeights),
        "0" => Some(GameType::Genetic),
        _ => None,
    }
}

fn get_game_from_gametype(game_type: GameType) -> State {
    let gb = GameBuilder::new();
    match game_type {
        GameType::Black => {
            // game as black
            let player_name = get_name_from_user("yourself");

            gb.set_player_1(PossiblePlayer::Person(Person::new(player_name)))
                .set_player_2(PossiblePlayer::AI(AI::from_name(
                    "AI".to_string(),
                )))
                .build()
        }
        GameType::White => {
            // game as white
            let player_name = get_name_from_user("yourself");

            gb.set_player_1(PossiblePlayer::AI(AI::from_name("AI".to_string())))
                .set_player_2(PossiblePlayer::Person(Person::new(player_name)))
                .build()
        }
        GameType::Humans => {
            // game between two humans
            let player_1_name = get_name_from_user("player 1");
            let player_2_name = get_name_from_user("player 2");

            gb.set_player_1(PossiblePlayer::Person(Person::new(player_1_name)))
                .set_player_2(PossiblePlayer::Person(Person::new(
                    player_2_name,
                )))
                .build()
        }
        GameType::AIs => {
            let depth = get_depth_from_user();
            // game between two AI
            gb.set_player_1(PossiblePlayer::AI(AI::new(
                "AI 1".to_string(),
                [1.0; NUM_HEURISTICS],
                SearchLimit::Depth(depth),
            )))
            .set_player_2(PossiblePlayer::AI(AI::new(
                "AI 2".to_string(),
                [1.0; NUM_HEURISTICS],
                SearchLimit::Depth(depth),
            )))
            .build()
        }
        GameType::String => {
            info!("Paste string below:");
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    State::from_repr_string(input.trim()).unwrap_or_else(|_| {
                        println!(
                            "This string wasn't parseable. Please try again"
                        );
                        get_game_from_gametype(GameType::String)
                    })
                }
                Err(e) => {
                    println!(
                        "Oops. Something went wrong ({e}), please try again"
                    );
                    get_game_from_gametype(GameType::String)
                }
            }
        }
        GameType::BlackWeights => {
            // game as white vs Black AI with inputted weights
            let player_name = get_name_from_user("yourself");
            let ai_weights = get_weights_from_user();


            gb.set_player_1(PossiblePlayer::AI(AI::from_weights("AI".to_string(), ai_weights)))
              .set_player_2(PossiblePlayer::Person(Person::new(player_name)))
              .build()
        }, 
        GameType::WhiteWeights => {
            // game as black vs White AI with inputted weights
            let player_name = get_name_from_user("yourself");
            let ai_weights = get_weights_from_user();
            
            gb.set_player_1(PossiblePlayer::Person(Person::new(player_name)))
                .set_player_2(PossiblePlayer::AI(AI::from_weights("AI".to_string(), ai_weights)))
                .build()

            
        }
        _ => {
            panic!("invalid game type");
        }
    }
}

pub fn play_game(mut game: State) {
    info!("{game}");

    while game.active {
        game.play_one_turn();
        info!("{game}");
        // game.add_to_state_history();
    }
    // game.push_game_and_state().unwrap();
}

pub fn start_genetic_process() {
    let conn = Connection::open(&*DATABASE_URL).unwrap();
    let mut stmt = conn
        .prepare(
            r#"
            SELECT batch_id, agents FROM recovery_table
            ORDER BY batch_id DESC LIMIT 1
            "#,
        )
        .unwrap();
    let mut batch_agents_iter = stmt
        .query_map([], |row| {
            let batch_num: u32 = row.get(0).unwrap();
            let bin_agent: Vec<u8> = row.get(1).unwrap();
            let mut agents_and_scores: Vec<(AI, i16)> =
                bincode::deserialize(&bin_agent).unwrap();
            agents_and_scores.sort_by_key(|(_, elo)| -elo);
            let agents: Vec<AI> =
                agents_and_scores.into_iter().map(|e| e.0).collect();
            Ok((batch_num, agents))
        })
        .unwrap()
        .peekable();

    let ai = if batch_agents_iter.peek().is_none() {
        // if no rows exist in recovery_table, start from scratch
        info!("No rows found in the recovery table, initializing genetic algorithm...");
        genetic::run(1, None)
    } else {
        // if rows exist, start from most recent agents
        let found_batch = batch_agents_iter.next().unwrap();
        let (batch_num, agents) = found_batch.unwrap();

        // drop to free the database connection before running genetic
        drop(batch_agents_iter);

        info!(
            "Found existing rows in recovery table for gen {batch_num}, \
             mutating & starting genetic algorithm from {}",
            batch_num + 1
        );

        genetic::run(
            batch_num + 1,
            Some(mutate_from_recovery(batch_num, agents)),
        )
    };
    info!("Genetic process completed");

    if env::var("PLAY_AFTER").is_ok() {
        let g = GameBuilder::new()
            .set_player_1(PossiblePlayer::Person(Person::default()))
            .set_player_2(PossiblePlayer::AI(ai))
            .build();
        play_game(g);
    }
}

pub fn choose_phase() {
    let gametype = match env::var("LAUNCH_ARG") {
        Ok(i) => {
            info!("Using argument {i} from environment variable.");
            match get_gametype_from_string(i.as_str()) {
                Some(i) => i,
                None => get_gametype_from_user(),
            }
        }
        Err(_) => get_gametype_from_user(),
    };

    if gametype == GameType::Genetic {
        start_genetic_process()
    } else {
        play_game(get_game_from_gametype(gametype))
    }
}

pub fn get_name_from_user(label: &str) -> String {
    println!("Please input a name for {label}:");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(e) => {
            println!("Oops. Something went wrong ({e}), please try again");
            get_name_from_user(label)
        }
    }
}

pub fn get_weights_from_user() -> Vec<f64> {
    println!("Please input {NUM_HEURISTICS} Weights for the AI:");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let arr: Vec<f64> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
            match arr.len() {
                NUM_HEURISTICS => arr,
                _ => {
                    println!("Oops. You do not have the correct number of weights");
                    get_weights_from_user()}
            }
        },
        Err(e) => {
            println!("Oops. Something went wrong ({e}), please try again");
            get_weights_from_user()
        }
    }
}

pub fn get_depth_from_user() -> u8 {
    println!("Please input a max depth for the two AI:");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim().parse::<u8>() {
            Ok(i) => i,
            _ => {
                println!("Sorry, couldn't convert that to a valid depth, please try again");
                get_depth_from_user()
            }
        },
        Err(e) => {
            println!("Oops. Something went wrong ({e}), please try again");
            get_depth_from_user()
        }
    }
}
