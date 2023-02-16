use super::pieces::Piece;
use std::collections::HashMap;
use std::fmt;

pub struct Board {
    pub board: [Hole; 37],
    black_move_lookup: HashMap<usize, Vec<usize>>,
    white_move_lookup: HashMap<usize, Vec<usize>>,
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

fn get_black_moves() -> HashMap<usize, Vec<usize>> {
    let mut hm: HashMap<usize, Vec<usize>> = HashMap::new();

    hm.insert(0, vec![1, 2, 4]);

    hm.insert(1, vec![1, 3, 7]);
    hm.insert(2, vec![4, 5, 8]);

    hm.insert(3, vec![6, 7, 10]);
    hm.insert(4, vec![7, 8, 11]);
    hm.insert(5, vec![8, 9, 12]);

    hm.insert(6, vec![10, 13]);
    hm.insert(7, vec![10, 11, 14]);
    hm.insert(8, vec![11, 12, 15]);
    hm.insert(9, vec![12, 16]);

    hm.insert(10, vec![13, 14, 17]);
    hm.insert(11, vec![14, 15, 18]);
    hm.insert(12, vec![15, 16, 19]);

    hm.insert(13, vec![17, 20]);
    hm.insert(14, vec![17, 18, 21]);
    hm.insert(15, vec![18, 19, 22]);
    hm.insert(16, vec![23, 26]);

    hm.insert(17, vec![20, 21, 24]);
    hm.insert(18, vec![21, 22, 25]);
    hm.insert(19, vec![22, 23, 26]);

    hm.insert(20, vec![24, 27]);
    hm.insert(21, vec![24, 25, 28]);
    hm.insert(22, vec![25, 26, 29]);
    hm.insert(23, vec![26, 30]);

    hm.insert(24, vec![27, 28, 31]);
    hm.insert(25, vec![28, 29, 32]);
    hm.insert(26, vec![29, 30, 33]);

    hm.insert(27, vec![31]);
    hm.insert(28, vec![31, 32, 34]);
    hm.insert(29, vec![32, 33, 25]);
    hm.insert(30, vec![33]);

    hm.insert(31, vec![34]);
    hm.insert(32, vec![34, 35, 36]);
    hm.insert(33, vec![35]);

    hm.insert(34, vec![36]);
    hm.insert(35, vec![36]);

    hm
}

fn get_white_moves() -> HashMap<usize, Vec<usize>> {
    let mut hm: HashMap<usize, Vec<usize>> = HashMap::new();

    hm.insert(36, vec![35, 34, 32]);

    hm.insert(35, vec![33, 32, 29]);
    hm.insert(34, vec![32, 31, 28]);

    hm.insert(33, vec![30, 29, 26]);
    hm.insert(32, vec![29, 28, 25]);
    hm.insert(31, vec![28, 27, 24]);

    hm.insert(30, vec![26, 23]);
    hm.insert(29, vec![26, 25, 22]);
    hm.insert(28, vec![25, 24, 21]);
    hm.insert(27, vec![24, 20]);

    hm.insert(26, vec![23, 22, 19]);
    hm.insert(25, vec![22, 21, 18]);
    hm.insert(24, vec![21, 20, 17]);

    hm.insert(23, vec![19, 16]);
    hm.insert(22, vec![19, 18, 15]);
    hm.insert(21, vec![18, 17, 14]);
    hm.insert(20, vec![17, 13]);

    hm.insert(19, vec![16, 15, 12]);
    hm.insert(18, vec![15, 14, 11]);
    hm.insert(17, vec![14, 13, 10]);

    hm.insert(16, vec![12, 9]);
    hm.insert(15, vec![12, 11, 8]);
    hm.insert(14, vec![11, 10, 7]);
    hm.insert(13, vec![10, 6]);

    hm.insert(12, vec![9, 8, 5]);
    hm.insert(11, vec![8, 7, 4]);
    hm.insert(10, vec![7, 5, 3]);

    hm.insert(9, vec![5]);
    hm.insert(8, vec![5, 4, 2]);
    hm.insert(7, vec![4, 3, 1]);
    hm.insert(6, vec![3]);

    hm.insert(5, vec![2]);
    hm.insert(4, vec![2, 1, 0]);
    hm.insert(3, vec![1]);

    hm.insert(2, vec![0]);
    hm.insert(1, vec![0]);

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

    pub fn can_move(&self, from: &usize, to: &usize, turn: u8) -> bool {
        let lookup = match turn {
            0 => &self.black_move_lookup,
            1 => &self.white_move_lookup,
            _ => panic!("it's impossible for more than two players to move"),
        };

        match lookup.get(from) {
            Some(possible_moves) => possible_moves.contains(to),
            None => false,
        }
    }
}
