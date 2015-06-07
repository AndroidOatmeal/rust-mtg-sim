#![feature(plugin)]
#![plugin(clippy)]

pub struct Player;

impl Player {
    pub fn new() -> Player {
        Player
    }

    pub fn play_turn(&self, prev_game_state: GameState) -> GameState {
        let hand = prev_game_state.hand;
        if hand.len() == 2 {
            GameState {
                turn: prev_game_state.turn + 1,
                battlefield: vec![],
                battlefield_tapped: vec![Card::named("Mountain")],
                player2_life_total: prev_game_state.player2_life_total - 3,
                hand: vec![],
            }
        } else {
            GameState {
                turn: prev_game_state.turn + 1,
                battlefield: hand.clone(),
                battlefield_tapped: vec![],
                player2_life_total: prev_game_state.player2_life_total,
                hand: vec![],
            }
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub turn: i32,
    pub battlefield: Vec<Card>,
    pub battlefield_tapped: Vec<Card>,
    pub hand: Vec<Card>,
    pub player2_life_total: i32,
}

impl GameState {
    pub fn new(hand: &Vec<Card>) -> GameState {
        GameState {
            turn: 0,
            battlefield: vec![],
            battlefield_tapped: vec![],
            hand: hand.clone(),
            player2_life_total: 20,
        }
    }
}


pub struct Hand {
    pub card: String,
}

impl Hand {
    pub fn new(card_str: &str) -> Hand {
        Hand { card: card_str.to_owned() }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Card {
    pub name: String,
}

impl Card {
    pub fn named(card_name: &str) -> Card {
        Card { name: card_name.to_owned() }
    }
}
