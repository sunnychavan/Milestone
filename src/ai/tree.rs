use std::iter::Iterator;

use super::super::game::board::Move::{Diagonal, Straight};
use super::super::game::gamestate::State;

pub struct GameNode {
    children: Vec<GameNode>,
    state: State,
    depth: u8,
    evaluation: Option<u64>,
}

pub fn create_new_tree(state: &State, max_depth: u8) {
    let mut root_node = GameNode {
        children: vec![],
        state: state.clone(),
        depth: 0,
        evaluation: None,
    };

    add_all_possible_children(&mut root_node, max_depth)
}

fn add_all_possible_children(root: &mut GameNode, max_depth: u8) {
    let possible_moves = root.state.current_possible_moves();
    for mv in possible_moves {
        let (Diagonal(origin, dest) | Straight(origin, dest)) = mv;
        match root.state.can_move(origin, dest) {
            Ok(_) => {
                println!("{}, {}", origin, dest);
                // add new child
                let new_state = root
                    .state
                    .clone()
                    .move_piece(origin, dest, true)
                    .unwrap()
                    .to_owned();
                root.add_child_from_state(new_state, max_depth);
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
        evaluation: Option<u64>,
    ) -> GameNode {
        GameNode {
            children: children.unwrap_or(vec![]),
            state: state,
            depth: depth.unwrap_or(0),
            evaluation: evaluation,
        }
    }

    fn add_child_from_state(&mut self, state: State, max_depth: u8) {
        let mut new_child = GameNode {
            children: vec![],
            state,
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
            result = self
                .children
                .iter()
                .map(|elt| match elt.evaluation {
                    Some(v) => v,
                    None => u64::MAX,
                })
                .min()
                .unwrap_or(u64::MAX);
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
            result = self
                .children
                .iter()
                .map(|elt| match elt.evaluation {
                    Some(v) => v,
                    None => u64::MIN,
                })
                .max()
                .unwrap_or(u64::MIN);
        }
        self.evaluation = Some(result);
        result
    }

    fn evaluate(&mut self) -> u64 {
        match self.evaluation {
            Some(v) => v,
            None => {
                // TODO: need to actually put a value here
                self.evaluation = Some(7);
                7
            }
        }
    }
}
