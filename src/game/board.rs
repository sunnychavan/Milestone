use super::pieces::Piece;
use lazy_static::lazy_static;

use std::fmt;

#[derive(Clone, Copy)]
pub struct Board {
    pub board: [Hole; 37],
}

impl fmt::Display for Board {
    #[allow(clippy::repeat_once)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let space = "  ";

        let mut repr = "{\n".to_owned();

        // first line
        repr.push_str(&space.repeat(3));
        repr.push_str(&format!("{:?}", self.board[0]));
        repr.push_str(&space.repeat(3));
        repr.push('\n');

        // second line
        repr.push_str(&space.repeat(2));
        repr.push_str(&format!("{:?}", self.board[1]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[2]));
        repr.push_str(&space.repeat(2));
        repr.push('\n');

        // third line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[3]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[4]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[5]));
        repr.push_str(&space.repeat(1));
        repr.push('\n');

        // fourth line
        repr.push_str(&format!("{:?}", self.board[6]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[7]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[8]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[9]));
        repr.push('\n');

        // fifth line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[10]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[11]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[12]));
        repr.push_str(&space.repeat(1));
        repr.push('\n');

        // sixth line
        repr.push_str(&format!("{:?}", self.board[13]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[14]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[15]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[16]));
        repr.push('\n');

        // seventh line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[17]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[18]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[19]));
        repr.push_str(&space.repeat(1));
        repr.push('\n');

        // eigth line
        repr.push_str(&format!("{:?}", self.board[20]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[21]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[22]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[23]));
        repr.push('\n');

        // ninth line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[24]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[25]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[26]));
        repr.push_str(&space.repeat(1));
        repr.push('\n');

        // tenth line
        repr.push_str(&format!("{:?}", self.board[27]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[28]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[29]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[30]));
        repr.push('\n');

        // eleventh line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[31]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[32]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[33]));
        repr.push_str(&space.repeat(1));
        repr.push('\n');

        // twelfth line
        repr.push_str(&space.repeat(2));
        repr.push_str(&format!("{:?}", self.board[34]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[35]));
        repr.push_str(&space.repeat(2));
        repr.push('\n');

        // thirteenth line
        repr.push_str(&space.repeat(3));
        repr.push_str(&format!("{:?}", self.board[36]));
        repr.push_str(&space.repeat(3));
        repr.push('\n');

        repr.push('}');

        write!(f, "{repr}")
    }
}

impl fmt::Debug for Board {
    #[allow(clippy::repeat_once)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let space = "    ";

        let mut repr = "{\n".to_owned();

        // first line
        repr.push_str(&space.repeat(3));
        repr.push_str(&format!(" 0:{:?}", self.board[0]));
        repr.push_str(&space.repeat(3));
        repr.push('\n');

