use super::{
    board::Board, board::Hole, board::Move, pieces::Piece, player::Player,
    player::PossiblePlayer,
};
use std::fmt;

#[derive(Clone)]
pub struct State {
    pub active: bool,
    pub current_turn: u8,
    pub board: Board,
    pub players: [PossiblePlayer; 2],
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = "Milestone: {\n".to_owned();

        repr.push_str(&format!("active: {:?}\n", self.active));

        let current_player = &self.players[usize::from(self.current_turn)];
        repr.push_str(&format!(
            "current_turn: {} {:?}\n",
            current_player.name(),
            current_player.get_pieces_type()
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

        let current_player: &dyn Player =
            &self.players[usize::from(self.current_turn)];
        repr.push_str(&format!(
            "  current_turn: {} {:?}\n",
            current_player.name(),
            current_player.get_pieces_type()
        ));

        repr.push_str(&format!("  board: {}", self.board));

        write!(f, "{}", repr)
    }
}

impl State {
    pub fn new(players: &[PossiblePlayer; 2]) -> State {
        State {
            active: true,
            current_turn: 0,
            board: Board::new(),
            players: players.clone(),
        }
    }

    // can_move() returns <T, _> if this players move would require a capture,
    //   <F, _> if it is a valid move without a capture,
    //   and <_, E(str)> if it is an invalid move
    pub fn can_move(
        &self,
        from: usize,
        to: usize,
    ) -> Result<bool, &'static str> {
        let current_player_pieces =
            self.players[usize::from(self.current_turn)].get_pieces_type();

        let valid_start: bool = self.board.current_players_pieces(self.current_turn).contains(&from);
        
        if valid_start {
        match self.board.possible_move(&from, &to, self.current_turn) {
            Some(m @ Move::Diagonal(a, d)) | Some(m @ Move::Straight(a, d)) => {
                match self.board.board[d].0 {
                    Some(existing_piece) if existing_piece == current_player_pieces => {
                        Err("can't occupy the same space as another one of your pieces")
                    }
                    Some(_) => match m {
                        Move::Straight(_, _) => Ok(true),
                        Move::Diagonal(_, _) => Err("must capture head-on"),
                    },
                    None => Ok(false),
                }
            }
            None => Err("not a legal move for this piece or it's not your turn"),
        }
        }
        else{
          Err("Not a legal move. No pieces at the starting space specified")
        }
    }

    pub fn move_piece(
        &mut self,
        from: usize,
        to: usize,
        capture: bool,
    ) -> Result<&State, &'static str> {
        let current_player_pieces =
            self.players[usize::from(self.current_turn)].get_pieces_type();

        match (self.can_move(from, to), capture) {
            (Ok(true), true) => {
                self.move_piece_aux(from, to, current_player_pieces);
                Ok(self)
            }
            (Ok(true), false) => {
                Err("you don't have permission to capture this piece")
            }
            (Ok(false), _) => {
                self.move_piece_aux(from, to, current_player_pieces);
                Ok(self)
            }
            (Err(e), _) => Err(e),
        }
    }

    // this function assumes that moving the piece is legal / valid
    fn move_piece_aux(
        &mut self,
        from: usize,
        to: usize,
        current_player_pieces: Piece,
    ) {
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

    pub fn current_possible_moves(&self) -> Vec<Move> {
        Board::all_valid_moves(&self.board, self.current_turn)
    }
}
