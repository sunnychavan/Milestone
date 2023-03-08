use lazy_static::lazy_static;

pub fn location_agnostic(location: &usize) -> i64 {
    *location as i64
}

pub fn centrality(location: &usize) -> i64 {
    lazy_static! {
        static ref CENTRALITY: [i64; 37] = {
            let mut centrality_map = [0; 37];

            centrality_map[4] = 1;
            centrality_map[7] = 1;
            centrality_map[8] = 1;
            centrality_map[10] = 1;
            centrality_map[12] = 1;
            centrality_map[17] = 1;
            centrality_map[19] = 1;
            centrality_map[24] = 1;
            centrality_map[26] = 1;
            centrality_map[28] = 1;
            centrality_map[29] = 1;
            centrality_map[32] = 1;

            centrality_map[11] = 2;
            centrality_map[14] = 2;
            centrality_map[15] = 2;
            centrality_map[21] = 2;
            centrality_map[22] = 2;
            centrality_map[25] = 2;

            centrality_map[18] = 3;

            centrality_map
        };
    }

    CENTRALITY[*location]
}

pub fn anti_centrality(location: &usize) -> i64 {
    3 - centrality(location)
}

pub fn middle_proximity(location: &usize) -> i64 {
    lazy_static! {
        static ref MIDDLE_PROXIMITY: [i64; 37] = {
            let mut middle_proximity: [i64; 37] = [0; 37];
            middle_proximity[0] = 6;
            middle_proximity[4] = 6;
            middle_proximity[11] = 6;
            middle_proximity[18] = 6;
            middle_proximity[25] = 6;
            middle_proximity[32] = 6;
            middle_proximity[36] = 6;

            middle_proximity[1] = 3;
            middle_proximity[2] = 3;
            middle_proximity[7] = 3;
            middle_proximity[8] = 3;
            middle_proximity[14] = 3;
            middle_proximity[15] = 3;
            middle_proximity[21] = 3;
            middle_proximity[22] = 3;
            middle_proximity[28] = 3;
            middle_proximity[29] = 3;
            middle_proximity[34] = 3;
            middle_proximity[35] = 3;

            middle_proximity[3] = 1;
            middle_proximity[5] = 1;
            middle_proximity[10] = 1;
            middle_proximity[12] = 1;
            middle_proximity[17] = 1;
            middle_proximity[19] = 1;
            middle_proximity[24] = 1;
            middle_proximity[26] = 1;
            middle_proximity[31] = 1;
            middle_proximity[33] = 1;

            middle_proximity
        };
    }

    MIDDLE_PROXIMITY[*location]
}

pub fn black_proximity(location: &usize) -> i64 {
    lazy_static! {
        static ref BLACK_PROXIMITY: [i64; 37] = {
            let mut black_proximity = [0; 37];

            black_proximity[0] = 5;

            black_proximity[1] = 4;
            black_proximity[2] = 4;
            black_proximity[4] = 4;

            black_proximity[3] = 3;
            black_proximity[5] = 3;
            black_proximity[7] = 3;
            black_proximity[8] = 3;
            black_proximity[11] = 3;

            black_proximity[6] = 2;
            black_proximity[9] = 2;
            black_proximity[10] = 2;
            black_proximity[12] = 2;
            black_proximity[14] = 2;
            black_proximity[15] = 2;
            black_proximity[18] = 2;

            black_proximity[13] = 1;
            black_proximity[16] = 1;
            black_proximity[17] = 1;
            black_proximity[19] = 1;
            black_proximity[21] = 1;
            black_proximity[22] = 1;
            black_proximity[25] = 1;

            black_proximity
        };
    }

    BLACK_PROXIMITY[*location]
}

pub fn white_proximity(location: &usize) -> i64 {
    lazy_static! {
        static ref WHITE_PROXIMITY: [i64; 37] = {
            let mut white_proximity = [0; 37];

            white_proximity[36] = 5;

            white_proximity[32] = 4;
            white_proximity[34] = 4;
            white_proximity[35] = 4;

            white_proximity[25] = 3;
            white_proximity[28] = 3;
            white_proximity[29] = 3;
            white_proximity[31] = 3;
            white_proximity[33] = 3;

            white_proximity[18] = 2;
            white_proximity[21] = 2;
            white_proximity[22] = 2;
            white_proximity[24] = 2;
            white_proximity[26] = 2;
            white_proximity[27] = 2;
            white_proximity[30] = 2;

            white_proximity[11] = 1;
            white_proximity[14] = 1;
            white_proximity[15] = 1;
            white_proximity[17] = 1;
            white_proximity[19] = 1;
            white_proximity[20] = 1;
            white_proximity[23] = 1;

            white_proximity
        };
    }

    WHITE_PROXIMITY[*location]
}

pub fn black_proximity_row(location: usize) -> i64 {
    lazy_static! {
        static ref BLACK_PROXIMITY: [i64; 37] = {
            let mut black_proximity = [0; 37];

            black_proximity[1] = 1;
            black_proximity[2] = 1;

            black_proximity[3] = 2;
            black_proximity[4] = 2;
            black_proximity[5] = 2;

            black_proximity[6] = 3;
            black_proximity[7] = 3;
            black_proximity[8] = 3;
            black_proximity[9] = 3;

            black_proximity[10] = 4;
            black_proximity[11] = 4;
            black_proximity[12] = 4;

            black_proximity[13] = 5;
            black_proximity[14] = 5;
            black_proximity[15] = 5;
            black_proximity[16] = 5;

            black_proximity[17] = 6;
            black_proximity[18] = 6;
            black_proximity[19] = 6;

            black_proximity[20] = 7;
            black_proximity[21] = 7;
            black_proximity[22] = 7;
            black_proximity[23] = 7;

            black_proximity[24] = 8;
            black_proximity[25] = 8;
            black_proximity[26] = 8;

            black_proximity[27] = 9;
            black_proximity[28] = 9;
            black_proximity[29] = 9;
            black_proximity[30] = 9;

            black_proximity[31] = 10;
            black_proximity[32] = 10;
            black_proximity[33] = 10;

            black_proximity[34] = 11;
            black_proximity[35] = 11;

            black_proximity[36] = 12;

            black_proximity
        };
    }

    BLACK_PROXIMITY[location]
}

pub fn white_proximity_row(location: usize) -> i64 {
    12 - black_proximity_row(location)
}
