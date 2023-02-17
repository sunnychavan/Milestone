use super::{board::Board, board::Hole, board::Move, pieces::Piece, player::Player};
use std::fmt;

pub struct State {
    pub active: bool,
    current_turn: u8,
    board: Board,
    players: [Player; 2],
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = "Milestone: {\n".to_owned();

        repr.push_str(&format!("active: {:?}\n", self.active));

        let current_player = &self.players[usize::from(self.current_turn)];
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

        let current_player: &Player = &self.players[usize::from(self.current_turn)];
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

    pub fn move_piece(
        &mut self,
        from: usize,
        to: usize,
        capture: bool,
    ) -> Result<&State, &'static str> {
        let current_player_pieces = self.players[usize::from(self.current_turn)].pieces;

        match self.board.possible_move(&from, &to, self.current_turn) {
            Some(m) => match self.board.board[to].0 {
                Some(existing_piece) if existing_piece == current_player_pieces => {
                    Err("can't occupy the same space as another one of your pieces")
                }
                Some(_) => match (m, capture) {
                    (Move::Straight(_), true) => {
                        self.move_piece_aux(from, to, current_player_pieces);
                        Ok(self)
                    }
                    (Move::Diagonal(_), _) => Err("must capture head-on"),
                    (_, false) => Err("you do not have permission to captue"),
                },
                None => {
                    self.move_piece_aux(from, to, current_player_pieces);
                    Ok(self)
                }
            },
            None => Err("not a legal move for this piece or it's not your turn"),
        }
    }

    fn move_piece_aux(&mut self, from: usize, to: usize, current_player_pieces: Piece) {
        // return state with piece moved
        self.board.board[from] = Hole(None);
        self.board.board[to] = Hole(Some(current_player_pieces));

        // check if game ends
        if to == 0 || to == 36 {
            self.active = false;
        } else {
            // one move per turn
            self.current_turn = 1 - self.current_turn;
        }
    }
}
