mod cli;
mod game;

fn main() {
    cli::two_player_game();

    // let mut game = game::new_game();
    // println!("{:?}", game);

    // game.move_piece(7, 14, false).unwrap();
    // println!("{:?}", game);

    // game.move_piece(29, 22, false).unwrap();
    // println!("{:?}", game);
    // game.move_piece(6, 10, false).unwrap();
    // println!("{:?}", game);
    // game.move_piece(32, 25, false).unwrap();
    // println!("{:?}", game);
    // game.move_piece(14, 21, false).unwrap();
    // println!("{:?}", game);
    // game.move_piece(28, 21, true).unwrap();
    // println!("{:?}", game);
}
