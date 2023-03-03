use crate::game::board::Move;

use crate::ai::tree::GameTree;

use super::gamestate::State;
use core::fmt::Debug;
use separator::Separatable;

use std::io;
use std::time::Instant;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Person {
    name: String,
}

impl Person {
    pub fn new(name: String) -> Person {
        Person { name }
    }
}

impl Player for Person {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn one_turn(&self, state: &mut State) {
        println!("Input your move:");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match handle_move_input(state, input.trim()) {
                    Ok(_) => (),
                    Err(e) => {
                        println!(
                            "Couldn't process that move ({e}). Please try again"
                        );
                        self.one_turn(state);
                    }
                };
            }
            Err(e) => println!("Oops. Something went wrong ({e})"),
        }

        println!("{state}");
    }
}

fn handle_move_input(
    game: &mut State,
    input: &str,
) -> Result<(), &'static str> {
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

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AI {
    name: String,
}

impl AI {
    pub fn new(name: String) -> AI {
        AI { name }
    }
}

impl Player for AI {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn one_turn(&self, state: &mut State) {
        let depth = 5;
        println!("AI thinking...");
        let old_state = state.clone();

        let before_tree_creation = Instant::now();
        let mut tree = GameTree::new(state.to_owned(), depth);
        tree.build_eval_tree();

        let (Move::Diagonal(origin, dest) | Move::Straight(origin, dest)) =
            tree.rollback(state.current_turn as usize);

        // let (Move::Diagonal(origin, dest) | Move::Straight(origin, dest)) =
        //     get_best_move(state);
        // tree.svg_from_tree();
        println!(
            "AI suggested {}-{} (depth of {}) in {:.2} seconds",
            origin,
            dest,
            depth,
            before_tree_creation.elapsed().as_secs_f32(),
        );
        
        state
            .move_piece(origin, dest, true)
            .expect("could not play the AI-suggested move");
        
        println!(
            "AI's reasoning:\n{:?}",
            tree.weights.difference(&old_state, state)
        );

        println!("{state}");
    }
}

pub trait Player {
    fn one_turn(&self, state: &mut State);

    fn name(&self) -> String;
}

#[derive(Clone, Debug, PartialEq)]
pub enum PossiblePlayer {
    Person(Person),
    AI(AI),
}

impl Default for PossiblePlayer {
    fn default() -> Self {
        PossiblePlayer::Person(Person::default())
    }
}

impl Player for PossiblePlayer {
    fn name(&self) -> String {
        match self {
            PossiblePlayer::Person(p) => p.name(),
            PossiblePlayer::AI(a) => a.name(),
        }
    }

    fn one_turn(&self, state: &mut State) {
        match self {
            PossiblePlayer::Person(p) => p.one_turn(state),
            PossiblePlayer::AI(a) => a.one_turn(state),
        }
    }
}
