#![feature(plugin)]
#![plugin(clippy)]

extern crate mtgsim;


mod test_harness {
    use mtgsim::{Card, GameState, Player};

    fn play_turn_with_hand(hand: Vec<&str>) -> GameState {
        let player = Player::new();
        let cards_in_hand: Vec<Card> = hand.iter().map(|n| Card::named(n)).collect();
        let simple_game_state = GameState::new(&cards_in_hand);
        player.play_turn(simple_game_state)
    }

    #[test]
    fn test_player_pass_turn() {
        let game_state = play_turn_with_hand(vec![]);
        let expected_state = GameState {
            turn: 1,
            battlefield: vec![],
            battlefield_tapped: vec![],
            player2_life_total: 20,
            hand: vec![],
        };
        assert_eq!(expected_state, game_state);
    }

    #[test]
    fn test_play_land() {
        let game_state = play_turn_with_hand(vec!["Mountain"]);
        let expected_state = GameState {
            turn: 1,
            battlefield: vec![Card::named("Mountain")],
            battlefield_tapped: vec![],
            player2_life_total: 20,
            hand: vec![],
        };
        assert_eq!(expected_state, game_state);
    }

    #[test]
    fn test_play_bolt() {
        let game_state = play_turn_with_hand(vec!["Mountain", "Lightning Bolt"]);
        let expected_state = GameState {
            turn: 1,
            battlefield: vec![],
            battlefield_tapped: vec![Card::named("Mountain")],
            player2_life_total: 17,
            hand: vec![],
        };
        assert_eq!(expected_state, game_state);
    }

}
