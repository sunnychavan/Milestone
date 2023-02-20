mod ai;
mod cli;
mod game;

fn main() {
    cli::two_player_game();

    // let game = game::new_game();
    // let mut tree = ai::tree::create_eval_tree(&game, 4);

    // tree.rollback();
    // print!("{:?}", tree);
}
