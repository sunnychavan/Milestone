use crate::game::pieces::Piece;
use crate::game::player::{Player, PossiblePlayer};

use super::game::gamestate::State;
use crate::game::player::{Person, AI};
use std::io;

pub fn two_player_game() {
    let player_pieces = choose_player_order();
    let player_name = get_name_from_user();

    let players: [PossiblePlayer; 2] = match player_pieces {
        Piece::Black => [
            PossiblePlayer::Person(Person::new(player_name, Piece::Black)),
            PossiblePlayer::AI(AI::new("AI".to_string(), Piece::White)),
        ],
        Piece::White => [
            PossiblePlayer::AI(AI::new("AI".to_string(), Piece::Black)),
            PossiblePlayer::Person(Person::new(player_name, Piece::White)),
        ],
    };

    let mut game = State::new(&players);

    println!("{:?}", game);

    while game.active {
        players[game.current_turn as usize].one_turn(&mut game);
    }
}

pub fn choose_player_order() -> Piece {
    println!("Enter (1) to play as black or (2) to play as white:");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.as_str().trim() {
            "1" => Piece::Black,
            "2" => Piece::White,
            s => {
                println!("Oops, {} is not a valid input. Please try again.", s);
                choose_player_order()
            }
        },
        Err(e) => {
            println!("Oops. Something went wrong ({})", e);
            choose_player_order()
        }
    }
}

pub fn get_name_from_user() -> String {
    println!("Please input a name for yourself:");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_string(),
        Err(e) => {
            println!("Oops. Something went wrong ({})", e);
            get_name_from_user()
        }
    }
}
