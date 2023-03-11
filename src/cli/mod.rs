use crate::ai::heuristics::NUM_HEURISTICS;
use crate::ai::tree::SearchLimit;
use crate::game::gamestate::{GameBuilder, State};

use crate::game::player::PossiblePlayer;

use crate::game::player::{Person, AI};
use std::io;

pub fn choose_type_of_game() -> State {
    println!(
        "Enter:\n\t(1) to play a game as black vs the AI,\
               \n\t(2) to play a game as white vs the AI,\
               \n\t(3) to play against another human, or\
               \n\t(0) to have two AIs play each other"
    );

    let gb = GameBuilder::new();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.as_str().trim() {
                "1" => {
                    // game as black
                    let player_name = get_name_from_user("yourself");

                    gb.set_player_1(PossiblePlayer::Person(Person::new(
                        player_name,
                    )))
                    .set_player_2(PossiblePlayer::AI(AI::from_name(
                        "AI".to_string(),
                    )))
                    .build()
                }
                "2" => {
                    // game as white
                    let player_name = get_name_from_user("yourself");

                    gb.set_player_1(PossiblePlayer::AI(AI::from_name(
                        "AI".to_string(),
                    )))
                    .set_player_2(PossiblePlayer::Person(Person::new(
                        player_name,
                    )))
                    .build()
                }
                "3" => {
                    // game between two humans
                    let player_1_name = get_name_from_user("player 1");
                    let player_2_name = get_name_from_user("player 2");

                    gb.set_player_1(PossiblePlayer::Person(Person::new(
                        player_1_name,
                    )))
                    .set_player_2(PossiblePlayer::Person(Person::new(
                        player_2_name,
                    )))
                    .build()
                }
                "0" => {
                    let depth = get_depth_from_user();
                    // game between two AI
                    gb.set_player_1(PossiblePlayer::AI(AI::new(
                        "AI 1".to_string(),
                        [1; NUM_HEURISTICS],
                        SearchLimit::Depth(depth),
                    )))
                    .set_player_2(PossiblePlayer::AI(AI::new(
                        "AI 2".to_string(),
                        [1; NUM_HEURISTICS],
                        SearchLimit::Depth(depth),
                    )))
                    .build()
                }
                _ => {
                    println!("Oops. That isn't a valid input, try again:");
                    choose_type_of_game()
                }
            }
        }
        Err(e) => {
            println!("Oops. Something went wrong ({e})");
            choose_type_of_game()
        }
    }
}

pub fn play_game() {
    let mut game = choose_type_of_game();

    println!("{game}");

    while game.active {
        game.play_one_turn();
    }

    println!("{game}");
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
