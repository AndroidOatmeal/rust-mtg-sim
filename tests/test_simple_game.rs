extern crate mtgsim;

use mtgsim::{GameState, Hand, Player};

#[test]
fn test_player_pass_turn() {
    let player = Player::new();
    let simple_game_state = GameState::new();
    let new_game_state = player.play_turn(simple_game_state);
    assert_eq!(1, new_game_state.turn);
}

#[test]
fn test_play_land() {
    let player = Player::new();
    let hand_with_land = Hand::new("Mountain");
    let state = GameState::new_with_hand(hand_with_land);
    let new_state = player.play_turn(state);

    assert!(new_state.battlefield.contains(&"Mountain".to_string()));
    assert_eq!(1, new_state.turn);
}
