use crate::game::gamestate::State;
use crate::game::pieces::Piece;
use crate::game::player::Person;
use crate::game::player::PossiblePlayer;

use super::tree::GameNode;

#[test]
fn test_rollback() {
    let players = [
        PossiblePlayer::Person(Person::new("Corban".to_string(), Piece::Black)),
        PossiblePlayer::Person(Person::new("Connor".to_string(), Piece::Black)),
    ];

    let node_2 = GameNode::new(
        None,
        State::new(&players.clone()),
        Some(2),
        None,
        Some(3),
    );
    let node_3 = GameNode::new(
        None,
        State::new(&players.clone()),
        Some(2),
        None,
        Some(5),
    );
    let node_1 = GameNode::new(
        Some(vec![node_2, node_3]),
        State::new(&players.clone()),
        Some(1),
        None,
        None,
    );

    let node_5 = GameNode::new(
        None,
        State::new(&players.clone()),
        Some(2),
        None,
        Some(2),
    );
    let node_6 = GameNode::new(
        None,
        State::new(&players.clone()),
        Some(2),
        None,
        Some(9),
    );
    let node_4 = GameNode::new(
        Some(vec![node_5, node_6]),
        State::new(&players.clone()),
        Some(1),
        None,
        None,
    );

    let mut node_0 = GameNode::new(
        Some(vec![node_1, node_4]),
        State::new(&players.clone()),
        Some(0),
        None,
        None,
    );

    assert_eq!(node_0.rollback(), 3);
    assert_eq!(node_0.total_subnodes, 6)
}
