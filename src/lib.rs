pub struct Player;

impl Player {
    pub fn new() -> Player {
        Player
    }

    pub fn play_turn(&self, prev_game_state: GameState) -> GameState {
        GameState {
            turn: prev_game_state.turn + 1,
            battlefield: prev_game_state.battlefield,
        }
    }
}


pub struct GameState {
    pub turn: i32,
    pub battlefield: Vec<String>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            turn: 0,
            battlefield: Vec::new(),
        }
    }

    pub fn new_with_hand(h: Hand) -> GameState {
        GameState {
            turn: 0,
            battlefield: vec![h.card],
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
