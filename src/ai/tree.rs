use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;
use petgraph::visit::{Dfs, EdgeRef};
use petgraph::Graph;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::Command;
use std::process::Stdio;

use std::iter::Iterator;
use std::ops::{Div, Index, IndexMut};
use std::{fmt, num, thread};

use crate::game::board::Move;

use super::super::game::board::Move::{Diagonal, Straight};
use super::super::game::gamestate::State;
use super::heuristics::{
    hold_important_pieces, middle_piece_differential, middle_proximity,
    number_of_pieces, piece_differential, win_lose_condition,
};

#[derive(Debug, Clone)]
pub struct GameTree {
    tree: DiGraph<GameNode, Move>,
    tree_root_idx: NodeIndex,
    base_state: State,
    max_depth: u8,
    best_move: Option<Move>,
    evaluation: Option<i8>,
}

#[derive(Debug, Clone)]
pub struct GameNode {
    state: State,
    depth: u8,
    evaluation: Option<i8>,
}

impl GameTree {
    pub fn new(base_state: State, max_depth: u8) -> GameTree {
        let mut tree = DiGraph::<GameNode, Move>::new();

        GameTree {
            tree_root_idx: tree.add_node(GameNode::new(0, base_state.clone())),
            tree,
            base_state: base_state,
            max_depth,
            best_move: None,
            evaluation: None,
        }
    }

    pub fn svg_from_tree(&self) {
        self.write_to_file(
            File::create("input.dot").expect("failed to create input file"),
        );
        create_svg_from_file(
            "input.dot",
            File::create("output.svg").expect("failed to create output file"),
        );
    }

    fn write_to_file(&self, mut file: File) {
        let dot = Dot::with_attr_getters(
            &self.tree,
            &[],
            &|_, _| "".to_owned(),
            &|_, (ni, gamenode)| format!("label = \"{}\"", gamenode.evaluate()),
        );
        // let dot = Dot::new(&self.tree);
        write!(file, "{:?}", dot).expect("failed to write to input file");
    }

    pub fn build_eval_tree(&mut self) {
        self.add_all_possible_children(self.tree_root_idx)
    }

    fn add_all_possible_children(&mut self, root: NodeIndex) {
        let root_node = self.tree.index(root).clone();
        if root_node.depth >= self.max_depth {
            return;
        }

        let possible_moves = root_node.state.current_possible_moves();
        for m @ Straight(origin, dest) | m @ Diagonal(origin, dest) in
            possible_moves
        {
            let mut new_state = root_node.state.clone();
            new_state.move_piece(origin, dest, true).unwrap();

            let new_node = self
                .tree
                .add_node(GameNode::new(root_node.depth + 1, new_state));
            self.tree.add_edge(root, new_node, m);

            self.add_all_possible_children(new_node);
        }
    }

    fn max_value(&self, root_idx: NodeIndex) -> (i8, Option<&Move>) {
        let result;

        let mut outgoing_moves = self.tree.edges(root_idx).peekable();
        // this node is a leaf node / at max-depth:
        if outgoing_moves.peek().is_none() {
            let root_node = self.tree.index(root_idx);
            return (root_node.evaluate(), None);
        } else {
            // to maximize this node, minimize its children
            let best_move;
            (result, best_move) = self
                .tree
                .edges(root_idx)
                .map(|edge| {
                    let d = edge.target();
                    (self.min_value(d).0, edge.weight())
                })
                .max_by_key(|elt| elt.0)
                .unwrap();
            return (result, Some(&best_move));
        }
    }

    fn min_value(&self, root_idx: NodeIndex) -> (i8, Option<&Move>) {
        let result;

        let mut outgoing_moves = self.tree.edges(root_idx).peekable();
        // this node is a leaf node / at max-depth:
        if outgoing_moves.peek().is_none() {
            let root_node = self.tree.index(root_idx);
            return (root_node.evaluate(), None);
        } else {
            // to minimize this node, maximize its children
            let best_move;
            (result, best_move) = self
                .tree
                .edges(root_idx)
                .map(|edge| {
                    let d = edge.target();
                    (self.max_value(d).0, edge.weight())
                })
                .min_by_key(|elt| elt.0)
                .unwrap();
            return (result, Some(&best_move));
        }
    }

    pub fn rollback(&mut self) -> Move {
        self.max_value(self.tree_root_idx)
            .1
            .expect("the best value didn't have an associated move")
            .to_owned()
    }

    pub fn total_subnodes(&self) -> usize {
        self.tree.edge_count()
    }
}

impl GameNode {
    pub fn new(depth: u8, state: State) -> GameNode {
        GameNode {
            depth,
            evaluation: None,
            state,
        }
    }

    fn evaluate(&self) -> i8 {
        return (win_lose_condition(&self.state, self.state.current_turn)
            .div(5)
            + middle_proximity(&self.state, self.state.current_turn).div(5)
            + middle_piece_differential(&self.state, self.state.current_turn)
                .div(5)
            + piece_differential(&self.state, self.state.current_turn).div(5)
            + hold_important_pieces(&self.state, self.state.current_turn)
                .div(5));
    }
}

fn create_svg_from_file(dot_file: &str, svg_file: File) {
    Command::new("dot")
        .arg("-Tsvg")
        .arg(dot_file)
        .stdout(Stdio::from(svg_file))
        .spawn()
        .expect("failed to launch dot process");
}
