use crate::game::board::Move;

use super::super::ai::tree::create_eval_tree;

use super::{gamestate::State, pieces::Piece};
use core::fmt::Debug;
use std::io;

#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    pieces: Piece,
}

impl Person {
    pub fn new(name: String, pieces: Piece) -> Person {
        Person { name, pieces }
    }
}

impl Player for Person {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_pieces_type(&self) -> Piece {
        self.pieces
    }

    fn one_turn(&self, state: &mut State) {
        println!("Input your move:");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match handle_move_input(state, &input.trim()) {
                    Ok(_) => (),
                    Err(e) => {
                        println!(
                            "Couldn't process that move ({}). Please try again",
                            e
                        );
                        self.one_turn(state);
                    }
                };
            }
            Err(e) => println!("Oops. Something went wrong ({})", e),
        }

        println!("{:?}", state);
    }
}

fn handle_move_input<'a>(
    game: &'a mut State,
    input: &str,
) -> Result<&'a State, &'static str> {
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

#[derive(Debug, Clone)]
pub struct AI {
    name: String,
    pieces: Piece,
}

impl AI {
    pub fn new(name: String, pieces: Piece) -> AI {
        AI { name, pieces }
    }
}

impl Player for AI {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn get_pieces_type(&self) -> Piece {
        self.pieces
    }

    fn one_turn(&self, state: &mut State) {
        let depth = 7;
        println!("AI thinking...)");
        let mut tree = create_eval_tree(state, 4);
        let (Move::Diagonal(origin, dest) | Move::Straight(origin, dest)) =
            tree.get_best_move();
        println!(
            "AI suggested {}-{} (depth of {}, {} total nodes)",
            origin, dest, depth, tree.total_subnodes
        );
        state
            .move_piece(origin, dest, true)
            .expect("could not play the AI-suggested move");
        println!("{:?}", state);
    }
}

pub trait Player {
    // fn new(name: String, pieces: Piece) -> Self
    // where
    //     Self: Sized;

    fn one_turn(&self, state: &mut State);

    fn name(&self) -> String;

    fn get_pieces_type(&self) -> Piece;
}

#[derive(Clone, Debug)]
pub enum PossiblePlayer {
    Person(Person),
    AI(AI),
}

impl Player for PossiblePlayer {
    fn name(&self) -> String {
        match self {
            PossiblePlayer::Person(p) => p.name(),
            PossiblePlayer::AI(a) => a.name(),
        }
    }

    fn get_pieces_type(&self) -> Piece {
        match self {
            PossiblePlayer::Person(p) => p.get_pieces_type(),
            PossiblePlayer::AI(a) => a.get_pieces_type(),
        }
    }

    fn one_turn(&self, state: &mut State) {
        match self {
            PossiblePlayer::Person(p) => p.one_turn(state),
            PossiblePlayer::AI(a) => a.one_turn(state),
        }
    }
}
