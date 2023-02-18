use super::super::game::board::Move::{Diagonal, Straight};
use super::super::game::gamestate::State;

pub struct GameNode<'a> {
    children: Vec<GameNode<'a>>,
    state: &'a State,
    depth: u8,
    evaluation: Option<u64>,
}

impl GameNode<'_> {
    pub fn create_new_tree(state: &State, depth: u8) {
        let root_node = GameNode {
            children: vec![],
            state: state,
            depth: 0,
            evaluation: None,
        };
        let mut current_depth = 0;

        // TODO: temporary depth for testing
        while current_depth <= 0 {
            let possible_moves = state.current_possible_moves();
            for mv in possible_moves {
                let (Diagonal(origin, dest) | Straight(origin, dest)) = mv;
                match state.can_move(origin, dest) {
                    Ok(_) => {
                        println!("{}, {}", origin, dest)
                        // add new child
                    }
                    Err(_) => {
                        // don't do anything
                    }
                }
            }
            current_depth += 1;
        }
    }

    // pub fn add_child_from_state(&mut self, state: &State) {
    //     self.children.push(GameNode {
    //         children: vec![],
    //         state,
    //         depth: self.depth + 1,
    //         evaluation: None,
    //     })
    // }
}
