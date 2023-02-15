use super::pieces::Piece;

#[derive(Debug)]
pub struct Player {
    name: String,
    pieces: Piece,
}

impl Player {
    pub fn new(name: String, pieces: Piece) -> Player {
        Player { name, pieces }
    }

    pub fn new_players(name_one: String, name_two: String) -> (Player, Player) {
        (
            Player::new(name_one, Piece::Black),
            Player::new(name_two, Piece::White),
        )
    }
}
