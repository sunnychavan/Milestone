use super::game::gamestate::State;

struct GameNode {
    children: Vec<GameNode>,
    state: State,
    depth: u8,
    evaluation: Some(u64),
}

impl GameNode {
    pub fn add_child_from_state(&self, state: State) {
        &self.children.append(GameNode {
            children: vec![],
            state: state,
            depth: self.depth + 1,
            evaluation: None,
        })
    }
}
