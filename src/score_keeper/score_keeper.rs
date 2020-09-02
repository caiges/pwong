use std::collections::HashMap;

pub struct ScoreKeeper {
  players: HashMap<String, PlayerScore>,
}

pub struct PlayerScore {
  score: u32,
}
