use petgraph::dot::Dot;
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use separator::Separatable;

use std::fmt::Debug;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;
use std::time::Instant;

use std::iter::Iterator;
use std::ops::Index;
use std::time::Duration;

use crate::ai::heuristics::HeuristicWeightsWithTwoStates;
use crate::game::board::Move;

use super::super::game::board::Move::{Diagonal, Straight};
use super::super::game::gamestate::State;
use super::heuristics::HeuristicWeights;

#[derive(Debug, Clone)]
pub struct GameTree {
    tree: DiGraph<GameNode, Move>,
    tree_root_idx: NodeIndex,
    max_depth: u8,
    pub weights: HeuristicWeights,
}

#[derive(Debug, Clone)]
pub struct GameNode {
    state: State,
    depth: u8,
}

impl GameTree {
    pub fn new(base_state: State, max_depth: u8) -> GameTree {
        let mut tree = DiGraph::<GameNode, Move>::new();

        GameTree {
            tree_root_idx: tree.add_node(GameNode::new(0, base_state)),
            tree,
            max_depth,
            weights: HeuristicWeights::new([1, 1, 1, 1, 1, 1, 1, 1, 1]),
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
            &|_, _| "".to_owned(),
            // &|_t, (_ni, gn)| {
            //     format!(
            //         "label = \"Heuristic: {:?}\"",
            //         HeuristicWeights::new_with_state(&self.weights, &gn.state,)
            //     )
            // },
        );
        write!(file, "{dot:?}").expect("failed to write to input file");
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

    fn max_value(
        &self,
        root_idx: NodeIndex,
        mut alpha: i64,
        beta: i64,
    ) -> (i64, Option<&Move>) {
        let mut outgoing_moves = self.tree.edges(root_idx).peekable();
        // this node is a leaf node / at max-depth:
        if outgoing_moves.peek().is_none() {
            let root_node = self.tree.index(root_idx);
            (root_node.evaluate(self), None)
        } else {
            // to maximize this node, minimize its children
            let mut best_score: i64 = i64::MIN;
            let mut best_move: Option<&Move> = None;

            for edge in outgoing_moves {
                let d = edge.target();
                let new_score = self.min_value(d, alpha, beta).0;
                let new_move = Some(edge.weight());

                if new_score > best_score {
                    best_score = new_score;
                    best_move = new_move;
                }

                // alpha / beta
                alpha = std::cmp::max(best_score, alpha);
                if alpha >= beta {
                    break;
                }
            }

            (best_score, best_move)
        }
    }

    fn min_value(
        &self,
        root_idx: NodeIndex,
        alpha: i64,
        mut beta: i64,
    ) -> (i64, Option<&Move>) {
        let mut outgoing_moves = self.tree.edges(root_idx).peekable();
        // this node is a leaf node / at max-depth:
        if outgoing_moves.peek().is_none() {
            let root_node = self.tree.index(root_idx);
            (root_node.evaluate(self), None)
        } else {
            // to minimize this node, maximize its children
            let mut best_score: i64 = i64::MAX;
            let mut best_move: Option<&Move> = None;

            for edge in outgoing_moves {
                let d = edge.target();
                let new_score = self.max_value(d, alpha, beta).0;
                let new_move = Some(edge.weight());

                if new_score < best_score {
                    best_score = new_score;
                    best_move = new_move;
                }

                // alpha / beta
                beta = std::cmp::min(best_score, beta);
                if alpha >= beta {
                    break;
                }
            }

            (best_score, best_move)
        }
    }

    pub fn rollback(&mut self, player_idx: usize) -> Move {
        match player_idx {
            0 => self
                .max_value(self.tree_root_idx, i64::MIN, i64::MAX)
                .1
                .expect("the best value didn't have an associated move")
                .to_owned(),
            1 => self
                .min_value(self.tree_root_idx, i64::MIN, i64::MAX)
                .1
                .expect("the best value didn't have an associated move")
                .to_owned(),
            _ => panic!(
                "player index must be confined. this is a two person game"
            ),
        }
    }

    pub fn total_subnodes(&self) -> usize {
        self.tree.edge_count()
    }
}

impl GameNode {
    pub fn new(depth: u8, state: State) -> GameNode {
        GameNode { depth, state }
    }

    fn evaluate(&self, tree: &GameTree) -> i64 {
        tree.weights.score(&self.state)
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

pub fn get_best_move(state: &State, time_limit: Duration) -> SuggestedMove {
    let function_beginning = Instant::now();
    let mut depth_to_search = 0;
    let mut best_move_opt: Option<Move> = None;
    let mut best_tree_opt: Option<GameTree> = None;
    let mut time_building = Duration::ZERO;
    let mut time_evaluating = Duration::ZERO;

    while Instant::now().duration_since(function_beginning) < time_limit {
        depth_to_search += 1;

        let before_building_tree = Instant::now();
        let mut tree = GameTree::new(state.to_owned(), depth_to_search);
        tree.build_eval_tree();
        let after_building_tree = Instant::now();
        let (m @ Move::Diagonal(_, _) | m @ Move::Straight(_, _)) =
            tree.rollback(state.current_turn as usize);
        let after_evaluating_tree = Instant::now();

        best_move_opt = Some(m);
        best_tree_opt = Some(tree);
        time_building +=
            after_building_tree.duration_since(before_building_tree);
        time_evaluating +=
            after_evaluating_tree.duration_since(after_building_tree);
    }

    let best_move = best_move_opt.expect("could not find a best move in time");
    let best_tree = best_tree_opt.expect("could not find a best tree in time");

    SuggestedMove {
        suggestion: best_move,
        max_depth_considered: depth_to_search,
        time_building_trees: time_building,
        time_evaluating_trees: time_evaluating,
        total_nodes_considered: best_tree.total_subnodes(),
        heuristical_reasoning: best_tree
            .weights
            .new_with_state_and_move(state.clone(), best_move),
    }
}

pub struct SuggestedMove {
    pub suggestion: Move,
    max_depth_considered: u8,
    time_building_trees: Duration,
    time_evaluating_trees: Duration,
    total_nodes_considered: usize,
    heuristical_reasoning: HeuristicWeightsWithTwoStates,
}

impl Debug for SuggestedMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "AI suggested {:?} ({} nodes considered, depth of {}) in {:.2} seconds ({:.2} to build, {:.2} to evaluate) with reasoning: {:#?}",
            self.suggestion,
            self.total_nodes_considered.separated_string(),
            self.max_depth_considered,
            (self.time_building_trees + self.time_evaluating_trees)
                .as_secs_f32(),
            self.time_building_trees.as_secs_f32(),
            self.time_evaluating_trees.as_secs_f32(),
            self.heuristical_reasoning,
        ))
    }
}
