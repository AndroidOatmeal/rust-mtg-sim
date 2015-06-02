pub struct Player;

impl Player {
    pub fn new() -> Player {
        Player
    }

    pub fn play_turn(&self, prev_game_state: GameState) -> GameState {
        GameState { turn: prev_game_state.turn + 1 }
    }
}


pub struct GameState {
    pub turn: i32,
}

impl GameState {
    pub fn new() -> GameState {
        GameState { turn: 0 }
    }
}
