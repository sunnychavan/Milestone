use super::{board::Board, player::Player};
use std::fmt;

pub struct State {
    active: bool,
    current_turn: usize,
    board: Board,
    players: [Player; 2],
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = "Milestone: {\n".to_owned();

        repr.push_str(&format!("active: {:?}\n", self.active));

        let current_player = &self.players[self.current_turn];
        repr.push_str(&format!(
            "current_turn: {} {:?}\n",
            current_player.name, current_player.pieces
        ));

        repr.push_str(&format!("board: {:?}\n", self.board));

        repr.push_str(&format!("players: [\n"));
        repr.push_str(&format!("  {:?},\n", self.players[0]));
        repr.push_str(&format!("  {:?}\n", self.players[1]));
        repr.push_str(&format!("]"));
        repr.push_str(&"}");

        write!(f, "{}", repr)
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = "Milestone:\n".to_owned();

        let current_player = &self.players[self.current_turn];
        repr.push_str(&format!(
            "  current_turn: {} {:?}\n",
            current_player.name, current_player.pieces
        ));

        repr.push_str(&format!("  board: {}", self.board));

        write!(f, "{}", repr)
    }
}

impl State {
    pub fn new(players: [Player; 2]) -> State {
        State {
            active: true,
            current_turn: 0,
            board: Board::new(),
            players: players,
        }
    }
}
