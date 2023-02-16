mod game;

fn main() {
    let mut game = game::new_game();
    println!("{:?}", game);

    game.move_piece(7, 14, false);
    println!("{:?}", game);

    game.move_piece(14, 18, false);
    println!("{:?}", game);
    game.move_piece(18, 25, false);
    println!("{:?}", game);
    game.move_piece(25, 28, true);
    println!("{:?}", game);
}
