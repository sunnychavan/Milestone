use crate::ai::genetic::GeneticAlgorithm;
use crate::ai::heuristics::NUM_HEURISTICS;
use crate::ai::tree::SearchLimit;
use crate::game::gamestate::{GameBuilder, State};

use crate::game::player::PossiblePlayer;

use crate::game::player::{Person, AI};
use std::{env, io};

#[derive(PartialEq)]
enum GameType {
    Black,
    White,
    Humans,
    AIs,
    String,
    Genetic,
}

fn get_gametype_from_user() -> GameType {
    println!(
        "Enter:\n\t(1) to play a game as black vs the AI,\
               \n\t(2) to play a game as white vs the AI,\
               \n\t(3) to play against another human,\
               \n\t(4) to have two AIs play each other\
               \n\t(5) to load from a string, or\
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
            println!("Paste string below:");
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
        _ => {
            panic!("invalid game type");
        }
    }
}

pub fn play_game(mut game: State) {
    println!("{game}");

    while game.active {
        game.play_one_turn();
    }

    println!("{game}");
}

pub fn run_genetic_game(
    black_weights: [f64; NUM_HEURISTICS],
    white_weights: [f64; NUM_HEURISTICS],
) -> State {
    println!("Genetic Algorithm Game Running");

    let gb = GameBuilder::new();
    // game between two AI
    gb.set_player_1(PossiblePlayer::AI(AI::new(
        "AI 1".to_string(),
        black_weights,
        SearchLimit::Depth(4),
    )))
    .set_player_2(PossiblePlayer::AI(AI::new(
        "AI 2".to_string(),
        white_weights,
        SearchLimit::Depth(4),
    )))
    .build()
}

pub fn play_genetic_game(
    black_weights: [f64; NUM_HEURISTICS],
    white_weights: [f64; NUM_HEURISTICS],
) -> Option<u8> {
    let mut game = run_genetic_game(black_weights, white_weights);

    println!("{game}");

    while game.active {
        game.play_one_turn();
    }

    println!("{game}");

    game.winner
}

pub fn start_genetic_process() {
    GeneticAlgorithm::new().run();
}

pub fn choose_phase() {
    let gametype = match env::var("LAUNCH_ARG") {
        Ok(i) => {
            println!("Using argument {i} from environment variable.");
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
