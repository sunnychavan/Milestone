use crate::game::gamestate::State;

pub fn number_of_pieces(state: &State) -> u64 {
    state
        .board
        .current_players_pieces(state.current_turn)
        .len()
        .try_into()
        .unwrap_or(0)
}
