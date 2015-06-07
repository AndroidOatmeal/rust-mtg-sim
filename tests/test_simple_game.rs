#![feature(plugin)]
#![plugin(clippy)]

extern crate mtgsim;


mod test_harness {
    use mtgsim::{GameState, Player};

    fn setup_gamestate_with_hand(hand: Vec<&str>) -> GameState {
        let player = Player::new();
        let simple_game_state = GameState::new(&hand);
        player.play_turn(simple_game_state)
    }

    #[test]
    fn test_player_pass_turn() {
        let new_game_state = setup_gamestate_with_hand(vec![]);
        assert_eq!(1, new_game_state.turn);
    }

    #[test]
    fn test_play_land() {
        let new_game_state = setup_gamestate_with_hand(vec!["Mountain"]);
        assert!(new_game_state.battlefield.contains(&"Mountain".to_owned()));
        assert_eq!(1, new_game_state.turn);
    }

    #[test]
    fn test_play_bolt() {
        let new_game_state = setup_gamestate_with_hand(vec!["Mountain", "Lightning Bolt"]);
        assert!(new_game_state.battlefield_tapped.contains(&"Mountain".to_owned()));
        assert_eq!(17, new_game_state.player2_life_total);
        assert_eq!(1, new_game_state.turn);
    }
}
