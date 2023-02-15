mod game;

fn main() {
    let board = game::gamestate::initialize_board();
    println!("{:?}", board);
}
