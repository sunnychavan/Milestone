use super::pieces::Piece;
use std::collections::HashMap;
use std::fmt;

pub struct Board {
    pub board: [Hole; 37],
    black_move_lookup: HashMap<usize, Vec<Move>>,
    white_move_lookup: HashMap<usize, Vec<Move>>,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let space = "  ";

        let mut repr = "{\n".to_owned();

        // first line
        repr.push_str(&space.repeat(3));
        repr.push_str(&format!("{:?}", self.board[0]));
        repr.push_str(&space.repeat(3));
        repr.push_str(&"\n");

        // second line
        repr.push_str(&space.repeat(2));
        repr.push_str(&format!("{:?}", self.board[1]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[2]));
        repr.push_str(&space.repeat(2));
        repr.push_str(&"\n");

        // third line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[3]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[4]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[5]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // fourth line
        repr.push_str(&format!("{:?}", self.board[6]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[7]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[8]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[9]));
        repr.push_str(&"\n");

        // fifth line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[10]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[11]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[12]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // sixth line
        repr.push_str(&format!("{:?}", self.board[13]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[14]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[15]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[16]));
        repr.push_str(&"\n");

        // seventh line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[17]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[18]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[19]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // eigth line
        repr.push_str(&format!("{:?}", self.board[20]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[21]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[22]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[23]));
        repr.push_str(&"\n");

        // ninth line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[24]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[25]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[26]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // tenth line
        repr.push_str(&format!("{:?}", self.board[27]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[28]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[29]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[30]));
        repr.push_str(&"\n");

        // eleventh line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[31]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[32]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[33]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // twelfth line
        repr.push_str(&space.repeat(2));
        repr.push_str(&format!("{:?}", self.board[34]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.board[35]));
        repr.push_str(&space.repeat(2));
        repr.push_str(&"\n");

        // thirteenth line
        repr.push_str(&space.repeat(3));
        repr.push_str(&format!("{:?}", self.board[36]));
        repr.push_str(&space.repeat(3));
        repr.push_str(&"\n");

        repr.push_str("}");

        write!(f, "{}", repr)
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let space = "    ";

        let mut repr = "{\n".to_owned();

        // first line
        repr.push_str(&space.repeat(3));
        repr.push_str(&format!(" 0:{:?}", self.board[0]));
        repr.push_str(&space.repeat(3));
        repr.push_str(&"\n");

