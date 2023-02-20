use crate::game::gamestate::State;

pub fn number_of_pieces(state: &State) -> i64 {
    state
        .board
        .current_players_pieces(state.current_turn)
        .len()
        .try_into()
        .unwrap_or(0)
}

pub fn piece_differential(state: &State) -> i64 {
    let current_player_num = state
        .board
        .current_players_pieces(state.current_turn)
        .len()
        .try_into()
        .unwrap_or(0);
    let opponent_num = state
        .board
        .current_players_pieces(1 - state.current_turn)
        .len()
        .try_into()
        .unwrap_or(0);
    return current_player_num - opponent_num;
}
