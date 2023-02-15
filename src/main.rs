mod game;

fn main() {
    let game = game::new_game();
    println!("{:?}", game);
}
