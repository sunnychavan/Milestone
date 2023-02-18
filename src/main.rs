mod ai;
mod cli;
mod game;

fn main() {
    // cli::two_player_game();

    let game = game::new_game();
    println!("This ran");
    ai::tree::GameNode::create_new_tree(&game, 0);
}