        // second line
        repr.push_str(&space.repeat(2));
        repr.push_str(&format!(" 1:{:?}", self.board[1]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 2:{:?}", self.board[2]));
        repr.push_str(&space.repeat(2));
        repr.push('\n');

        // third line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 3:{:?}", self.board[3]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 4:{:?}", self.board[4]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 5:{:?}", self.board[5]));
        repr.push_str(&space.repeat(1));
        repr.push('\n');

        // fourth line
        repr.push_str(&format!(" 6:{:?}", self.board[6]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 7:{:?}", self.board[7]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 8:{:?}", self.board[8]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 9:{:?}", self.board[9]));
        repr.push('\n');

        // fifth line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("10:{:?}", self.board[10]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("11:{:?}", self.board[11]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("12:{:?}", self.board[12]));
        repr.push_str(&space.repeat(1));
        repr.push('\n');

        // sixth line
        repr.push_str(&format!("13:{:?}", self.board[13]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("14:{:?}", self.board[14]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("15:{:?}", self.board[15]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("16:{:?}", self.board[16]));
        repr.push('\n');

        // seventh line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("17:{:?}", self.board[17]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("18:{:?}", self.board[18]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("19:{:?}", self.board[19]));
        repr.push_str(&space.repeat(1));
        repr.push('\n');

        // eigth line
        repr.push_str(&format!("20:{:?}", self.board[20]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("21:{:?}", self.board[21]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("22:{:?}", self.board[22]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("23:{:?}", self.board[23]));
        repr.push('\n');

        // ninth line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("24:{:?}", self.board[24]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("25:{:?}", self.board[25]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("26:{:?}", self.board[26]));
        repr.push_str(&space.repeat(1));
        repr.push('\n');

        // tenth line
        repr.push_str(&format!("27:{:?}", self.board[27]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("28:{:?}", self.board[28]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("29:{:?}", self.board[29]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("30:{:?}", self.board[30]));
        repr.push('\n');

        // eleventh line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("31:{:?}", self.board[31]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("32:{:?}", self.board[32]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("33:{:?}", self.board[33]));
        repr.push_str(&space.repeat(1));
        repr.push('\n');

        // twelfth line
        repr.push_str(&space.repeat(2));
        repr.push_str(&format!("34:{:?}", self.board[34]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("35:{:?}", self.board[35]));
        repr.push_str(&space.repeat(2));
        repr.push('\n');

        // thirteenth line
        repr.push_str(&space.repeat(3));
        repr.push_str(&format!("36:{:?}", self.board[36]));
        repr.push_str(&space.repeat(3));
        repr.push('\n');

        repr.push('}');

        write!(f, "{repr}")
    }
}

#[derive(Clone, Copy)]
pub struct Hole(pub Option<Piece>);

impl fmt::Debug for Hole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(piece) => Piece::fmt(&piece, f),
            None => write!(f, "〇"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Move {
    // of the form: (origin, dest)
    Straight(usize, usize),
    Diagonal(usize, usize),
}
use Move::{Diagonal, Straight};

lazy_static! {
    static ref BLACK_MOVES: [Vec<Move>; 37] = [
        vec![Diagonal(0, 1), Diagonal(0, 2), Straight(0, 4)],
        vec![Diagonal(1, 4), Diagonal(1, 3), Straight(1, 7)],
        vec![Diagonal(2, 4), Diagonal(2, 5), Straight(2, 8)],
        vec![Diagonal(3, 6), Diagonal(3, 7), Straight(3, 10)],
        vec![Diagonal(4, 7), Diagonal(4, 8), Straight(4, 11)],
        vec![Diagonal(5, 8), Diagonal(5, 9), Straight(5, 12)],
        vec![Diagonal(6, 10), Straight(6, 13)],
        vec![Diagonal(7, 10), Diagonal(8, 11), Straight(9, 14)],
        vec![Diagonal(8, 11), Diagonal(8, 12), Straight(8, 15)],
        vec![Diagonal(9, 12), Straight(9, 16)],
        vec![Diagonal(10, 13), Diagonal(10, 14), Straight(10, 17)],
        vec![Diagonal(11, 14), Diagonal(11, 15), Straight(11, 18)],
        vec![Diagonal(12, 15), Diagonal(12, 16), Straight(12, 19)],
        vec![Diagonal(13, 17), Straight(13, 20)],
        vec![Diagonal(14, 17), Diagonal(14, 18), Straight(14, 21)],
        vec![Diagonal(15, 18), Diagonal(15, 19), Straight(15, 22)],
        vec![Diagonal(16, 19), Straight(16, 23)],
        vec![Diagonal(17, 20), Diagonal(17, 21), Straight(17, 24)],
        vec![Diagonal(18, 21), Diagonal(18, 22), Straight(18, 25)],
        vec![Diagonal(19, 22), Diagonal(19, 23), Straight(19, 26)],
        vec![Diagonal(20, 24), Straight(20, 27)],
        vec![Diagonal(21, 24), Diagonal(21, 25), Straight(21, 28)],
        vec![Diagonal(22, 25), Diagonal(22, 26), Straight(22, 29)],
        vec![Diagonal(23, 26), Straight(23, 30)],
        vec![Diagonal(24, 27), Diagonal(24, 28), Straight(24, 31)],
        vec![Diagonal(25, 28), Diagonal(25, 29), Straight(25, 32)],
        vec![Diagonal(26, 29), Diagonal(26, 30), Straight(26, 33)],
        vec![Diagonal(27, 31)],
        vec![Diagonal(28, 31), Diagonal(28, 32), Straight(28, 34)],
        vec![Diagonal(29, 32), Diagonal(29, 33), Straight(29, 35)],
        vec![Diagonal(30, 33)],
        vec![Diagonal(31, 34)],
        vec![Diagonal(32, 34), Diagonal(32, 35), Straight(32, 36)],
        vec![Diagonal(33, 35)],
        vec![Diagonal(34, 36)],
        vec![Diagonal(35, 36)],
        vec![],
    ];
}

lazy_static! {
    static ref WHITE_MOVES: [Vec<Move>; 37] = [
        vec![],
        vec![Diagonal(1, 0)],
        vec![Diagonal(2, 0)],
        vec![Diagonal(3, 1)],
        vec![Diagonal(4, 2), Diagonal(4, 1), Straight(4, 0)],
        vec![Diagonal(5, 2)],
        vec![Diagonal(6, 3)],
        vec![Diagonal(7, 4), Diagonal(7, 3), Straight(7, 1)],
        vec![Diagonal(8, 5), Diagonal(8, 4), Straight(8, 2)],
        vec![Diagonal(9, 5)],
        vec![Diagonal(10, 7), Diagonal(10, 6), Straight(10, 3)],
        vec![Diagonal(11, 8), Diagonal(11, 7), Straight(11, 4)],
        vec![Diagonal(12, 9), Diagonal(12, 8), Straight(12, 5)],
        vec![Diagonal(13, 10), Straight(13, 6)],
        vec![Diagonal(14, 11), Diagonal(14, 10), Straight(14, 7)],
        vec![Diagonal(15, 12), Diagonal(15, 11), Straight(15, 8)],
        vec![Diagonal(16, 12), Straight(16, 9)],
        vec![Diagonal(17, 14), Diagonal(17, 13), Straight(17, 10)],
        vec![Diagonal(18, 15), Diagonal(18, 14), Straight(18, 11)],
        vec![Diagonal(19, 16), Diagonal(19, 15), Straight(19, 12)],
        vec![Diagonal(20, 17), Straight(20, 13)],
        vec![Diagonal(21, 18), Diagonal(21, 17), Straight(21, 14)],
        vec![Diagonal(22, 19), Diagonal(22, 18), Straight(22, 15)],
        vec![Diagonal(23, 19), Straight(23, 16)],
        vec![Diagonal(24, 21), Diagonal(24, 20), Straight(24, 17)],
        vec![Diagonal(25, 22), Diagonal(25, 21), Straight(25, 18)],
        vec![Diagonal(26, 23), Diagonal(26, 22), Straight(26, 19)],
        vec![Diagonal(27, 24), Straight(27, 20)],
        vec![Diagonal(28, 25), Diagonal(28, 24), Straight(28, 21)],
        vec![Diagonal(29, 26), Diagonal(29, 25), Straight(29, 22)],
        vec![Diagonal(30, 26), Straight(30, 23)],
        vec![Diagonal(31, 28), Diagonal(31, 27), Straight(31, 24)],
        vec![Diagonal(32, 29), Diagonal(32, 28), Straight(32, 25)],
        vec![Diagonal(33, 30), Diagonal(33, 29), Straight(33, 26)],
        vec![Diagonal(34, 32), Diagonal(34, 31), Straight(34, 28)],
        vec![Diagonal(35, 33), Diagonal(35, 32), Straight(35, 29)],
        vec![Diagonal(36, 35), Diagonal(36, 34), Straight(36, 32)],
    ];
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl Board {
    pub fn new() -> Board {
        let mut pieces = [Hole(None); 37];
        #[allow(clippy::needless_range_loop)]
        for i in 0..=36 {
            pieces[i] = match i {
                i if i < 10 => Hole(Some(Piece::Black)),
                i if i > 26 => Hole(Some(Piece::White)),
                _ => Hole(None),
            }
        }
        Board { board: pieces }
    }

    pub fn possible_move(
        &self,
        from: &usize,
        to: &usize,
        turn: u8,
    ) -> Option<Move> {
        let lookup = match turn {
            // need to use * to get the true type (not that created by lazy_static)
            // and then we use a reference to it
            0 => &*BLACK_MOVES,
            1 => &*WHITE_MOVES,
            _ => panic!("it's impossible for more than two players to move"),
        };

        match lookup.get(from.to_owned()) {
            Some(possible_moves) => possible_moves
                .iter()
                .find(|elt| match elt {
                    Straight(_, m) | Diagonal(_, m) => m == to,
                })
                .copied(),
            None => None,
        }
    }

    pub fn current_players_pieces(&self, turn: u8) -> Vec<usize> {
        let piece_type = match turn {
            0 => Piece::Black,
            1 => Piece::White,
            _ => panic!("it's impossible for more than two players to move"),
        };

        // current_pieces is a vector of all pieces for the given player
        self.board
            .into_iter()
            .enumerate()
            .filter_map(|(idx, elt)| match elt {
                Hole(Some(p)) => {
                    if p == piece_type {
                        Some(idx)
                    } else {
                        None
                    }
                }
                Hole(None) => None,
            })
            .collect::<Vec<usize>>()
    }

    pub fn all_valid_moves(&self, turn: u8) -> Vec<Move> {
        let lookup = match turn {
            // need to use * to get the true type (not that created by lazy_static)
            // and then we use a reference to it
            0 => &*BLACK_MOVES,
            1 => &*WHITE_MOVES,
            _ => panic!("it's impossible for more than two players to move"),
        };

        // current_pieces is a vector of all pieces for the given player
        let current_pieces = &self.current_players_pieces(turn);

        current_pieces
            .iter()
            .flat_map(|elt| {
                let move_dests = lookup.get(elt.to_owned());
                match move_dests {
                    Some(moves) => moves.to_vec(),
                    None => vec![],
                }
            })
            .collect::<Vec<Move>>()
    }

    pub fn get_moves_of_piece(&self, turn: u8) -> &[Vec<Move>] {
        match turn {
            0 => &*BLACK_MOVES,
            1 => &*WHITE_MOVES,
            _ => panic!("Invalid Turn (must be zero or one"),
        }
    }

    pub fn get_straight_hex(&self, turn: u8, index: usize) -> Option<&usize>{
        let piece_moves = self.get_moves_of_piece(turn);

        // in the move hashmaps up above, the straight move is always last 
        let potential_straight_move = piece_moves.get(index).unwrap().last() ;
        
        match potential_straight_move {
            Some(Straight(_, dest)) => Some(dest),
            _ => None,
        }
    }
}
