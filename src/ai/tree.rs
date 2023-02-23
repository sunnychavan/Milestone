use std::iter::Iterator;
use std::ops::Div;
use std::{fmt, num, thread};

use crate::game::board::Move;

use super::super::game::board::Move::{Diagonal, Straight};
use super::super::game::gamestate::State;
use super::heuristics::{
    middle_proximity, number_of_pieces, piece_differential, win_lose_condition, hold_important_pieces
};

pub struct GameNode {
    children: Vec<GameNode>,
    state: State,
    recent_move: Option<Move>,
    best_next_move: Option<Move>,
    pub total_subnodes: u64,
    depth: u8,
    evaluation: Option<i8>,
}

impl fmt::Debug for GameNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = "".to_owned();
        repr.push_str(&format!("{:?}\n", self.state));
        repr.push_str(&format!(
            "Possible Moves: {:?}\n",
            self.children
                .iter()
                .map(|child| { child.recent_move })
                .collect::<Vec<Option<Move>>>()
        ));
        repr.push_str(&format!(
            "Best Move: {:?} - {:?}\n",
            self.best_next_move, self.evaluation
        ));
        repr.push_str(&format!(
            "Subnodes Considered: {:?}\n",
            self.total_subnodes
        ));
        write!(f, "{}", repr)
    }
}

impl fmt::Display for GameNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = "".to_owned();
        repr.push_str(&format!("{:?}\n", self.state));
        repr.push_str(&format!(
            "Best Move: {:?} - {:?}\n",
            self.best_next_move, self.evaluation
        ));
        repr.push_str(&format!(
            "Subnodes Considered: {:?}\n",
            self.total_subnodes
        ));
        write!(f, "{}", repr)
    }
}

pub fn create_eval_tree(state: &State, max_depth: u8) -> GameNode {
    let mut root_node = GameNode {
        children: vec![],
        state: state.clone(),
        recent_move: None,
        best_next_move: None,
        total_subnodes: 0,
        depth: 0,
        evaluation: None,
    };

    add_all_possible_children_mt(&mut root_node, max_depth);
    // add_all_possible_children(&mut root_node, max_depth);
    root_node
}

// the multithreaded implementation of the function with similar name
fn add_all_possible_children_mt(root: &mut GameNode, max_depth: u8) -> u64 {
    let mut subtrees = vec![];

    let possible_moves = root.state.current_possible_moves();

    for mv in possible_moves {
        let (m @ Diagonal(origin, dest) | m @ Straight(origin, dest)) = mv;
        match root.state.can_move(origin, dest) {
            // for each possible move, add a new child
            Ok(_) => {
                // add new child
                let new_state = root
                    .state
                    .clone()
                    .move_piece(origin, dest, true)
                    .unwrap()
                    .to_owned();
                subtrees.push(thread::spawn(move || -> GameNode {
                    let mut new_root =
                        GameNode::new(None, new_state, Some(1), Some(mv), None);
                    add_all_possible_children(&mut new_root, max_depth);
                    new_root
                }))
            }
            Err(_) => {
                // don't do anything
            }
        }
    }
    let final_result: Vec<GameNode> =
        subtrees.into_iter().map(|n| n.join().unwrap()).collect();

    let num_added = final_result
        .iter()
        .map(|elt| elt.total_subnodes + 1)
        .sum::<u64>();

    root.children = final_result;
    root.total_subnodes += num_added;

    num_added
}

// the return value is the number of children created
fn add_all_possible_children(root: &mut GameNode, max_depth: u8) -> u64 {
    let possible_moves = root.state.current_possible_moves();
    for mv in possible_moves {
        let (m @ Diagonal(origin, dest) | m @ Straight(origin, dest)) = mv;
        // for each possible move, add a new child
        match root.state.can_move(origin, dest) {
            Ok(_) => {
                // add new child
                let new_state = root
                    .state
                    .clone()
                    .move_piece(origin, dest, true)
                    .unwrap()
                    .to_owned();
                root.add_child_from_state(new_state, m);
            }
            Err(_) => {
                // don't do anything
            }
        }
    }
    // add all possible children to that child, abiding by max depth
    let mut new_children = 0;
    if root.depth < max_depth - 1 {
        new_children = root
            .children
            .iter_mut()
            .map(|child| add_all_possible_children(child, max_depth))
            .sum::<u64>();
    }
    root.total_subnodes += new_children;
    root.total_subnodes
}

impl GameNode {
    pub fn new(
        children: Option<Vec<GameNode>>,
        state: State,
        depth: Option<u8>,
        recent_move: Option<Move>,
        evaluation: Option<i8>,
    ) -> GameNode {
        GameNode {
            total_subnodes: match &children {
                Some(c) => c.iter().map(|elt| 1 + elt.total_subnodes).sum(),
                None => 0,
            },
            children: children.unwrap_or(vec![]),
            state: state,
            recent_move: recent_move,
            best_next_move: None,
            depth: depth.unwrap_or(0),
            evaluation: evaluation,
        }
    }

    // the return value is the number of children created
    fn add_child_from_state(&mut self, state: State, mv: Move) {
        let new_child = GameNode {
            children: vec![],
            state,
            recent_move: Some(mv),
            best_next_move: None,
            total_subnodes: 0,
            depth: self.depth + 1,
            evaluation: None,
        };

        self.children.push(new_child);
        self.total_subnodes += 1;
    }

    pub fn rollback(&mut self) -> i8 {
        self.max_value()
    }

    pub fn get_best_move(&mut self) -> Move {
        self.rollback();
        self.best_next_move.unwrap()
    }

    fn min_value(&mut self) -> i8 {
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
                    None => (i8::MAX, elt.recent_move),
                })
                .min_by(|(x_eval, _), (y_eval, _)| x_eval.cmp(y_eval))
                .unwrap();

            self.best_next_move = best_move;
        }
        self.evaluation = Some(result);
        result
    }

    fn max_value(&mut self) -> i8 {
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
                    None => (i8::MIN, elt.recent_move),
                })
                .max_by(|(x_eval, _), (y_eval, _)| x_eval.cmp(y_eval))
                .unwrap();

            self.best_next_move = best_move;
        }
        self.evaluation = Some(result);
        result
    }

    fn evaluate(&mut self) -> i8 {
        match self.evaluation {
            Some(v) => v,
            None => {
                let result = (win_lose_condition(&self.state, self.state.current_turn).div(3)
                    + middle_proximity(&self.state, self.state.current_turn).div(3)
                    + piece_differential(&self.state, self.state.current_turn).div(3)
                + hold_important_pieces(&self.state, self.state.current_turn).div(3));
                self.evaluation = Some(result);
                result
            }
        }
    }
}
