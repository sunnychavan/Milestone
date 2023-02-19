mod ai;
mod cli;
mod game;

fn main() {
    // cli::two_player_game();

    let game = game::new_game();
    ai::tree::create_new_tree(&game, 2);
}
