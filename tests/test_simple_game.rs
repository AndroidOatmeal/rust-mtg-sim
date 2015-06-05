#![feature(plugin)]
#![plugin(clippy)]

extern crate mtgsim;

use mtgsim::{GameState, Player};

#[test]
fn test_player_pass_turn() {
    let player = Player::new();
    let simple_game_state = GameState::new(&vec![]);
    let new_game_state = player.play_turn(simple_game_state);
    assert_eq!(1, new_game_state.turn);
}

#[test]
fn test_play_land() {
    let player = Player::new();
    let state = GameState::new(&vec!["Mountain"]);
    let new_state = player.play_turn(state);

    assert!(new_state.battlefield.contains(&"Mountain".to_owned()));
    assert_eq!(1, new_state.turn);
}
