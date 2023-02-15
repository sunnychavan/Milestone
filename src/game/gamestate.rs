use super::{board::Board, player::Player};

#[derive(Debug)]
pub struct State {
    active: bool,
    current_turn: u8,
    board: Board,
    players: (Player, Player),
}

impl State {
    pub fn new(players: (Player, Player)) -> State {
        State {
            active: true,
            current_turn: 0,
            board: Board::new(),
            players: players,
        }
    }
}