        // second line
        repr.push_str(&space.repeat(2));
        repr.push_str(&format!(" 1:{:?}", self.board[1]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 2:{:?}", self.board[2]));
        repr.push_str(&space.repeat(2));
        repr.push_str(&"\n");

        // third line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 3:{:?}", self.board[3]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 4:{:?}", self.board[4]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 5:{:?}", self.board[5]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // fourth line
        repr.push_str(&format!(" 6:{:?}", self.board[6]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 7:{:?}", self.board[7]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 8:{:?}", self.board[8]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 9:{:?}", self.board[9]));
        repr.push_str(&"\n");

        // fifth line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("10:{:?}", self.board[10]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("11:{:?}", self.board[11]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("12:{:?}", self.board[12]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // sixth line
        repr.push_str(&format!("13:{:?}", self.board[13]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("14:{:?}", self.board[14]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("15:{:?}", self.board[15]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("16:{:?}", self.board[16]));
        repr.push_str(&"\n");

        // seventh line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("17:{:?}", self.board[17]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("18:{:?}", self.board[18]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("19:{:?}", self.board[19]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // eigth line
        repr.push_str(&format!("20:{:?}", self.board[20]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("21:{:?}", self.board[21]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("22:{:?}", self.board[22]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("23:{:?}", self.board[23]));
        repr.push_str(&"\n");

        // ninth line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("24:{:?}", self.board[24]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("25:{:?}", self.board[25]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("26:{:?}", self.board[26]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // tenth line
        repr.push_str(&format!("27:{:?}", self.board[27]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("28:{:?}", self.board[28]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("29:{:?}", self.board[29]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("30:{:?}", self.board[30]));
        repr.push_str(&"\n");

        // eleventh line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("31:{:?}", self.board[31]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("32:{:?}", self.board[32]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("33:{:?}", self.board[33]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // twelfth line
        repr.push_str(&space.repeat(2));
        repr.push_str(&format!("34:{:?}", self.board[34]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("35:{:?}", self.board[35]));
        repr.push_str(&space.repeat(2));
        repr.push_str(&"\n");

        // thirteenth line
        repr.push_str(&space.repeat(3));
        repr.push_str(&format!("36:{:?}", self.board[36]));
        repr.push_str(&space.repeat(3));
        repr.push_str(&"\n");

        repr.push_str("}");

        write!(f, "{}", repr)
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

fn get_black_moves() -> HashMap<usize, Vec<Move>> {
    let mut hm: HashMap<usize, Vec<Move>> = HashMap::new();

    hm.insert(0, vec![Diagonal(0, 1), Diagonal(0, 2), Straight(0, 4)]);

    hm.insert(1, vec![Diagonal(1, 1), Diagonal(1, 3), Straight(1, 7)]);
    hm.insert(2, vec![Diagonal(2, 4), Diagonal(2, 5), Straight(2, 8)]);

    hm.insert(3, vec![Diagonal(3, 6), Diagonal(3, 7), Straight(3, 10)]);
    hm.insert(4, vec![Diagonal(4, 7), Diagonal(4, 8), Straight(4, 11)]);
    hm.insert(5, vec![Diagonal(5, 8), Diagonal(5, 9), Straight(5, 12)]);

    hm.insert(6, vec![Diagonal(6, 10), Straight(6, 13)]);
    hm.insert(7, vec![Diagonal(7, 10), Diagonal(8, 11), Straight(9, 14)]);
    hm.insert(8, vec![Diagonal(8, 11), Diagonal(8, 12), Straight(8, 15)]);
    hm.insert(9, vec![Diagonal(9, 12), Straight(9, 16)]);

    hm.insert(
        10,
        vec![Diagonal(10, 13), Diagonal(10, 14), Straight(10, 17)],
    );
    hm.insert(
        11,
        vec![Diagonal(11, 14), Diagonal(11, 15), Straight(11, 18)],
    );
    hm.insert(
        12,
        vec![Diagonal(12, 15), Diagonal(12, 16), Straight(12, 19)],
    );

    hm.insert(13, vec![Diagonal(13, 17), Straight(13, 20)]);
    hm.insert(
        14,
        vec![Diagonal(14, 17), Diagonal(14, 18), Straight(14, 21)],
    );
    hm.insert(
        15,
        vec![Diagonal(15, 18), Diagonal(15, 19), Straight(15, 22)],
    );
    hm.insert(16, vec![Diagonal(16, 23), Straight(16, 26)]);

    hm.insert(
        17,
        vec![Diagonal(17, 20), Diagonal(17, 21), Straight(17, 24)],
    );
    hm.insert(
        18,
        vec![Diagonal(18, 21), Diagonal(18, 22), Straight(18, 25)],
    );
    hm.insert(
        19,
        vec![Diagonal(19, 22), Diagonal(19, 23), Straight(19, 26)],
    );

    hm.insert(20, vec![Diagonal(20, 24), Straight(20, 27)]);
    hm.insert(
        21,
        vec![Diagonal(21, 24), Diagonal(21, 25), Straight(21, 28)],
    );
    hm.insert(
        22,
        vec![Diagonal(22, 25), Diagonal(22, 26), Straight(22, 29)],
    );
    hm.insert(23, vec![Diagonal(23, 26), Straight(23, 30)]);

    hm.insert(
        24,
        vec![Diagonal(24, 27), Diagonal(24, 28), Straight(24, 31)],
    );
    hm.insert(
        25,
        vec![Diagonal(25, 28), Diagonal(25, 29), Straight(25, 32)],
    );
    hm.insert(
        26,
        vec![Diagonal(26, 29), Diagonal(26, 30), Straight(26, 33)],
    );

    hm.insert(27, vec![Diagonal(27, 31)]);
    hm.insert(
        28,
        vec![Diagonal(28, 31), Diagonal(28, 32), Straight(28, 34)],
    );
    hm.insert(
        29,
        vec![Diagonal(29, 32), Diagonal(29, 33), Straight(29, 25)],
    );
    hm.insert(30, vec![Diagonal(30, 33)]);

    hm.insert(31, vec![Diagonal(31, 34)]);
    hm.insert(
        32,
        vec![Diagonal(32, 34), Diagonal(32, 35), Straight(32, 36)],
    );
    hm.insert(33, vec![Diagonal(33, 35)]);

    hm.insert(34, vec![Diagonal(34, 36)]);
    hm.insert(35, vec![Diagonal(35, 36)]);

    hm
}

fn get_white_moves() -> HashMap<usize, Vec<Move>> {
    let mut hm: HashMap<usize, Vec<Move>> = HashMap::new();

    hm.insert(
        36,
        vec![Diagonal(36, 35), Diagonal(36, 34), Straight(36, 32)],
    );

    hm.insert(
        35,
        vec![Diagonal(35, 33), Diagonal(35, 32), Straight(35, 29)],
    );
    hm.insert(
        34,
        vec![Diagonal(34, 32), Diagonal(34, 31), Straight(34, 28)],
    );

    hm.insert(
        33,
        vec![Diagonal(33, 30), Diagonal(33, 29), Straight(33, 26)],
    );
    hm.insert(
        32,
        vec![Diagonal(32, 29), Diagonal(32, 28), Straight(32, 25)],
    );
    hm.insert(
        31,
        vec![Diagonal(31, 28), Diagonal(31, 27), Straight(31, 24)],
    );

    hm.insert(30, vec![Diagonal(30, 26), Straight(30, 23)]);
    hm.insert(
        29,
        vec![Diagonal(29, 26), Diagonal(29, 25), Straight(29, 22)],
    );
    hm.insert(
        28,
        vec![Diagonal(28, 25), Diagonal(28, 24), Straight(28, 21)],
    );
    hm.insert(27, vec![Diagonal(27, 24), Straight(27, 20)]);

    hm.insert(
        26,
        vec![Diagonal(26, 23), Diagonal(26, 22), Straight(26, 19)],
    );
    hm.insert(
        25,
        vec![Diagonal(25, 22), Diagonal(25, 21), Straight(25, 18)],
    );
    hm.insert(
        24,
        vec![Diagonal(24, 21), Diagonal(24, 20), Straight(24, 17)],
    );

    hm.insert(23, vec![Diagonal(23, 19), Straight(23, 16)]);
    hm.insert(
        22,
        vec![Diagonal(22, 19), Diagonal(22, 18), Straight(22, 15)],
    );
    hm.insert(
        21,
        vec![Diagonal(21, 18), Diagonal(21, 17), Straight(21, 14)],
    );
    hm.insert(20, vec![Diagonal(20, 17), Straight(20, 13)]);

    hm.insert(
        19,
        vec![Diagonal(19, 16), Diagonal(19, 15), Straight(19, 12)],
    );
    hm.insert(
        18,
        vec![Diagonal(18, 15), Diagonal(18, 14), Straight(18, 11)],
    );
    hm.insert(
        17,
        vec![Diagonal(17, 14), Diagonal(17, 13), Straight(17, 10)],
    );

    hm.insert(16, vec![Diagonal(16, 12), Straight(16, 9)]);
    hm.insert(
        15,
        vec![Diagonal(15, 12), Diagonal(15, 11), Straight(15, 8)],
    );
    hm.insert(
        14,
        vec![Diagonal(14, 11), Diagonal(14, 10), Straight(14, 7)],
    );
    hm.insert(13, vec![Diagonal(13, 10), Straight(13, 6)]);

    hm.insert(12, vec![Diagonal(12, 9), Diagonal(12, 8), Straight(12, 5)]);
    hm.insert(11, vec![Diagonal(11, 8), Diagonal(11, 7), Straight(11, 4)]);
    hm.insert(10, vec![Diagonal(10, 7), Diagonal(10, 5), Straight(10, 3)]);

    hm.insert(9, vec![Diagonal(9, 5)]);
    hm.insert(8, vec![Diagonal(8, 5), Diagonal(8, 4), Straight(8, 2)]);
    hm.insert(7, vec![Diagonal(7, 4), Diagonal(7, 3), Straight(7, 1)]);
    hm.insert(6, vec![Diagonal(6, 3)]);

    hm.insert(5, vec![Diagonal(5, 2)]);
    hm.insert(4, vec![Diagonal(4, 2), Diagonal(4, 1), Straight(4, 0)]);
    hm.insert(3, vec![Diagonal(3, 1)]);

    hm.insert(2, vec![Diagonal(2, 0)]);
    hm.insert(1, vec![Diagonal(1, 0)]);

    hm
}

impl Board {
    pub fn new() -> Board {
        let mut pieces = [Hole(None); 37];
        for i in 0..=36 {
            pieces[i] = match i {
                i if i < 10 => Hole(Some(Piece::Black)),
                i if i > 26 => Hole(Some(Piece::White)),
                _ => Hole(None),
            }
        }
        Board {
            board: pieces,
            black_move_lookup: get_black_moves(),
            white_move_lookup: get_white_moves(),
        }
    }

    pub fn possible_move(&self, from: &usize, to: &usize, turn: u8) -> Option<Move> {
        let lookup = match turn {
            0 => &self.black_move_lookup,
            1 => &self.white_move_lookup,
            _ => panic!("it's impossible for more than two players to move"),
        };

        match lookup.get(from).cloned() {
            Some(mut possible_moves) => {
                possible_moves.retain(|elt| match elt {
                    Straight(_, m) | Diagonal(_, m) => m == to,
                });
                possible_moves.pop()
            }
            None => None,
        }
    }

    pub fn all_valid_moves(&self, turn: u8) -> Vec<Move> {
        let (piece_type, lookup) = match turn {
            0 => (Piece::Black, &self.black_move_lookup),
            1 => (Piece::White, &self.white_move_lookup),
            _ => panic!("it's impossible for more than two players to move"),
        };

        // current_pieces is a vector of all pieces for the given player
        let current_pieces = &self
            .board
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
            .collect::<Vec<usize>>();

        current_pieces
            .iter()
            .flat_map(|elt| {
                let move_dests = lookup.get(elt).cloned();
                match move_dests {
                    Some(moves) => moves.into_iter().collect::<Vec<Move>>(),
                    None => vec![],
                }
            })
            .collect::<Vec<Move>>()
    }
}
