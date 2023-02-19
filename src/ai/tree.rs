use std::fmt;
use std::iter::Iterator;

use crate::game::board::Move;

use super::super::game::board::Move::{Diagonal, Straight};
use super::super::game::gamestate::State;
use super::heuristics::number_of_pieces;

pub struct GameNode {
    children: Vec<GameNode>,
    state: State,
    recent_move: Option<Move>,
    best_next_move: Option<Move>,
    depth: u8,
    evaluation: Option<u64>,
}

impl fmt::Debug for GameNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = "Node {".to_owned();
        repr.push_str(&format!("{:?}\n", self.state));
        repr.push_str(&format!("{:?}\n", self.best_next_move));
        repr.push_str("}");
        write!(f, "{}", repr)
    }
}

pub fn create_new_tree(state: &State, max_depth: u8) -> GameNode {
    let mut root_node = GameNode {
        children: vec![],
        state: state.clone(),
        recent_move: None,
        best_next_move: None,
        depth: 0,
        evaluation: None,
    };

    add_all_possible_children(&mut root_node, max_depth);
    root_node
}

fn add_all_possible_children(root: &mut GameNode, max_depth: u8) {
    let possible_moves = root.state.current_possible_moves();
    for mv in possible_moves {
        let (m @ Diagonal(origin, dest) | m @ Straight(origin, dest)) = mv;
        match root.state.can_move(origin, dest) {
            Ok(_) => {
                // add new child
                let new_state = root
                    .state
                    .clone()
                    .move_piece(origin, dest, true)
                    .unwrap()
                    .to_owned();
                root.add_child_from_state(new_state, m, max_depth);
            }
            Err(_) => {
                // don't do anything
            }
        }
    }
}

impl GameNode {
    pub fn new(
        children: Option<Vec<GameNode>>,
        state: State,
        depth: Option<u8>,
        recent_move: Option<Move>,
        evaluation: Option<u64>,
    ) -> GameNode {
        GameNode {
            children: children.unwrap_or(vec![]),
            state: state,
            recent_move: recent_move,
            best_next_move: None,
            depth: depth.unwrap_or(0),
            evaluation: evaluation,
        }
    }

    fn add_child_from_state(&mut self, state: State, mv: Move, max_depth: u8) {
        let mut new_child = GameNode {
            children: vec![],
            state,
            recent_move: Some(mv),
            best_next_move: None,
            depth: self.depth + 1,
            evaluation: None,
        };

        // recurse on new child
        if new_child.depth < max_depth {
            add_all_possible_children(&mut new_child, max_depth);
        }
        self.children.push(new_child);
    }

    pub fn rollback(&mut self) -> u64 {
        self.max_value()
    }

    fn min_value(&mut self) -> u64 {
        let result;

        if self.children.len() == 0 {
            result = match self.evaluation {
                Some(v) => v,
                None => self.evaluate(),
            }
        } else {
            for child in self.children.iter_mut() {
                child.max_value();
            }
            let best_move;
            (result, best_move) = self
                .children
                .iter()
                .map(|elt| match elt.evaluation {
                    Some(v) => (v, elt.recent_move),
                    None => (u64::MAX, elt.recent_move),
                })
                .min_by(|(x_eval, _), (y_eval, _)| x_eval.cmp(y_eval))
                .unwrap();

            self.best_next_move = best_move;
        }
        self.evaluation = Some(result);
        result
    }

    fn max_value(&mut self) -> u64 {
        let result;

        if self.children.len() == 0 {
            result = match self.evaluation {
                Some(v) => v,
                None => self.evaluate(),
            }
        } else {
            for child in self.children.iter_mut() {
                child.min_value();
            }
            let best_move;
            (result, best_move) = self
                .children
                .iter()
                .map(|elt| match elt.evaluation {
                    Some(v) => (v, elt.recent_move),
                    None => (u64::MIN, elt.recent_move),
                })
                .max_by(|(x_eval, _), (y_eval, _)| x_eval.cmp(y_eval))
                .unwrap();

            self.best_next_move = best_move;
        }
        self.evaluation = Some(result);
        result
    }

    fn evaluate(&mut self) -> u64 {
        match self.evaluation {
            Some(v) => v,
            None => {
                let result = number_of_pieces(&self.state);
                self.evaluation = Some(result);
                result
            }
        }
    }
}
