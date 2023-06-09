use super::{
    board::Board,
    board::Hole,
    board::Move,
    board::Move::Diagonal,
    board::{get_moves_of_piece, Move::Straight},
    pieces::Piece,
    player::Player,
    player::PossiblePlayer,
    player::AI,
};
use crate::DATABASE_URL;
use rusqlite::{params, Connection, Result};
use std::fmt::{self};

#[derive(Default)]
pub struct GameBuilder {
    board: Board,
    players: [PossiblePlayer; 2],
}

impl GameBuilder {
    pub fn new() -> GameBuilder {
        GameBuilder {
            board: Board::new(),
            players: [PossiblePlayer::default(), PossiblePlayer::default()],
        }
    }

    pub fn set_player_1(mut self, p: PossiblePlayer) -> GameBuilder {
        self.players[0] = p;
        self
    }

    pub fn set_player_2(mut self, p: PossiblePlayer) -> GameBuilder {
        self.players[1] = p;
        self
    }

    pub fn build(self) -> State {
        State {
            active: true,
            winner: None,
            current_turn: 0,
            board: self.board.to_owned(),
            players: self.players,
            state_history: vec![],
        }
    }
}

#[derive(Clone)]
pub struct State {
    pub active: bool,
    pub winner: Option<u8>,
    pub current_turn: u8,
    pub board: Board,
    pub players: [PossiblePlayer; 2],
    pub state_history: Vec<String>,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_struct = f.debug_struct("Milestone");

        fmt_struct.field("active", &self.active);

        let current_player = &self.players[usize::from(self.current_turn)];
        fmt_struct.field(
            "current_turn",
            &format!(
                "{} {:?}",
                current_player.name(),
                self.get_pieces_type_from_player(current_player)
            ),
        );

        fmt_struct.field("board", &self.board);
        fmt_struct.field("players", &self.players);
        fmt_struct.field("string repr", &self.to_repr_string());

        fmt_struct.finish()
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_struct = f.debug_struct("Milestone");

        let current_player = &self.players[usize::from(self.current_turn)];
        fmt_struct.field(
            "current_turn",
            &format!(
                "{} {:?}",
                current_player.name(),
                self.get_pieces_type_from_player(current_player)
            ),
        );

        fmt_struct.field("board", &self.board);
        fmt_struct.field("string repr", &self.to_repr_string());

        fmt_struct.finish()
    }
}

impl State {
    // can_move() returns <T, _> if this players move would require a capture,
    //   <F, _> if it is a valid move without a capture,
    //   and <_, E(str)> if it is an invalid move
    pub fn can_move(
        &self,
        from: &usize,
        to: &usize,
        turn: u8,
    ) -> Result<bool, &'static str> {
        let current_player_pieces = self.get_pieces_type_from_idx(turn);

        let valid_start =
            self.board.board[*from] == Hole(Some(current_player_pieces));

        if valid_start {
            match self.board.possible_move(from, to, turn) {
                Some(m @ Move::Diagonal(_a, d))
                | Some(m @ Move::Straight(_a, d)) => {
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
                None => {
                    Err("not a legal move for this piece or it's not your turn")
                }
            }
        } else {
            Err("Not a legal move. No pieces at the starting space specified")
        }
    }

    pub fn move_piece(
        &mut self,
        from: usize,
        to: usize,
        capture: bool,
    ) -> Result<(), &'static str> {
        let current_player_pieces =
            self.get_pieces_type_from_idx(self.current_turn);

