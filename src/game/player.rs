use super::pieces::Piece;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub pieces: Piece,
}

impl Player {
    pub fn new(name: String, pieces: Piece) -> Player {
        Player { name, pieces }
    }

    pub fn new_players(name_one: String, name_two: String) -> [Player; 2] {
        [
            Player::new(name_one, Piece::Black),
            Player::new(name_two, Piece::White),
        ]
    }
}
