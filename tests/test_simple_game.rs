extern crate mtgsim;

use mtgsim::{GameState, Player};

#[test]
fn test_simple_game() {
    let player = Player::new();
    let simple_game_state = GameState::new();
    let new_game_state = player.play_turn(simple_game_state);
    assert_eq!(new_game_state.turn, 1);
}
