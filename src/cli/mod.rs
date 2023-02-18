use super::game;
use super::game::gamestate::State;
use std::io;

pub fn two_player_game() {
    let mut game = game::new_game();
    println!("{:?}", game);

    while game.active {
        one_turn(&mut game);
    }
}

fn one_turn(game: &mut State) {
    println!("Input your move:");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match handle_move_input(game, &input.trim()) {
                Ok(_) => (),
                Err(e) => {
                    println!("Couldn't process that move ({}). Please try again", e);
                    one_turn(game);
                }
            };
        }
        Err(e) => println!("Oops. Something went wrong ({})", e),
    }

    println!("{:?}", game);
}

fn handle_move_input<'a>(game: &'a mut State, input: &str) -> Result<&'a State, &'static str> {
    match input.split('-').collect::<Vec<&str>>()[..] {
        [a, b] => {
            let from = a.parse::<usize>();
            let to = b.parse::<usize>();
            match (from, to) {
                (Ok(origin), Ok(dest)) => game.move_piece(origin, dest, true),
                _ => Err("couldn't parse your move"),
            }
        }
        _ => Err("improperly formatted move"),
    }
}