        match (self.can_move(&from, &to, self.current_turn), capture) {
            (Ok(true), true) => {
                self.move_piece_aux(from, to, current_player_pieces);
                Ok(())
            }
            (Ok(true), false) => {
                Err("you don't have permission to capture this piece")
            }
            (Ok(false), _) => {
                self.move_piece_aux(from, to, current_player_pieces);
                Ok(())
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

        // if white just moved, and black now can't move, white wins
        // if black just moved, and white now can't move, black wins
        if self
            .board
            .current_players_pieces(1 - self.current_turn)
            .is_empty()
            || !self.has_a_possible_move(1 - self.current_turn)
        {
            self.active = false;
            self.winner = Some(self.current_turn);
        }

        // check if game ends (via landing in home square)
        if to == 0 || to == 36 {
            self.active = false;
            self.winner = Some(self.current_turn);
        } else {
            // one move per turn
            self.current_turn = 1 - self.current_turn;
        }
    }

    pub fn current_possible_moves(&self, turn: u8) -> Vec<Move> {
        if !self.active {
            return vec![];
        }
        Board::all_valid_moves(&self.board, turn)
            .into_iter()
            .filter(|&m| {
                let (Diagonal(origin, dest) | Straight(origin, dest)) = m;
                matches!(self.can_move(&origin, &dest, turn), Ok(_))
            })
            .collect::<Vec<Move>>()
    }

    pub fn get_pieces_type_from_player(&self, p: &PossiblePlayer) -> Piece {
        // operating on the assumption that players are placed in order (black first)
        let idx = self
            .players
            .iter()
            .position(|plr| plr == p)
            .expect("this player is not in the game");
        self.get_pieces_type_from_idx(idx.try_into().unwrap())
    }

    pub fn get_pieces_type_from_idx(&self, idx: u8) -> Piece {
        match idx {
            0 => Piece::Black,
            1 => Piece::White,
            _ => panic!("games only have two players"),
        }
    }

    pub fn play_one_turn(&mut self) {
        let current_player =
            self.players[self.current_turn as usize].to_owned();

        current_player.one_turn(self);
    }

    fn has_a_possible_move(&mut self, turn: u8) -> bool {
        for origin in self.board.current_players_pieces(turn).iter() {
            for mv in get_moves_of_piece(turn, origin).iter() {
                let (Straight(origin, dest) | Diagonal(origin, dest)) = mv;
                if self.can_move(origin, dest, turn).is_ok() {
                    return true;
                }
            }
        }

        false
    }

    pub fn add_to_state_history(&mut self) {
        self.state_history.push(self.to_repr_string())
    }

    #[allow(clippy::result_unit_err)]
    pub fn from_repr_string(s: &str) -> Result<State, ()> {
        match s.get(0..=1) {
            Some("b:") => {
                let b = match s.get(2..) {
                    Some(s) => Board::from_repr_string(s),
                    None => return Err(()),
                };
                Ok(State {
                    active: true,
                    winner: None,
                    current_turn: 0,
                    players: [
                        PossiblePlayer::AI(AI::default()),
                        PossiblePlayer::AI(AI::default()),
                    ],
                    board: b?,
                    state_history: vec![s.to_string()],
                })
            }
            Some("w:") => {
                let b = match s.get(2..) {
                    Some(s) => Board::from_repr_string(s),
                    None => return Err(()),
                };
                Ok(State {
                    active: true,
                    winner: None,
                    current_turn: 1,
                    players: [
                        PossiblePlayer::AI(AI::default()),
                        PossiblePlayer::AI(AI::default()),
                    ],
                    board: b?,
                    state_history: vec![s.to_string()],
                })
            }
            _ => Err(()),
        }
    }

    pub fn to_repr_string(&self) -> String {
        if !self.active {
            return "".to_string();
        }
        match self.current_turn {
            0 => "b:".to_owned() + &self.board.to_repr_string(),
            1 => "w:".to_owned() + &self.board.to_repr_string(),
            _ => panic!("Impossible current turn"),
        }
    }

    pub fn push_game_and_state(&self) -> Result<()> {
        let mut conn = Connection::open(&*DATABASE_URL).unwrap();
        let game_id = self.push_game(&mut conn).unwrap();
        self.push_game_state_history(&mut conn, game_id).unwrap();
        Ok(())
    }

    pub fn push_game(&self, conn: &mut Connection) -> Result<i64> {
        let game_winner = self.winner.unwrap();
        conn.execute(
            r#"
            INSERT INTO game_table (result)
            VALUES (?)
            "#,
            [game_winner],
        )?;
        let last_id = conn.last_insert_rowid();
        Ok(last_id)
    }

    pub fn push_game_state_history(
        &self,
        conn: &mut Connection,
        game_id: i64,
    ) -> Result<()> {
        let tx = conn.transaction()?;
        for (i, state) in self.state_history.clone().into_iter().enumerate() {
            tx.execute(
                r#"
                INSERT INTO state_table (state, game_id, move_number)
                VALUES (?1, ?2, ?3)
                "#,
                params![state, game_id, i],
            )?;
        }
        tx.commit()
    }
}
