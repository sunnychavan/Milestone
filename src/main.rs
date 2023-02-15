mod game;

fn main() {
    let board = game::new_game();
    println!("{:?}", board);
}
