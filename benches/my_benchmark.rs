use criterion::{black_box, criterion_group, criterion_main, Criterion};
use milestone::{
    self,
    ai::{heuristics::NUM_HEURISTICS, tree::GameTree},
    game::{
        gamestate::{GameBuilder, State},
        player::{PossiblePlayer, AI},
    },
};

fn create_game_env() -> milestone::game::gamestate::State {
    GameBuilder::new()
        .set_player_1(PossiblePlayer::AI(AI::from_name("P1".to_string())))
        .set_player_2(PossiblePlayer::AI(AI::from_name("P2".to_string())))
        .build()
}

fn build_tree(state: &mut State, depth: u8) -> GameTree {
    let mut tree =
        GameTree::new(state.to_owned(), depth, &[1.0; NUM_HEURISTICS]);
    tree.build_eval_tree();
    tree
}

fn build_tree_benchmark(c: &mut Criterion) {
    let state = create_game_env();
    c.bench_function("build tree (depth: 2)", |b| {
        b.iter(|| build_tree(&mut state.clone(), 2))
    });
}

fn evaluate_tree_benchmark(c: &mut Criterion) {
    let mut state = create_game_env();
    let tree = build_tree(&mut state, 2);

    c.bench_function("evaluate tree (depth: 2)", |b| {
        b.iter(|| black_box(tree.clone()).rollback(0))
    });
}

criterion_group!(benches, build_tree_benchmark, evaluate_tree_benchmark);
criterion_main!(benches);
