use super::pieces::Piece;
use std::fmt;

pub struct Board(pub [Hole; 37]);

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let space = "  ";

        let mut repr = "{\n".to_owned();

        // first line
        repr.push_str(&space.repeat(3));
        repr.push_str(&format!("{:?}", self.0[0]));
        repr.push_str(&space.repeat(3));
        repr.push_str(&"\n");

        // second line
        repr.push_str(&space.repeat(2));
        repr.push_str(&format!("{:?}", self.0[1]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[2]));
        repr.push_str(&space.repeat(2));
        repr.push_str(&"\n");

        // third line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[3]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[4]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[5]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // fourth line
        repr.push_str(&format!("{:?}", self.0[6]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[7]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[8]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[9]));
        repr.push_str(&"\n");

        // fifth line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[10]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[11]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[12]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // sixth line
        repr.push_str(&format!("{:?}", self.0[13]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[14]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[15]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[16]));
        repr.push_str(&"\n");

        // seventh line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[17]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[18]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[19]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // eigth line
        repr.push_str(&format!("{:?}", self.0[20]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[21]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[22]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[23]));
        repr.push_str(&"\n");

        // ninth line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[24]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[25]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[26]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // tenth line
        repr.push_str(&format!("{:?}", self.0[27]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[28]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[29]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[30]));
        repr.push_str(&"\n");

        // eleventh line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[31]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[32]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[33]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // twelfth line
        repr.push_str(&space.repeat(2));
        repr.push_str(&format!("{:?}", self.0[34]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("{:?}", self.0[35]));
        repr.push_str(&space.repeat(2));
        repr.push_str(&"\n");

        // thirteenth line
        repr.push_str(&space.repeat(3));
        repr.push_str(&format!("{:?}", self.0[36]));
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
        repr.push_str(&format!(" 0:{:?}", self.0[0]));
        repr.push_str(&space.repeat(3));
        repr.push_str(&"\n");

        // second line
        repr.push_str(&space.repeat(2));
        repr.push_str(&format!(" 1:{:?}", self.0[1]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 2:{:?}", self.0[2]));
        repr.push_str(&space.repeat(2));
        repr.push_str(&"\n");

        // third line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 3:{:?}", self.0[3]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 4:{:?}", self.0[4]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 5:{:?}", self.0[5]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // fourth line
        repr.push_str(&format!(" 6:{:?}", self.0[6]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 7:{:?}", self.0[7]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 8:{:?}", self.0[8]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!(" 9:{:?}", self.0[9]));
        repr.push_str(&"\n");

        // fifth line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("10:{:?}", self.0[10]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("11:{:?}", self.0[11]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("12:{:?}", self.0[12]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // sixth line
        repr.push_str(&format!("13:{:?}", self.0[13]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("14:{:?}", self.0[14]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("15:{:?}", self.0[15]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("16:{:?}", self.0[16]));
        repr.push_str(&"\n");

        // seventh line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("17:{:?}", self.0[17]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("18:{:?}", self.0[18]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("19:{:?}", self.0[19]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // eigth line
        repr.push_str(&format!("20:{:?}", self.0[20]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("21:{:?}", self.0[21]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("22:{:?}", self.0[22]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("23:{:?}", self.0[23]));
        repr.push_str(&"\n");

        // ninth line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("24:{:?}", self.0[24]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("25:{:?}", self.0[25]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("26:{:?}", self.0[26]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // tenth line
        repr.push_str(&format!("27:{:?}", self.0[27]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("28:{:?}", self.0[28]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("29:{:?}", self.0[29]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("30:{:?}", self.0[30]));
        repr.push_str(&"\n");

        // eleventh line
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("31:{:?}", self.0[31]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("32:{:?}", self.0[32]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("33:{:?}", self.0[33]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&"\n");

        // twelfth line
        repr.push_str(&space.repeat(2));
        repr.push_str(&format!("34:{:?}", self.0[34]));
        repr.push_str(&space.repeat(1));
        repr.push_str(&format!("35:{:?}", self.0[35]));
        repr.push_str(&space.repeat(2));
        repr.push_str(&"\n");

        // thirteenth line
        repr.push_str(&space.repeat(3));
        repr.push_str(&format!("36:{:?}", self.0[36]));
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
        Board(pieces)
    }
}
