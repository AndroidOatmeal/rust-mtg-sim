#![feature(plugin)]
#![plugin(clippy)]

pub struct Player;

impl Player {
    pub fn new() -> Player {
        Player
    }

    pub fn play_turn(&self, prev_game_state: GameState) -> GameState {
        let hand = prev_game_state.hand;
        GameState {
            turn: prev_game_state.turn + 1,
            battlefield: hand.clone(),
            battlefield_tapped: hand.clone(),
            player2_life_total: prev_game_state.player2_life_total,
            hand: vec![],
        }
    }
}


pub struct GameState {
    pub turn: i32,
    pub battlefield: Vec<String>,
    pub battlefield_tapped: Vec<String>,
    pub hand: Vec<String>,
    pub player2_life_total: i32,
}

impl GameState {
    pub fn new(hand: &Vec<&str>) -> GameState {
        GameState {
            turn: 0,
            battlefield: vec![],
            battlefield_tapped: vec![],
            hand: hand.clone().iter()
                .map(|n| n.to_string())
                .collect(),
            player2_life_total: 17,
        }
    }
}


pub struct Hand {
    pub card: String,
}

impl Hand {
    pub fn new(card_str: &str) -> Hand {
        Hand { card: card_str.to_string() }
    }
}
